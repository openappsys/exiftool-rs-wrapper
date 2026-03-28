//! 流式处理和性能优化模块
//!
//! 支持大文件流式处理、进度回调、内存池优化

use crate::ExifTool;
use crate::error::Result;
use std::fmt;
use std::io::{self, Read};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// 进度回调函数类型
pub type ProgressCallback = Arc<dyn Fn(usize, usize) -> bool + Send + Sync>;

/// 流式处理选项
#[derive(Clone)]
pub struct StreamOptions {
    /// 缓冲区大小（字节）
    pub buffer_size: usize,
    /// 进度回调
    pub progress_callback: Option<ProgressCallback>,
}

impl fmt::Debug for StreamOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamOptions")
            .field("buffer_size", &self.buffer_size)
            .field("has_callback", &self.progress_callback.is_some())
            .finish()
    }
}

impl Default for StreamOptions {
    fn default() -> Self {
        Self {
            buffer_size: 64 * 1024, // 64KB 默认缓冲区
            progress_callback: None,
        }
    }
}

impl StreamOptions {
    /// 创建新的流式处理选项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置缓冲区大小
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// 设置进度回调
    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, usize) -> bool + Send + Sync + 'static,
    {
        self.progress_callback = Some(Arc::new(callback));
        self
    }
}

/// 进度追踪器
pub struct ProgressTracker {
    /// 总字节数
    total: AtomicU64,
    /// 已处理字节数
    processed: AtomicU64,
    /// 回调函数
    callback: Option<ProgressCallback>,
    /// 是否取消
    cancelled: std::sync::atomic::AtomicBool,
}

impl fmt::Debug for ProgressTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProgressTracker")
            .field("total", &self.total.load(Ordering::SeqCst))
            .field("processed", &self.processed.load(Ordering::SeqCst))
            .field("has_callback", &self.callback.is_some())
            .field("cancelled", &self.cancelled.load(Ordering::SeqCst))
            .finish()
    }
}

impl ProgressTracker {
    /// 创建新的进度追踪器
    pub fn new(total: usize, callback: Option<ProgressCallback>) -> Self {
        Self {
            total: AtomicU64::new(total as u64),
            processed: AtomicU64::new(0),
            callback,
            cancelled: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// 更新进度
    pub fn update(&self, bytes: usize) {
        let processed = self.processed.fetch_add(bytes as u64, Ordering::SeqCst) + bytes as u64;
        let total = self.total.load(Ordering::SeqCst);

        if let Some(ref callback) = self.callback
            && !callback(processed as usize, total as usize)
        {
            self.cancelled.store(true, Ordering::SeqCst);
        }
    }

    /// 检查是否已取消
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// 获取进度百分比
    pub fn percentage(&self) -> f64 {
        let processed = self.processed.load(Ordering::SeqCst);
        let total = self.total.load(Ordering::SeqCst);

        if total == 0 {
            0.0
        } else {
            (processed as f64 / total as f64) * 100.0
        }
    }

    /// 获取已处理字节数
    pub fn processed(&self) -> u64 {
        self.processed.load(Ordering::SeqCst)
    }

    /// 获取总字节数
    pub fn total(&self) -> u64 {
        self.total.load(Ordering::SeqCst)
    }
}

/// 缓冲读取器（支持进度追踪）
pub struct ProgressReader<R: Read> {
    inner: R,
    tracker: Arc<ProgressTracker>,
}

impl<R: Read> ProgressReader<R> {
    /// 创建新的进度读取器
    pub fn new(inner: R, tracker: Arc<ProgressTracker>, _buffer_size: usize) -> Self {
        Self { inner, tracker }
    }

    /// 检查是否已取消
    pub fn is_cancelled(&self) -> bool {
        self.tracker.is_cancelled()
    }
}

impl<R: Read> Read for ProgressReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Operation cancelled",
            ));
        }

        let n = self.inner.read(buf)?;
        self.tracker.update(n);
        Ok(n)
    }
}

