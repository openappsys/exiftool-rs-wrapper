//! ExifTool Rust Wrapper
//!
//! 一个高性能、类型安全的 ExifTool Rust 封装库。
//!
//! # 特性
//!
//! - **`-stay_open` 模式**：保持进程运行以获得最佳性能
//! - **类型安全**：完整的标签类型系统
//! - **Builder 模式**：符合 Rust 习惯的 API
//! - **线程安全**：支持多线程并发访问
//! - **零拷贝**：最小化内存分配
//!
//! # 示例
//!
//! ```rust,no_run
//! use exiftool_rs_wrapper::ExifTool;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建 ExifTool 实例
//! let exiftool = ExifTool::new()?;
//!
//! // 读取元数据
//! let metadata = exiftool.query("photo.jpg").execute()?;
//! println!("相机制造商: {:?}", metadata.get("Make"));
//!
//! // 写入元数据
//! exiftool.write("photo.jpg")
//!     .tag("Copyright", "© 2024")
//!     .overwrite_original(true)
//!     .execute()?;
//! # Ok(())
//! # }
//! ```

// 模块声明
mod binary;
mod error;
mod format;
mod pool;
mod process;
mod query;
mod types;
mod write;

/// 高级功能模块
pub mod advanced;

/// 异步 API 模块
#[cfg(feature = "async")]
pub mod async_ext;

// 公开导出
pub use advanced::{
    AdvancedWriteOperations, DateShiftDirection, DateTimeOffset, NumericOperation, TimeUnit,
};
pub use binary::{BinaryOperations, BinaryTag, BinaryWriteBuilder, BinaryWriteResult};
pub use error::{Error, Result};
pub use process::Response;
pub use query::{BatchQueryBuilder, QueryBuilder};
pub use types::{Metadata, TagId, TagValue};
pub use write::{WriteBuilder, WriteResult};

// 连接池
pub use pool::{ExifToolPool, PoolConnection, batch_with_pool, with_pool};

// 格式化输出
pub use format::{FormatOperations, FormattedOutput, OutputFormat, ReadOptions};

#[cfg(feature = "async")]
pub use async_ext::{AsyncExifTool, process_files_parallel, read_metadata_parallel};

use process::ExifToolInner;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

/// ExifTool 主结构体
///
/// 使用 `-stay_open` 模式保持 ExifTool 进程运行，
/// 避免每次操作都重新启动进程的开销。
///
/// # 线程安全
///
/// `ExifTool` 是线程安全的，可以在多个线程间共享。
/// 内部使用 `Arc<Mutex>` 保护进程通信。
#[derive(Debug, Clone)]
pub struct ExifTool {
    inner: Arc<Mutex<ExifToolInner>>,
}

impl ExifTool {
    /// 创建新的 ExifTool 实例
    ///
    /// 启动一个 `-stay_open` 模式的 ExifTool 进程。
    ///
    /// # 错误
    ///
    /// 如果 ExifTool 未安装或无法启动，返回 `Error::ExifToolNotFound`。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// let exiftool = ExifTool::new()?;
    /// # Ok::<(), exiftool_rs_wrapper::Error>(())
    /// ```
    pub fn new() -> Result<Self> {
        info!("Creating new ExifTool instance");

        let inner = ExifToolInner::new()?;

        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    /// 查询单个文件的元数据
    ///
    /// 返回一个 `QueryBuilder`，可以使用 Builder 模式配置查询选项。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 基本查询
    /// let metadata = exiftool.query("photo.jpg").execute()?;
    ///
    /// // 高级查询
    /// let metadata = exiftool.query("photo.jpg")
    ///     .include_unknown(true)
    ///     .tag("Make")
    ///     .tag("Model")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn query<P: AsRef<Path>>(&self, path: P) -> QueryBuilder<'_> {
        QueryBuilder::new(self, path)
    }

