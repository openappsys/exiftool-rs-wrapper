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
mod binary;
mod config;
mod error;
mod file_ops;
mod format;
mod geo;
mod pool;
mod process;
mod query;
mod retry;
mod stream;
pub mod tags;
mod types;
mod write;

/// Serde 结构体模块
#[cfg(feature = "serde-structs")]
pub mod structs;

/// 高级功能模块
mod advanced;

/// 异步 API 模块
#[cfg(feature = "async")]
pub mod async_ext;

// 公开导出
pub use advanced::{
    AdvancedWriteOperations, DateShiftDirection, DateTimeOffset, NumericOperation, TimeUnit,
};
pub use binary::{BinaryOperations, BinaryTag, BinaryWriteBuilder, BinaryWriteResult};
pub use error::{Error, Result};
pub use process::{CommandId, CommandRequest, Response};
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
pub use geo::{GeoOperations, GpsCoordinate};

// 配置和校验
pub use config::{
    ConfigOperations, DiffResult, HexDumpOperations, HexDumpOptions, VerboseOperations,
    VerboseOptions,
};

// 流式处理和性能优化
pub use stream::{
    Cache, PerformanceStats, ProgressCallback, ProgressReader, ProgressTracker, StreamOptions,
    StreamingOperations,
};

#[cfg(feature = "async")]
/// 异步流式处理模块（需要 async feature）
pub use stream::async_stream;

// 错误恢复和重试
pub use retry::{BatchResult, Recoverable, RetryPolicy, with_retry_sync};

#[cfg(feature = "async")]
pub use retry::with_retry;

#[cfg(feature = "async")]
pub use async_ext::{AsyncExifTool, process_files_parallel, read_metadata_parallel};

use process::ExifToolInner;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
    global_args: Arc<Vec<String>>,
}

/// ExifTool 能力快照
#[derive(Debug, Clone)]
pub struct CapabilitySnapshot {
    pub version: String,
    pub tags_count: usize,
    pub writable_tags_count: usize,
    pub file_extensions_count: usize,
    pub writable_file_extensions_count: usize,
    pub groups_count: usize,
    pub descriptions_count: usize,
}

/// ExifTool 构建器
pub struct ExifToolBuilder {
    executable: Option<std::path::PathBuf>,
    response_timeout: Option<Duration>,
    config_path: Option<std::path::PathBuf>,
}

impl ExifToolBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            executable: None,
            response_timeout: None,
            config_path: None,
        }
    }

    /// 指定 exiftool 可执行文件路径
    pub fn executable<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.executable = Some(path.into());
        self
    }

    /// 设置响应超时
    pub fn response_timeout(mut self, timeout: Duration) -> Self {
        self.response_timeout = Some(timeout);
        self
    }

    /// 指定 ExifTool 配置文件路径（等价于 `-config`）
    pub fn config<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.config_path = Some(path.into());
        self
    }

    /// 构建 ExifTool 实例
    pub fn build(self) -> Result<ExifTool> {
        let timeout = self
            .response_timeout
            .unwrap_or_else(|| Duration::from_secs(30));

        let inner = if let Some(exe) = self.executable {
            ExifToolInner::with_executable_and_timeout(exe, timeout)?
        } else {
            ExifToolInner::with_executable_and_timeout("exiftool", timeout)?
        };

        let mut global_args = Vec::new();
        if let Some(config_path) = self.config_path {
            global_args.push("-config".to_string());
            global_args.push(config_path.to_string_lossy().to_string());
        }

        Ok(ExifTool {
            inner: Arc::new(Mutex::new(inner)),
            global_args: Arc::new(global_args),
        })
    }
}

impl Default for ExifToolBuilder {
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
            global_args: Arc::new(Vec::new()),
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
    pub fn builder() -> ExifToolBuilder {
        ExifToolBuilder::new()
    }

