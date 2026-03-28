//! 异步 API 支持模块
//!
//! 提供基于 tokio 的异步 ExifTool API

use crate::CapabilitySnapshot;
use crate::ExifTool;
use crate::error::Result;
use crate::types::Metadata;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// 在阻塞线程池中执行同步 ExifTool 调用
async fn run_blocking<T, F>(f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T> + Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| crate::error::Error::process(format!("异步任务执行失败: {}", e)))?
}

/// 异步 ExifTool 包装器
#[derive(Debug, Clone)]
pub struct AsyncExifTool {
    inner: Arc<ExifTool>,
}

impl AsyncExifTool {
    /// 创建新的异步 ExifTool 实例
    pub fn new() -> Result<Self> {
        let exiftool = ExifTool::new()?;
        Ok(Self {
            inner: Arc::new(exiftool),
        })
    }

    /// 从同步 ExifTool 创建异步版本
    pub fn from_sync(exiftool: ExifTool) -> Self {
        Self {
            inner: Arc::new(exiftool),
        }
    }

    /// 异步查询元数据
    pub async fn query<P: AsRef<Path> + Send>(&self, path: P) -> Result<Metadata> {
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.query(&path).execute()).await
    }

    /// 异步查询特定标签
    pub async fn read_tag<T, P, S>(&self, path: P, tag: S) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
        P: AsRef<Path> + Send,
        S: AsRef<str> + Send,
    {
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        let tag = tag.as_ref().to_string();
        run_blocking(move || exiftool.read_tag(&path, &tag)).await
    }

    /// 异步批量查询
    pub async fn query_batch<P: AsRef<Path> + Send + Sync>(
        &self,
        paths: &[P],
    ) -> Result<Vec<(PathBuf, Metadata)>> {
        self.query_batch_with_concurrency(paths, 4).await
    }

    /// 异步批量查询（可控制并发度）
    pub async fn query_batch_with_concurrency<P: AsRef<Path> + Send + Sync>(
        &self,
        paths: &[P],
        concurrency: usize,
    ) -> Result<Vec<(PathBuf, Metadata)>> {
        let concurrency = concurrency.max(1);
        let paths: Vec<PathBuf> = paths.iter().map(|p| p.as_ref().to_path_buf()).collect();

        let results = process_files_parallel(paths.clone(), concurrency, {
            let exiftool = self.clone();
            move |path| {
                let exiftool = exiftool.clone();
                async move {
                    let metadata = exiftool.query(&path).await?;
                    Ok((path, metadata))
                }
            }
        })
        .await;

        let mut out = Vec::with_capacity(results.len());
        for item in results {
            out.push(item?);
        }

        Ok(out)
    }

    /// 异步写入标签
    pub async fn write_tag<P, S, V>(&self, path: P, tag: S, value: V) -> Result<()>
    where
        P: AsRef<Path> + Send,
        S: AsRef<str> + Send + Into<String>,
        V: AsRef<str> + Send + Into<String>,
    {
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        let tag = tag.into();
        let value = value.into();
        run_blocking(move || {
            exiftool
                .write(&path)
                .tag(tag, value)
                .overwrite_original(true)
                .execute()
                .map(|_| ())
        })
        .await
    }

    /// 异步删除标签
    pub async fn delete_tag<P, S>(&self, path: P, tag: S) -> Result<()>
    where
        P: AsRef<Path> + Send,
        S: AsRef<str> + Send + Into<String>,
    {
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        let tag = tag.into();
        run_blocking(move || {
            exiftool
                .write(&path)
                .delete(tag)
                .overwrite_original(true)
                .execute()
                .map(|_| ())
        })
        .await
    }

    /// 异步批量写入
    ///
    /// 并行处理多个文件的写入操作
    pub async fn write_batch<P, I, F>(&self, operations: I) -> Vec<Result<()>>
    where
        P: AsRef<Path> + Send + Sync + 'static,
        I: IntoIterator<Item = (P, F)>,
        F: FnOnce(crate::WriteBuilder<'_>) -> crate::WriteBuilder<'_> + Send + 'static,
    {
        self.write_batch_with_concurrency(operations, 4).await
    }

    /// 异步批量写入（可控制并发度）
    pub async fn write_batch_with_concurrency<P, I, F>(
        &self,
        operations: I,
        concurrency: usize,
    ) -> Vec<Result<()>>
    where
        P: AsRef<Path> + Send + Sync + 'static,
        I: IntoIterator<Item = (P, F)>,
        F: FnOnce(crate::WriteBuilder<'_>) -> crate::WriteBuilder<'_> + Send + 'static,
    {
        use futures::stream::{self, StreamExt};

        let concurrency = concurrency.max(1);
        let operations: Vec<_> = operations
            .into_iter()
            .map(|(path, f)| (path.as_ref().to_path_buf(), f))
            .collect();

        let exiftool = Arc::clone(&self.inner);
        stream::iter(operations)
            .map(move |(path, builder_fn)| {
                let exiftool = Arc::clone(&exiftool);
                async move {
                    run_blocking(move || builder_fn(exiftool.write(&path)).execute().map(|_| ()))
                        .await
                }
            })
            .buffer_unordered(concurrency)
            .collect()
            .await
    }

    /// 异步获取版本
    pub async fn version(&self) -> Result<String> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.version()).await
    }

    /// 异步关闭
    pub async fn close(&self) -> Result<()> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.close()).await
    }

    /// 异步获取可写标签列表（`-listw`）
    pub async fn list_writable_tags(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_writable_tags()).await
    }

    /// 异步获取文件扩展名列表（`-listf`）
    pub async fn list_file_extensions(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_file_extensions()).await
    }

    /// 异步获取分组列表（`-listg`）
    pub async fn list_groups(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_groups()).await
    }

    /// 异步获取标签描述列表（`-listd`）
    pub async fn list_descriptions(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_descriptions()).await
    }

    /// 异步生成能力快照
    pub async fn capability_snapshot(&self) -> Result<CapabilitySnapshot> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.capability_snapshot()).await
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
    use super::*;

    #[tokio::test]
    async fn test_process_files_parallel() {
        let inputs = vec![
            PathBuf::from("a.jpg"),
            PathBuf::from("b.jpg"),
            PathBuf::from("c.jpg"),
        ];
        let outputs: Vec<Result<String>> =
            process_files_parallel(inputs, 2, |v: PathBuf| async move {
                Ok::<_, crate::Error>(v.to_string_lossy().to_string())
            })
            .await;

        assert_eq!(outputs.len(), 3);
        assert!(outputs.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn test_async_exiftool_creation() {
        let _ = AsyncExifTool::new();
    }

    #[tokio::test]
    async fn test_async_list_writable_tags() {
        let et = match AsyncExifTool::new() {
            Ok(et) => et,
            Err(crate::Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let tags = et
            .list_writable_tags()
            .await
            .expect("list writable tags should succeed");
        assert!(!tags.is_empty());
    }

    #[tokio::test]
    async fn test_async_capability_snapshot() {
        let et = match AsyncExifTool::new() {
            Ok(et) => et,
            Err(crate::Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let snapshot = et
            .capability_snapshot()
            .await
            .expect("capability snapshot should succeed");
        assert!(snapshot.tags_count > 0);
    }
}
