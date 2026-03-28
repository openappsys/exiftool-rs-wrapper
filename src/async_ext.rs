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

    /// 获取查询构建器
    ///
    /// 返回同步的 `QueryBuilder`，用户可以链式调用所有查询选项，
    /// 最后使用 `async_execute()` 异步执行。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::AsyncExifTool;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let async_et = AsyncExifTool::new()?;
    ///
    /// // 使用完整的链式 API
    /// let metadata = async_et.query_builder("photo.jpg")
    ///     .binary()
    ///     .group_headings(None)
    ///     .tag("Make")
    ///     .tag("Model")
    ///     .async_execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_builder<P: AsRef<Path>>(&self, path: P) -> crate::QueryBuilder<'_> {
        self.inner.query(path)
    }

    /// 获取写入构建器
    ///
    /// 返回同步的 `WriteBuilder`，用户可以链式调用所有写入选项，
    /// 最后使用 `async_execute()` 异步执行。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::AsyncExifTool;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let async_et = AsyncExifTool::new()?;
    ///
    /// // 使用完整的链式 API
    /// let result = async_et.write_builder("photo.jpg")
    ///     .tag("Artist", "Photographer")
    ///     .tag("Copyright", "© 2026")
    ///     .overwrite_original(true)
    ///     .async_execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_builder<P: AsRef<Path>>(&self, path: P) -> crate::WriteBuilder<'_> {
        self.inner.write(path)
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

    /// 异步获取可写文件类型扩展名列表
    pub async fn list_writable_file_extensions(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_writable_file_extensions()).await
    }

    /// 异步生成能力快照
    pub async fn capability_snapshot(&self) -> Result<CapabilitySnapshot> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.capability_snapshot()).await
    }

    // ── 补齐缺失的异步方法 ──

    /// 异步获取标签列表（`-list`）
    pub async fn list_tags(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_tags()).await
    }

    /// 异步获取可读文件扩展名列表（`-listr`）
    pub async fn list_readable_file_extensions(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_readable_file_extensions()).await
    }

    /// 异步获取支持的 GPS 日志格式（`-listgeo`）
    pub async fn list_geo_formats(&self) -> Result<Vec<String>> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || exiftool.list_geo_formats()).await
    }

    /// 异步删除备份文件
    ///
    /// 使用 `-delete_original` 选项删除 `_original` 备份文件。
    pub async fn delete_original<P: AsRef<Path> + Send>(&self, path: P, force: bool) -> Result<()> {
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.delete_original(&path, force)).await
    }

    /// 异步从备份恢复原始文件
    ///
    /// 使用 `-restore_original` 选项从 `_original` 备份文件恢复原始文件。
    pub async fn restore_original<P: AsRef<Path> + Send>(&self, path: P) -> Result<()> {
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.restore_original(&path)).await
    }

    /// 异步执行原始命令
    ///
    /// 高级 API，允许直接发送参数到 ExifTool。
    pub async fn execute(&self, args: Vec<String>) -> Result<crate::process::Response> {
        let exiftool = Arc::clone(&self.inner);
        run_blocking(move || {
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            exiftool.execute(&str_args)
        })
        .await
    }

    /// 异步读取文件元数据并反序列化为结构体
    pub async fn read_struct<T, P>(&self, path: P) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
        P: AsRef<Path> + Send,
    {
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.read_struct(&path)).await
    }

    // ── 扩展 trait 异步包装 ──

    /// 异步偏移日期时间标签
    pub async fn shift_datetime<P: AsRef<Path> + Send>(
        &self,
        path: P,
        offset: crate::advanced::DateTimeOffset,
    ) -> Result<()> {
        use crate::advanced::AdvancedWriteOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.shift_datetime(&path, offset)).await
    }

    /// 异步读取二进制数据
    pub async fn read_binary<P: AsRef<Path> + Send>(
        &self,
        path: P,
        tag: crate::binary::BinaryTag,
    ) -> Result<Vec<u8>> {
        use crate::binary::BinaryOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.read_binary(&path, tag)).await
    }

    /// 异步提取缩略图到文件
    pub async fn extract_thumbnail<P: AsRef<Path> + Send, Q: AsRef<Path> + Send>(
        &self,
        source: P,
        dest: Q,
    ) -> Result<()> {
        use crate::binary::BinaryOperations;
        let exiftool = Arc::clone(&self.inner);
        let source = source.as_ref().to_path_buf();
        let dest = dest.as_ref().to_path_buf();
        run_blocking(move || exiftool.extract_thumbnail(&source, &dest)).await
    }

    /// 异步比较两个文件的元数据
    pub async fn diff<P: AsRef<Path> + Send, Q: AsRef<Path> + Send>(
        &self,
        source: P,
        target: Q,
    ) -> Result<crate::config::DiffResult> {
        use crate::config::ConfigOperations;
        let exiftool = Arc::clone(&self.inner);
        let source = source.as_ref().to_path_buf();
        let target = target.as_ref().to_path_buf();
        run_blocking(move || exiftool.diff(&source, &target)).await
    }

    /// 异步获取十六进制转储
    pub async fn hex_dump<P: AsRef<Path> + Send>(
        &self,
        path: P,
        options: crate::config::HexDumpOptions,
    ) -> Result<String> {
        use crate::config::HexDumpOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.hex_dump(&path, &options)).await
    }

    /// 异步获取详细输出
    pub async fn verbose_dump<P: AsRef<Path> + Send>(
        &self,
        path: P,
        options: crate::config::VerboseOptions,
    ) -> Result<String> {
        use crate::config::VerboseOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.verbose_dump(&path, &options)).await
    }

    /// 异步重命名文件
    pub async fn rename_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
        pattern: crate::file_ops::RenamePattern,
    ) -> Result<std::path::PathBuf> {
        use crate::file_ops::FileOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.rename_file(&path, &pattern)).await
    }

    /// 异步按日期组织文件到目录
    pub async fn organize_by_date<P: AsRef<Path> + Send, Q: AsRef<Path> + Send>(
        &self,
        path: P,
        target_dir: Q,
        date_format: String,
    ) -> Result<std::path::PathBuf> {
        use crate::file_ops::FileOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        let target_dir = target_dir.as_ref().to_path_buf();
        run_blocking(move || exiftool.organize_by_date(&path, &target_dir, &date_format)).await
    }

    /// 异步使用自定义格式读取元数据
    pub async fn read_formatted<P: AsRef<Path> + Send>(
        &self,
        path: P,
        options: crate::format::ReadOptions,
    ) -> Result<crate::format::FormattedOutput> {
        use crate::format::FormatOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.read_formatted(&path, &options)).await
    }

    /// 异步读取为 XML 格式
    pub async fn read_xml<P: AsRef<Path> + Send>(&self, path: P) -> Result<String> {
        use crate::format::FormatOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.read_xml(&path)).await
    }

    /// 异步读取为 CSV 格式
    pub async fn read_csv<P: AsRef<Path> + Send>(&self, path: P) -> Result<String> {
        use crate::format::FormatOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.read_csv(&path)).await
    }

    /// 异步获取 GPS 坐标
    pub async fn get_gps<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> Result<Option<crate::geo::GpsCoordinate>> {
        use crate::geo::GeoOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.get_gps(&path)).await
    }

    /// 异步设置 GPS 坐标
    pub async fn set_gps<P: AsRef<Path> + Send>(
        &self,
        path: P,
        coord: crate::geo::GpsCoordinate,
    ) -> Result<()> {
        use crate::geo::GeoOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.set_gps(&path, &coord)).await
    }

    /// 异步删除 GPS 信息
    pub async fn remove_gps<P: AsRef<Path> + Send>(&self, path: P) -> Result<()> {
        use crate::geo::GeoOperations;
        let exiftool = Arc::clone(&self.inner);
        let path = path.as_ref().to_path_buf();
        run_blocking(move || exiftool.remove_gps(&path)).await
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

    #[tokio::test]
    async fn test_query_builder_async_execute() {
        // 验证 query_builder 链式调用 + async_execute 可用
        let et = match AsyncExifTool::new() {
            Ok(et) => et,
            Err(crate::Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let temp = tempfile::tempdir().expect("创建临时目录失败");
        let file = temp.path().join("test.jpg");
        std::fs::write(&file, crate::tests::tiny_jpeg()).expect("写入测试图片失败");

        // 使用链式 API 进行异步查询
        let metadata = et
            .query_builder(&file)
            .tag("FileName")
            .tag("FileSize")
            .raw_values(true)
            .async_execute()
            .await
            .expect("异步查询应成功");

        assert!(metadata.get("FileName").is_some());
    }

    #[tokio::test]
    async fn test_write_builder_async_execute() {
        // 验证 write_builder 链式调用 + async_execute 可用
        let et = match AsyncExifTool::new() {
            Ok(et) => et,
            Err(crate::Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let temp = tempfile::tempdir().expect("创建临时目录失败");
        let file = temp.path().join("test.jpg");
        std::fs::write(&file, crate::tests::tiny_jpeg()).expect("写入测试图片失败");

        // 使用链式 API 进行异步写入
        let result = et
            .write_builder(&file)
            .tag("Artist", "AsyncTest")
            .tag("Copyright", "© 2026")
            .overwrite_original(true)
            .async_execute()
            .await
            .expect("异步写入应成功");

        assert!(result.is_success());

        // 验证写入结果
        let metadata = et
            .query_builder(&file)
            .tag("Artist")
            .async_execute()
            .await
            .expect("异步查询应成功");

        let artist = metadata.get("Artist").expect("应包含 Artist 标签");
        assert_eq!(artist.to_string(), "AsyncTest");
    }

    #[tokio::test]
    async fn test_query_builder_async_execute_text() {
        // 验证 async_execute_text 可用
        let et = match AsyncExifTool::new() {
            Ok(et) => et,
            Err(crate::Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let temp = tempfile::tempdir().expect("创建临时目录失败");
        let file = temp.path().join("test.jpg");
        std::fs::write(&file, crate::tests::tiny_jpeg()).expect("写入测试图片失败");

        let text = et
            .query_builder(&file)
            .print_format("$FileName")
            .async_execute_text()
            .await
            .expect("异步文本查询应成功");

        assert!(text.contains("test.jpg"));
    }
}
