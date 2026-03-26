//! 异步 API 支持模块
//!
//! 提供基于 tokio 的异步 ExifTool API

use crate::ExifTool;
use crate::error::Result;
use crate::types::Metadata;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// 异步 ExifTool 包装器
#[derive(Debug, Clone)]
pub struct AsyncExifTool {
    inner: Arc<tokio::sync::Mutex<ExifTool>>,
}

impl AsyncExifTool {
    /// 创建新的异步 ExifTool 实例
    pub fn new() -> Result<Self> {
        let exiftool = ExifTool::new()?;
        Ok(Self {
            inner: Arc::new(tokio::sync::Mutex::new(exiftool)),
        })
    }

    /// 从同步 ExifTool 创建异步版本
    pub fn from_sync(exiftool: ExifTool) -> Self {
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(exiftool)),
        }
    }

    /// 异步查询元数据
    pub async fn query<P: AsRef<Path> + Send>(&self, path: P) -> Result<Metadata> {
        let exiftool = self.inner.lock().await;
        exiftool.query(path).execute()
    }

    /// 异步查询特定标签
    pub async fn read_tag<T, P, S>(&self, path: P, tag: S) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
        P: AsRef<Path> + Send,
        S: AsRef<str> + Send,
    {
        let exiftool = self.inner.lock().await;
        exiftool.read_tag(path, tag)
    }

    /// 异步批量查询
    pub async fn query_batch<P: AsRef<Path> + Send + Sync>(
        &self,
        paths: &[P],
    ) -> Result<Vec<(PathBuf, Metadata)>> {
        let exiftool = self.inner.lock().await;
        exiftool.query_batch(paths).execute()
    }

    /// 异步写入标签
    pub async fn write_tag<P, S, V>(&self, path: P, tag: S, value: V) -> Result<()>
    where
        P: AsRef<Path> + Send,
        S: AsRef<str> + Send + Into<String>,
        V: AsRef<str> + Send + Into<String>,
    {
        let exiftool = self.inner.lock().await;
        exiftool
            .write(path)
            .tag(tag, value)
            .overwrite_original(true)
            .execute()
            .map(|_| ())
    }

    /// 异步删除标签
    pub async fn delete_tag<P, S>(&self, path: P, tag: S) -> Result<()>
    where
        P: AsRef<Path> + Send,
        S: AsRef<str> + Send + Into<String>,
    {
        let exiftool = self.inner.lock().await;
        exiftool
            .write(path)
            .delete(tag)
            .overwrite_original(true)
            .execute()
            .map(|_| ())
    }

    /// 异步批量写入
    ///
    /// 并行处理多个文件的写入操作
    pub async fn write_batch<P, I, F, Fut>(&self, operations: I) -> Vec<Result<()>>
    where
        P: AsRef<Path> + Send + Sync + 'static,
        I: IntoIterator<Item = (P, F)>,
        F: FnOnce(crate::WriteBuilder<'_>) -> crate::WriteBuilder<'_>,
    {
        let operations: Vec<_> = operations.into_iter().collect();
        let mut results = Vec::with_capacity(operations.len());

        for (path, builder_fn) in operations {
            let exiftool = self.inner.lock().await;
            let result = builder_fn(exiftool.write(path)).execute().map(|_| ());
            results.push(result);
        }

        results
    }

    /// 异步获取版本
    pub async fn version(&self) -> Result<String> {
        let exiftool = self.inner.lock().await;
        exiftool.version()
    }

    /// 异步关闭
    pub async fn close(&self) -> Result<()> {
        let exiftool = self.inner.lock().await;
        exiftool.close()
    }
}

impl Default for AsyncExifTool {
    fn default() -> Self {
        Self::new().expect("Failed to create AsyncExifTool")
    }
}

/// 异步批量处理辅助函数
///
/// 使用并行流处理大量文件
pub async fn process_files_parallel<P, F, Fut, R>(
    paths: Vec<P>,
    concurrency: usize,
    processor: F,
) -> Vec<Result<R>>
where
    P: AsRef<Path> + Send + Sync + 'static,
    F: Fn(P) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<R>> + Send,
    R: Send + 'static,
{
    use futures::stream::{self, StreamExt};
    use std::sync::Arc;

    let processor = Arc::new(processor);

    stream::iter(paths)
        .map(move |path| {
            let processor = Arc::clone(&processor);
            async move { processor(path).await }
        })
        .buffer_unordered(concurrency)
        .collect()
        .await
}

/// 并发元数据读取
pub async fn read_metadata_parallel<P: AsRef<Path> + Send + Sync + 'static>(
    exiftool: AsyncExifTool,
    paths: Vec<P>,
    concurrency: usize,
) -> Vec<Result<Metadata>> {
    process_files_parallel(paths, concurrency, move |path| {
        let et = exiftool.clone();
        async move { et.query(path).await }
    })
    .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_async_exiftool_creation() {
        // 创建异步 ExifTool
        // 注意：实际测试需要 tokio runtime
    }
}