    /// 批量查询多个文件的元数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let paths = vec!["photo1.jpg", "photo2.jpg", "photo3.jpg"];
    /// let results = exiftool.query_batch(&paths)
    ///     .tag("FileName")
    ///     .tag("ImageSize")
    ///     .execute()?;
    ///
    /// for (path, metadata) in results {
    ///     println!("{}: {:?}", path.display(), metadata.get("FileName"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_batch<P: AsRef<Path>>(&self, paths: &[P]) -> BatchQueryBuilder<'_> {
        let path_bufs: Vec<PathBuf> = paths.iter().map(|p| p.as_ref().to_path_buf()).collect();
        BatchQueryBuilder::new(self, path_bufs)
    }

    /// 写入元数据到文件
    ///
    /// 返回一个 `WriteBuilder`，可以使用 Builder 模式配置写入选项。
    ///
    /// # 警告
    ///
    /// 默认情况下，ExifTool 会创建备份文件（`filename_original`）。
    /// 使用 `overwrite_original(true)` 可以不创建备份直接覆盖原文件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 基本写入
    /// exiftool.write("photo.jpg")
    ///     .tag("Copyright", "© 2024 My Company")
    ///     .execute()?;
    ///
    /// // 高级写入
    /// exiftool.write("photo.jpg")
    ///     .tag("Artist", "Photographer")
    ///     .tag("Copyright", "© 2024")
    ///     .delete("Comment")
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write<P: AsRef<Path>>(&self, path: P) -> WriteBuilder<'_> {
        WriteBuilder::new(self, path)
    }

    /// 读取单个标签的值
    ///
    /// 这是 `query().tag().execute()` 的快捷方式。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let make: String = exiftool.read_tag("photo.jpg", "Make")?;
    /// println!("相机制造商: {}", make);
    ///
    /// // 使用 TagId
    /// use exiftool_rs_wrapper::TagId;
    /// let model: String = exiftool.read_tag("photo.jpg", "Model")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_tag<T, P, S>(&self, path: P, tag: S) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let metadata = self.query(path).tag(tag.as_ref()).execute()?;

        let value = metadata
            .get(tag.as_ref())
            .ok_or_else(|| Error::TagNotFound(tag.as_ref().to_string()))?;

        // 将 TagValue 转换为目标类型
        let json = serde_json::to_value(value)?;
        let result: T = serde_json::from_value(json)?;

        Ok(result)
    }

    /// 获取 ExifTool 版本
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    /// let version = exiftool.version()?;
    /// println!("ExifTool version: {}", version);
    /// # Ok(())
    /// # }
    /// ```
    pub fn version(&self) -> Result<String> {
        let mut inner = self.inner.lock()?;
        inner.send_line("-ver")?;
        inner.send_line("-execute")?;
        inner.flush()?;

        let response = inner.read_response()?;
        Ok(response.text().trim().to_string())
    }

    /// 获取所有支持的标签列表
    pub fn list_tags(&self) -> Result<Vec<String>> {
        let mut inner = self.inner.lock()?;
        inner.send_line("-list")?;
        inner.send_line("-execute")?;
        inner.flush()?;

        let response = inner.read_response()?;
        let tags: Vec<String> = response
            .lines()
            .iter()
            .map(|line| line.trim().to_string())
            .filter(|line| {
                !line.is_empty() && !line.starts_with('-') && !line.contains("Command-line")
            })
            .collect();

        Ok(tags)
    }

    /// 执行原始命令
    ///
    /// 这是高级 API，允许直接发送参数到 ExifTool。
    ///
    /// # 安全性
    ///
    /// 谨慎使用此功能，确保参数不包含恶意输入。
    pub(crate) fn execute_raw(&self, args: &[impl AsRef<str>]) -> Result<Response> {
        debug!("Executing raw command with {} args", args.len());

        let args: Vec<String> = args.iter().map(|a| a.as_ref().to_string()).collect();

        let mut inner = self.inner.lock()?;
        inner.execute(&args)
    }

    /// 关闭 ExifTool 进程
    ///
    /// 优雅地关闭进程。通常不需要手动调用，
    /// 因为 `Drop` 实现会自动处理。
    pub fn close(&self) -> Result<()> {
        let mut inner = self.inner.lock()?;
        inner.close()
    }
}

impl Default for ExifTool {
    fn default() -> Self {
        Self::new().expect("Failed to create default ExifTool instance")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exiftool_new() {
        // 仅在 ExifTool 可用时运行
        match ExifTool::new() {
            Ok(_) => {
                println!("✓ ExifTool is available");
            }
            Err(Error::ExifToolNotFound) => {
                println!("⚠ ExifTool not found, skipping test");
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_version() {
        match ExifTool::new() {
            Ok(et) => {
                let version = et.version().unwrap();
                assert!(!version.is_empty());
                println!("ExifTool version: {}", version);
            }
            Err(Error::ExifToolNotFound) => {
                println!("⚠ ExifTool not found, skipping test");
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
