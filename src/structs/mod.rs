//! Serde 结构体支持
//!
//! 本模块提供预定义的元数据结构体，用于通过 serde 反序列化 ExifTool 输出。
//!
//! 需要在 Cargo.toml 中启用 `serde-structs` feature：
//!
//! ```toml
//! [dependencies]
//! exiftool-rs-wrapper = { version = "0.1.4", features = ["serde-structs"] }
//! ```
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use exiftool_rs_wrapper::ExifTool;
//! use exiftool_rs_wrapper::structs::Metadata;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let exiftool = ExifTool::new()?;
//! let meta: Metadata = exiftool.read_struct("photo.jpg")?;
//!
//! println!("File: {}", meta.file.file_name);
//! if let Some(exif) = meta.exif {
//!     println!("Camera: {:?}", exif.make);
//! }
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "serde-structs")]
pub mod common;

#[cfg(feature = "serde-structs")]
pub use common::*;
