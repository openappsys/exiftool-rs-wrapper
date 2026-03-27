//! 连接池支持模块
//!
//! 用于高并发场景下的性能优化

use crate::ExifTool;
use crate::error::{Error, Result};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// ExifTool 连接池
#[derive(Debug)]
pub struct ExifToolPool {
    /// 连接池
    connections: Arc<Mutex<VecDeque<ExifTool>>>,
    /// 池大小
    size: usize,
}

impl ExifToolPool {
    /// 创建新的连接池
    pub fn new(size: usize) -> Result<Self> {
        if size == 0 {
            return Err(Error::invalid_arg("Pool size must be greater than 0"));
        }

        let mut connections = VecDeque::with_capacity(size);

        for _ in 0..size {
            let exiftool = ExifTool::new()?;
            connections.push_back(exiftool);
        }

        Ok(Self {
            connections: Arc::new(Mutex::new(connections)),
            size,
        })
    }

    /// 获取池大小
    pub fn size(&self) -> usize {
        self.size
    }

    /// 获取可用连接数
    pub fn available(&self) -> Result<usize> {
        let connections = self.connections.lock().map_err(|_| Error::MutexPoisoned)?;
        Ok(connections.len())
    }

    /// 获取连接
    pub fn acquire(&self) -> Result<PoolConnection> {
        let mut connections = self.connections.lock().map_err(|_| Error::MutexPoisoned)?;

        if let Some(exiftool) = connections.pop_front() {
            Ok(PoolConnection {
                exiftool: Some(exiftool),
                pool: Arc::clone(&self.connections),
            })
        } else {
            Err(Error::process("No available connections in pool"))
        }
    }

    /// 在超时时间内等待获取连接
    pub fn acquire_timeout(&self, timeout: Duration) -> Result<PoolConnection> {
        let start = Instant::now();

        loop {
            if let Some(conn) = self.try_acquire() {
                return Ok(conn);
            }

            if start.elapsed() >= timeout {
                return Err(Error::Timeout);
            }

            std::thread::sleep(Duration::from_millis(5));
        }
    }

    /// 尝试获取连接（非阻塞）
    pub fn try_acquire(&self) -> Option<PoolConnection> {
        self.acquire().ok()
    }

    /// 关闭所有连接
    pub fn close(&self) -> Result<()> {
        let mut connections = self.connections.lock().map_err(|_| Error::MutexPoisoned)?;

        while let Some(exiftool) = connections.pop_front() {
            let _ = exiftool.close();
        }

        Ok(())
    }
}

impl Clone for ExifToolPool {
    fn clone(&self) -> Self {
        Self {
            connections: Arc::clone(&self.connections),
            size: self.size,
        }
    }
}

/// 池连接包装器
///
/// 当此对象被丢弃时，连接会自动归还到池中
pub struct PoolConnection {
    exiftool: Option<ExifTool>,
    pool: Arc<Mutex<VecDeque<ExifTool>>>,
}

impl PoolConnection {
    /// 获取内部 ExifTool 的引用
    pub fn get(&self) -> Option<&ExifTool> {
        self.exiftool.as_ref()
    }

    /// 获取内部 ExifTool 的可变引用
    pub fn get_mut(&mut self) -> Option<&mut ExifTool> {
        self.exiftool.as_mut()
    }
}

impl Drop for PoolConnection {
    fn drop(&mut self) {
        if let Some(exiftool) = self.exiftool.take()
            && let Ok(mut pool) = self.pool.lock()
        {
            pool.push_back(exiftool);
        }
        // 如果锁被污染，连接会被丢弃
    }
}

/// 使用连接池的辅助函数
///
/// 获取连接，执行操作，自动归还连接
pub fn with_pool<F, R>(pool: &ExifToolPool, f: F) -> Result<R>
where
    F: FnOnce(&mut ExifTool) -> Result<R>,
{
    let mut conn = pool.acquire()?;
    let exiftool = conn
        .get_mut()
        .ok_or_else(|| Error::process("Failed to get connection"))?;
    f(exiftool)
}

/// 批量处理使用连接池
pub fn batch_with_pool<P, F, R>(pool: &ExifToolPool, items: Vec<P>, processor: F) -> Vec<Result<R>>
where
    P: Send + 'static,
    F: Fn(&mut ExifTool, P) -> Result<R> + Send + Sync + 'static,
    R: Send + 'static,
{
    use std::thread;

    let total = items.len();
    if total == 0 {
        return Vec::new();
    }

    let processor = Arc::new(processor);
    let pool = pool.clone();
    let workers = pool.size().min(total);
    let queue: Arc<Mutex<VecDeque<(usize, P)>>> =
        Arc::new(Mutex::new(items.into_iter().enumerate().collect()));
    let results: Arc<Mutex<Vec<Option<Result<R>>>>> = Arc::new(Mutex::new(
        std::iter::repeat_with(|| None).take(total).collect(),
    ));
    let mut handles = Vec::with_capacity(workers);

    for _ in 0..workers {
        let pool = pool.clone();
        let processor = Arc::clone(&processor);
        let queue = Arc::clone(&queue);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            loop {
                let next = {
                    let mut queue = match queue.lock() {
                        Ok(q) => q,
                        Err(_) => return,
                    };
                    queue.pop_front()
                };

                let Some((index, item)) = next else {
                    return;
                };

                let result = match pool.acquire_timeout(Duration::from_secs(30)) {
                    Ok(mut conn) => match conn.get_mut() {
                        Some(exiftool) => processor(exiftool, item),
                        None => Err(Error::process("Failed to get connection")),
                    },
                    Err(e) => Err(e),
                };

                if let Ok(mut all_results) = results.lock() {
                    all_results[index] = Some(result);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if handle.join().is_err() {
            return vec![Err(Error::process("Thread panicked"))];
        }
    }

    let mut out = Vec::with_capacity(total);
    if let Ok(mut locked) = results.lock() {
        for item in locked.drain(..) {
            out.push(item.unwrap_or_else(|| Err(Error::process("Missing batch result"))));
        }
        return out;
    }

    vec![Err(Error::MutexPoisoned)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_creation() {
        match ExifToolPool::new(2) {
            Ok(pool) => {
                assert_eq!(pool.size(), 2);
                assert_eq!(pool.available().unwrap(), 2);
            }
            Err(Error::ExifToolNotFound) => {
                println!("⚠ ExifTool not found, skipping test");
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_pool_acquire() {
        match ExifToolPool::new(2) {
            Ok(pool) => {
                // 获取两个连接
                let _conn1 = pool.acquire().unwrap();
                let _conn2 = pool.acquire().unwrap();

                // 池应该空了
                assert_eq!(pool.available().unwrap(), 0);
            }
            Err(Error::ExifToolNotFound) => {
                println!("⚠ ExifTool not found, skipping test");
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_pool_connection_return() {
        match ExifToolPool::new(1) {
            Ok(pool) => {
                {
                    let _conn = pool.acquire().unwrap();
                    assert_eq!(pool.available().unwrap(), 0);
                }
                // 连接应该已归还
                assert_eq!(pool.available().unwrap(), 1);
            }
            Err(Error::ExifToolNotFound) => {
                println!("⚠ ExifTool not found, skipping test");
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
