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
//!     .tag("Copyright", "© 2026")
//!     .overwrite_original(true)
//!     .execute()?;
//! # Ok(())
//! # }
//! ```

// 模块声明
mod batch;
mod binary;
mod config;
mod error;
mod file_ops;
mod format;
mod geo;
mod pool;
mod process;
mod query;
mod repl;
mod retry;
mod stream;
mod types;
mod write;

/// Serde 结构体模块
#[cfg(feature = "serde-structs")]
pub mod structs;

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
pub use query::{BatchQueryBuilder, EscapeFormat, QueryBuilder};
pub use types::{Metadata, TagId, TagValue};
pub use write::{WriteBuilder, WriteMode, WriteResult};

// 连接池
pub use pool::{ExifToolPool, PoolConnection, batch_with_pool, with_pool};

// 格式化输出
pub use format::{FormatOperations, FormattedOutput, OutputFormat, ReadOptions};

// 文件操作
pub use file_ops::{FileOperations, OrganizeOptions, RenamePattern};

// 地理信息
pub use geo::{GeoOperations, GeocodeResult, GpsCoordinate};

// 配置和校验
pub use config::{
    ChecksumAlgorithm, ChecksumResult, ConfigOperations, DiffResult, HexDumpOperations,
    HexDumpOptions, VerboseOperations, VerboseOptions,
};

// 流式处理和性能优化
pub use stream::{
    Cache, PerformanceStats, ProgressCallback, ProgressReader, ProgressTracker, StreamOptions,
    StreamingOperations,
};

// 错误恢复和重试
pub use retry::{BatchResult, Recoverable, RetryPolicy, with_retry_sync};

#[cfg(feature = "async")]
pub use retry::with_retry;

// 批处理脚本和管道
pub use batch::{BatchResult as ScriptBatchResult, BatchScript, PipeProcessor, example_script};

// REPL 交互式 shell
pub use repl::{ReplShell, run_repl};

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

/// ExifTool 构建器包装器
pub struct ExifToolBuilderWrapper {
    executable: Option<std::path::PathBuf>,
}

impl ExifToolBuilderWrapper {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { executable: None }
    }

    /// 指定 exiftool 可执行文件路径
    pub fn executable<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.executable = Some(path.into());
        self
    }

    /// 构建 ExifTool 实例
    pub fn build(self) -> Result<ExifTool> {
        let inner = if let Some(exe) = self.executable {
            ExifToolInner::with_executable(exe)?
        } else {
            ExifToolInner::new()?
        };

        Ok(ExifTool {
            inner: Arc::new(Mutex::new(inner)),
        })
    }
}

impl Default for ExifToolBuilderWrapper {
    fn default() -> Self {
        Self::new()
    }
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

    /// 创建 ExifTool 构建器
    ///
    /// 使用 Builder 模式可以更灵活地配置 ExifTool 实例。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // 使用自定义路径
    /// let exiftool = ExifTool::builder()
    ///     .executable("/usr/local/bin/exiftool")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> ExifToolBuilderWrapper {
        ExifToolBuilderWrapper::new()
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
    ///     .tag("Copyright", "© 2026 My Company")
    ///     .execute()?;
    ///
    /// // 高级写入
    /// exiftool.write("photo.jpg")
    ///     .tag("Artist", "Photographer")
    ///     .tag("Copyright", "© 2026")
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

    /// 读取文件元数据并反序列化为结构体
    ///
    /// 需要启用 `serde-structs` feature。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    /// use exiftool_rs_wrapper::structs::Metadata;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    /// let meta: Metadata = exiftool.read_struct("photo.jpg")?;
    /// println!("File: {}", meta.file.file_name);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "serde-structs")]
    pub fn read_struct<T, P>(&self, path: P) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
        P: AsRef<Path>,
    {
        // 获取 JSON 格式的原始输出
        let output = self
            .query(path)
            .arg("-json")
            .arg("-g2")
            .execute_text()?;

        let result: T = serde_json::from_str(&output)?;
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

    /// 删除备份文件
    ///
    /// 使用 `-delete_original` 选项删除 `_original` 备份文件。
    ///
    /// # 参数
    ///
    /// * `path` - 原始文件路径（ExifTool 会找到对应的备份文件）
    /// * `force` - 是否强制删除（使用 `-delete_original!`）
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 删除 photo.jpg 的备份文件 photo.jpg_original
    /// exiftool.delete_original("photo.jpg", false)?;
    ///
    /// // 强制删除备份文件
    /// exiftool.delete_original("photo.jpg", true)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_original<P: AsRef<Path>>(&self, path: P, force: bool) -> Result<()> {
        let arg = if force {
            "-delete_original!"
        } else {
            "-delete_original"
        };
        let args = vec![arg.to_string(), path.as_ref().to_string_lossy().to_string()];
        self.execute_raw(&args)?;
        Ok(())
    }

    /// 从备份恢复原始文件
    ///
    /// 使用 `-restore_original` 选项从 `_original` 备份文件恢复原始文件。
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 从 photo.jpg_original 恢复 photo.jpg
    /// exiftool.restore_original("photo.jpg")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn restore_original<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let args = vec![
            "-restore_original".to_string(),
            path.as_ref().to_string_lossy().to_string(),
        ];
        self.execute_raw(&args)?;
        Ok(())
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
mod tags;