    /// 基于当前实例附加配置文件参数（`-config`）
    pub fn with_config<P: AsRef<Path>>(&self, path: P) -> Self {
        let mut global_args = self.global_args.as_ref().clone();
        global_args.push("-config".to_string());
        global_args.push(path.as_ref().to_string_lossy().to_string());

        Self {
            inner: Arc::clone(&self.inner),
            global_args: Arc::new(global_args),
        }
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
        let output = self.query(path).arg("-json").arg("-g2").execute_text()?;

        // ExifTool 的 JSON 输出是数组，需要提取第一个元素
        let json_array: Vec<serde_json::Value> = serde_json::from_str(&output)?;
        if json_array.is_empty() {
            return Err(Error::process("Empty JSON response from ExifTool"));
        }

        let result: T = serde_json::from_value(json_array[0].clone())?;
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

    /// 获取可写标签列表（对应 `-listw`）
    pub fn list_writable_tags(&self) -> Result<Vec<String>> {
        let response = self.execute(&["-listw"])?;
        Ok(parse_word_list(response.text()))
    }

    /// 获取支持的文件扩展名列表（对应 `-listf`）
    pub fn list_file_extensions(&self) -> Result<Vec<String>> {
        let response = self.execute(&["-listf"])?;
        Ok(parse_word_list(response.text()))
    }

    /// 获取支持的分组列表（对应 `-listg`）
    pub fn list_groups(&self) -> Result<Vec<String>> {
        let response = self.execute(&["-listg"])?;
        Ok(parse_word_list(response.text()))
    }

    /// 获取指定族的分组列表（对应 `-listg[NUM]`）
    ///
    /// ExifTool 将标签分组为不同的族（family），使用 `-listg[NUM]` 可以列出特定族的分组。
    ///
    /// # 参数
    ///
    /// - `family` - 分组族编号（0-7），对应 `-listg0` 到 `-listg7`
    ///   - Family 0: 信息类型（EXIF、IPTC、XMP 等）
    ///   - Family 1: 特定位置（MakerNotes、Composite 等）
    ///   - Family 2: 类别（Image、Camera、Author 等）
    ///   - Family 3-7: 其他分类方式
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 获取 Family 1 的分组列表（特定位置）
    /// let family1_groups = exiftool.list_groups_family(1)?;
    /// println!("Family 1 分组: {:?}", family1_groups);
    ///
    /// // 获取 Family 2 的分组列表（类别）
    /// let family2_groups = exiftool.list_groups_family(2)?;
    /// println!("Family 2 分组: {:?}", family2_groups);
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_groups_family(&self, family: u8) -> Result<Vec<String>> {
        let args = if family == 0 {
            vec!["-listg".to_string()]
        } else {
            vec![format!("-listg{}", family)]
        };
        let response = self.execute(&args)?;
        Ok(parse_word_list(response.text()))
    }

    /// 获取标签描述列表（对应 `-listd`）
    pub fn list_descriptions(&self) -> Result<Vec<String>> {
        let response = self.execute(&["-listd"])?;
        Ok(parse_word_list(response.text()))
    }

    /// 获取可写文件类型扩展名列表（对应 `-listwf`）
    ///
    /// 返回 ExifTool 支持写入的文件类型扩展名列表。
    /// 与 `list_file_extensions()` 不同，此方法仅返回可写入元数据的文件类型。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let writable_exts = exiftool.list_writable_file_extensions()?;
    /// println!("可写文件类型数量: {}", writable_exts.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_writable_file_extensions(&self) -> Result<Vec<String>> {
        let response = self.execute(&["-listwf"])?;
        Ok(parse_word_list(response.text()))
    }