/// 流式处理 trait
pub trait StreamingOperations {
    /// 流式处理大文件
    fn process_streaming<P, F, R>(
        &self,
        path: P,
        options: &StreamOptions,
        processor: F,
    ) -> Result<R>
    where
        P: AsRef<Path>,
        F: FnMut(&mut dyn Read) -> Result<R>;

    /// 批量处理带进度回调
    fn process_batch_with_progress<P, F>(
        &self,
        paths: &[P],
        options: &StreamOptions,
        processor: F,
    ) -> Vec<Result<()>>
    where
        P: AsRef<Path>,
        F: FnMut(&ExifTool, &Path, &ProgressTracker) -> Result<()>;
}

impl StreamingOperations for ExifTool {
    fn process_streaming<P, F, R>(
        &self,
        path: P,
        options: &StreamOptions,
        mut processor: F,
    ) -> Result<R>
    where
        P: AsRef<Path>,
        F: FnMut(&mut dyn Read) -> Result<R>,
    {
        // 使用标准文件读取实现流式处理
        let file = std::fs::File::open(path.as_ref()).map_err(crate::error::Error::Io)?;

        let tracker = Arc::new(ProgressTracker::new(1, options.progress_callback.clone()));

        let mut reader = ProgressReader::new(file, tracker, options.buffer_size);

        processor(&mut reader)
    }

    fn process_batch_with_progress<P, F>(
        &self,
        paths: &[P],
        options: &StreamOptions,
        processor: F,
    ) -> Vec<Result<()>>
    where
        P: AsRef<Path>,
        F: FnMut(&ExifTool, &Path, &ProgressTracker) -> Result<()>,
    {
        let total = paths.len();
        let tracker = Arc::new(ProgressTracker::new(
            total,
            options.progress_callback.clone(),
        ));

        let mut results = Vec::with_capacity(total);
        let mut processor = processor;

        for path in paths {
            let result = processor(self, path.as_ref(), &tracker);
            tracker.update(1);
            results.push(result);
        }

        results
    }
}

/// 缓存管理器
pub struct Cache<K, V> {
    /// 内部缓存
    inner: std::sync::Mutex<lru::LruCache<K, V>>,
    /// 命中率统计
    hits: AtomicU64,
    /// 未命中统计
    misses: AtomicU64,
}

impl<K, V> fmt::Debug for Cache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cache")
            .field("hits", &self.hits.load(Ordering::SeqCst))
            .field("misses", &self.misses.load(Ordering::SeqCst))
            .finish()
    }
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Cache<K, V> {
    /// 创建新的缓存
    pub fn new(capacity: usize) -> Self {
        use std::num::NonZeroUsize;
        let capacity = NonZeroUsize::new(capacity.max(1)).unwrap();
        Self {
            inner: std::sync::Mutex::new(lru::LruCache::new(capacity)),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    /// 获取值
    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.inner.lock().ok()?;

        if let Some(value) = cache.get(key) {
            self.hits.fetch_add(1, Ordering::SeqCst);
            Some(value.clone())
        } else {
            self.misses.fetch_add(1, Ordering::SeqCst);
            None
        }
    }

    /// 插入值
    pub fn put(&self, key: K, value: V) {
        if let Ok(mut cache) = self.inner.lock() {
            cache.put(key, value);
        }
    }

    /// 获取命中率
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::SeqCst);
        let misses = self.misses.load(Ordering::SeqCst);
        let total = hits + misses;

        if total == 0 {
            0.0
        } else {
            (hits as f64 / total as f64) * 100.0
        }
    }

    /// 清空缓存
    pub fn clear(&self) {
        if let Ok(mut cache) = self.inner.lock() {
            cache.clear();
        }
        self.hits.store(0, Ordering::SeqCst);
        self.misses.store(0, Ordering::SeqCst);
    }
}

/// 性能统计
#[derive(Debug, Default)]
pub struct PerformanceStats {
    /// 总操作数
    pub total_operations: AtomicU64,
    /// 成功操作数
    pub successful_operations: AtomicU64,
    /// 失败操作数
    pub failed_operations: AtomicU64,
    /// 总耗时（微秒）
    pub total_time_us: AtomicU64,
}