    /// 获取可读文件类型扩展名列表（对应 `-listr`）
    ///
    /// 返回 ExifTool 支持读取的文件类型扩展名列表。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let readable_exts = exiftool.list_readable_file_extensions()?;
    /// println!("可读文件类型数量: {}", readable_exts.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_readable_file_extensions(&self) -> Result<Vec<String>> {
        let response = self.execute(&["-listr"])?;
        Ok(parse_word_list(response.text()))
    }

    /// 获取支持的 GPS 日志格式列表（对应 `-listgeo`）
    ///
    /// 返回 ExifTool 支持的地理标记日志文件格式列表。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let geo_formats = exiftool.list_geo_formats()?;
    /// println!("支持的 GPS 日志格式: {:?}", geo_formats);
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_geo_formats(&self) -> Result<Vec<String>> {
        let response = self.execute(&["-listgeo"])?;
        Ok(parse_word_list(response.text()))
    }

    /// 生成当前 ExifTool 的能力快照
    pub fn capability_snapshot(&self) -> Result<CapabilitySnapshot> {
        Ok(CapabilitySnapshot {
            version: self.version()?,
            tags_count: self.list_tags()?.len(),
            writable_tags_count: self.list_writable_tags()?.len(),
            file_extensions_count: self.list_file_extensions()?.len(),
            writable_file_extensions_count: self.list_writable_file_extensions()?.len(),
            groups_count: self.list_groups()?.len(),
            descriptions_count: self.list_descriptions()?.len(),
        })
    }

    /// 执行原始命令
    ///
    /// 这是高级 API，允许直接发送参数到 ExifTool。
    ///
    /// # 安全性
    ///
    /// 谨慎使用此功能，确保参数不包含恶意输入。
    pub fn execute<S: AsRef<str>>(&self, args: &[S]) -> Result<Response> {
        self.execute_raw(args)
    }

    pub(crate) fn execute_raw(&self, args: &[impl AsRef<str>]) -> Result<Response> {
        debug!("Executing raw command with {} args", args.len());

        let mut merged_args = self.global_args.as_ref().clone();
        merged_args.extend(args.iter().map(|a| a.as_ref().to_string()));

        let mut inner = self.inner.lock()?;
        inner.execute(&merged_args)
    }

    /// 批量执行多个命令（原子多命令）
    ///
    /// 使用 `-executeNUM` 格式在一个事务中发送多个命令，
    /// 通过编号区分各个命令的响应。
    ///
    /// # 优势
    ///
    /// - 减少进程间通信开销（一次性发送所有命令）
    /// - 原子性：所有命令在一个批次中执行
    /// - 支持复杂的读写链操作
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 批量执行多个命令
    /// let commands = vec![
    ///     vec!["-ver".to_string()],
    ///     vec!["-list".to_string()],
    /// ];
    /// let responses = exiftool.execute_multiple(&commands)?;
    ///
    /// for (idx, response) in responses.iter().enumerate() {
    ///     println!("命令 {}: {}", idx, response.text().trim());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn execute_multiple<S: AsRef<str>>(&self, commands: &[Vec<S>]) -> Result<Vec<Response>> {
        debug!("Executing {} commands atomically", commands.len());

        // 转换参数格式
        let converted_commands: Vec<Vec<String>> = commands
            .iter()
            .map(|cmd| cmd.iter().map(|a| a.as_ref().to_string()).collect())
            .collect();

        // 合并全局参数到每个命令
        let commands_with_global: Vec<Vec<String>> = converted_commands
            .iter()
            .map(|cmd| {
                let mut merged = self.global_args.as_ref().clone();
                merged.extend(cmd.iter().cloned());
                merged
            })
            .collect();

        let mut inner = self.inner.lock()?;
        inner.execute_multiple(&commands_with_global)
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

    #[cfg(test)]
    pub(crate) fn debug_global_args(&self) -> Vec<String> {
        self.global_args.as_ref().clone()
    }
}

fn parse_word_list(text: String) -> Vec<String> {
    text.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// 最小有效 JPEG 文件数据，供测试使用
    #[cfg(feature = "async")]
    pub(crate) fn tiny_jpeg() -> &'static [u8] {
        &[
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06,
            0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D,
            0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D,
            0x1A, 0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28,
            0x37, 0x29, 0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32,
            0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01,
            0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0xFF, 0xC4, 0x00,
            0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00,
            0xD2, 0xCF, 0x20, 0xFF, 0xD9,
        ]
    }

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

    #[test]
    fn test_builder_with_config_args() {
        let et = match ExifTool::builder()
            .config("/tmp/exiftool-test.config")
            .build()
        {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let args = et.debug_global_args();
        assert_eq!(args, vec!["-config", "/tmp/exiftool-test.config"]);
    }

    #[test]
    fn test_with_config_clone_args() {
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let configured = et.with_config("/tmp/exiftool-test.config");
        let args = configured.debug_global_args();
        assert_eq!(args, vec!["-config", "/tmp/exiftool-test.config"]);
    }

    #[test]
    fn test_public_execute_raw_passthrough() {
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let response = et.execute(&["-ver"]).expect("execute should succeed");
        let version = response.text().trim().to_string();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_list_writable_tags() {
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let tags = et
            .list_writable_tags()
            .expect("list writable tags should succeed");
        assert!(!tags.is_empty());
    }

    #[test]
    fn test_list_file_extensions() {
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let exts = et
            .list_file_extensions()
            .expect("list file extensions should succeed");
        assert!(!exts.is_empty());
    }

    #[test]
    fn test_list_groups() {
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let groups = et.list_groups().expect("list groups should succeed");
        assert!(!groups.is_empty());
    }

    #[test]
    fn test_list_descriptions() {
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let desc = et
            .list_descriptions()
            .expect("list descriptions should succeed");
        assert!(!desc.is_empty());
    }

    #[test]
    fn test_capability_snapshot() {
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let snapshot = et
            .capability_snapshot()
            .expect("capability snapshot should succeed");
        assert!(!snapshot.version.is_empty());
        assert!(snapshot.tags_count > 0);
        assert!(snapshot.writable_tags_count > 0);
        assert!(snapshot.file_extensions_count > 0);
        assert!(snapshot.writable_file_extensions_count > 0);
        assert!(snapshot.groups_count > 0);
        assert!(snapshot.descriptions_count > 0);
    }
}