impl PerformanceStats {
    /// 记录操作
    pub fn record(&self, success: bool, elapsed_us: u64) {
        self.total_operations.fetch_add(1, Ordering::SeqCst);
        self.total_time_us.fetch_add(elapsed_us, Ordering::SeqCst);

        if success {
            self.successful_operations.fetch_add(1, Ordering::SeqCst);
        } else {
            self.failed_operations.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// 获取平均耗时（微秒）
    pub fn avg_time_us(&self) -> u64 {
        let total = self.total_operations.load(Ordering::SeqCst);
        let time = self.total_time_us.load(Ordering::SeqCst);

        if total == 0 { 0 } else { time / total }
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        let total = self.total_operations.load(Ordering::SeqCst);
        let success = self.successful_operations.load(Ordering::SeqCst);

        if total == 0 {
            0.0
        } else {
            (success as f64 / total as f64) * 100.0
        }
    }
}

// ============================================================================
// 异步流支持（需要 async feature）
// ============================================================================

#[cfg(feature = "async")]
pub mod async_stream {
    //! 异步流式处理支持模块
    //!
    //! 提供基于流的异步元数据处理，支持进度跟踪和取消操作。

    use crate::error::{Error, Result};
    use crate::types::Metadata;
    use std::path::{Path, PathBuf};
    use tokio::sync::mpsc;
    use tokio::sync::watch;

    /// 流事件类型
    #[derive(Debug, Clone)]
    pub enum StreamEvent {
        /// 进度更新（已处理字节数，总字节数）
        Progress(usize, usize),
        /// 元数据块（用于流式解析）
        MetadataChunk(Metadata),
        /// 处理完成
        Complete,
        /// 处理取消
        Cancelled,
    }

    /// 异步流选项
    #[derive(Debug, Clone)]
    pub struct AsyncStreamOptions {
        /// 缓冲区大小（字节）
        pub buffer_size: usize,
        /// 是否启用进度报告
        pub enable_progress: bool,
        /// 进度报告间隔（毫秒）
        pub progress_interval_ms: u64,
    }

    impl Default for AsyncStreamOptions {
        fn default() -> Self {
            Self {
                buffer_size: 64 * 1024, // 64KB
                enable_progress: true,
                progress_interval_ms: 100,
            }
        }
    }

    impl AsyncStreamOptions {
        /// 创建新的异步流选项
        pub fn new() -> Self {
            Self::default()
        }

        /// 设置缓冲区大小
        pub fn buffer_size(mut self, size: usize) -> Self {
            self.buffer_size = size;
            self
        }

        /// 设置是否启用进度报告
        pub fn enable_progress(mut self, enable: bool) -> Self {
            self.enable_progress = enable;
            self
        }

        /// 设置进度报告间隔
        pub fn progress_interval_ms(mut self, interval: u64) -> Self {
            self.progress_interval_ms = interval;
            self
        }
    }

    /// 异步流处理句柄
    ///
    /// 用于控制异步流处理的执行，支持取消操作。
    #[derive(Debug, Clone)]
    pub struct AsyncStreamHandle {
        cancel_tx: watch::Sender<bool>,
    }

    impl AsyncStreamHandle {
        /// 创建新的流处理句柄
        pub fn new() -> (Self, watch::Receiver<bool>) {
            let (cancel_tx, cancel_rx) = watch::channel(false);
            (Self { cancel_tx }, cancel_rx)
        }

        /// 取消流处理
        pub fn cancel(&self) -> Result<()> {
            self.cancel_tx
                .send(true)
                .map_err(|e| Error::process(format!("Failed to send cancel signal: {}", e)))
        }

        /// 检查是否已请求取消
        pub fn is_cancelled(&self) -> bool {
            *self.cancel_tx.borrow()
        }
    }

    impl Default for AsyncStreamHandle {
        fn default() -> Self {
            let (handle, _) = Self::new();
            handle
        }
    }

    /// 异步批量处理结果
    #[derive(Debug, Clone)]
    pub struct AsyncBatchResult {
        /// 成功处理的文件数
        pub success_count: usize,
        /// 失败的文件数
        pub failure_count: usize,
        /// 总文件数
        pub total_count: usize,
    }

    /// 异步流处理 trait
    ///
    /// 为 ExifTool 提供异步流式处理能力。
    #[async_trait::async_trait]
    pub trait AsyncStreamingOperations {
        /// 异步流式查询文件元数据
        ///
        /// 返回一个接收流事件的通道，可用于实时跟踪处理进度。
        ///
        /// # 示例
        ///
        /// ```rust,ignore
        /// use exiftool_rs_wrapper::AsyncExifTool;
        /// use exiftool_rs_wrapper::stream::async_stream::AsyncStreamingOperations;
        ///
        /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
        ///     let exiftool = AsyncExifTool::new()?;
        ///
        ///     let (mut rx, _handle) = exiftool.stream_query("photo.jpg").await?;
        ///
        ///     while let Some(event) = rx.recv().await {
        ///         match event {
        ///             StreamEvent::Progress(processed, total) => {
        ///                 println!("Progress: {}/{} bytes", processed, total);
        ///             }
        ///             StreamEvent::Complete => {
        ///                 println!("Processing complete");
        ///                 break;
        ///             }
        ///             _ => {}
        ///         }
        ///     }
        ///
        ///     Ok(())
        /// }
        /// ```
        async fn stream_query<P: AsRef<Path> + Send>(
            &self,
            path: P,
        ) -> Result<(mpsc::Receiver<StreamEvent>, AsyncStreamHandle)>;

        /// 异步批量处理多个文件
        ///
        /// 并发处理多个文件，通过流返回进度和结果。
        async fn stream_batch<P: AsRef<Path> + Send>(
            &self,
            paths: &[P],
            options: &AsyncStreamOptions,
        ) -> Result<(
            mpsc::Receiver<(PathBuf, Result<Metadata>)>,
            AsyncStreamHandle,
        )>;

        /// 异步处理大文件
        ///
        /// 适合处理视频等大文件，支持分块读取和进度跟踪。
        async fn stream_large_file<P: AsRef<Path> + Send>(
            &self,
            path: P,
            options: &AsyncStreamOptions,
        ) -> Result<(mpsc::Receiver<StreamEvent>, AsyncStreamHandle)>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_progress_tracker() {
        let tracker = ProgressTracker::new(100, None);

        tracker.update(25);
        assert_eq!(tracker.processed(), 25);
        assert_eq!(tracker.percentage(), 25.0);

        tracker.update(50);
        assert_eq!(tracker.processed(), 75);
        assert_eq!(tracker.percentage(), 75.0);
    }

    #[test]
    fn test_progress_tracker_callback() {
        let called = Arc::new(AtomicU64::new(0));
        let called_clone = Arc::clone(&called);

        let callback: ProgressCallback = Arc::new(move |processed, total| {
            called_clone.store(processed as u64, Ordering::SeqCst);
            assert_eq!(total, 100);
            true
        });

        let tracker = ProgressTracker::new(100, Some(callback));
        tracker.update(50);

        assert_eq!(called.load(Ordering::SeqCst), 50);
    }

    #[test]
    fn test_progress_reader() {
        let data = b"Hello, World!";
        let tracker = Arc::new(ProgressTracker::new(data.len(), None));

        let mut reader = ProgressReader::new(Cursor::new(data), tracker.clone(), 1024);

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, data);
        assert_eq!(tracker.processed(), data.len() as u64);
    }

    #[test]
    fn test_performance_stats() {
        let stats = PerformanceStats::default();

        stats.record(true, 1000);
        stats.record(true, 2000);
        stats.record(false, 500);

        assert_eq!(stats.total_operations.load(Ordering::SeqCst), 3);
        assert_eq!(stats.successful_operations.load(Ordering::SeqCst), 2);
        assert_eq!(stats.failed_operations.load(Ordering::SeqCst), 1);
        assert_eq!(stats.avg_time_us(), 1166); // (1000+2000+500)/3
    }
}
