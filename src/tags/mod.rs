//! 标签模块 - 按需加载的灵活标签系统
//!
//! 本模块支持通过 Cargo features 灵活选择需要编译的标签组。
//!
//! # 使用示例
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! # 默认使用平衡方案
//! exiftool-rs-wrapper = "0.1"
//!
//! # 仅使用标准 EXIF 标签（最小体积）
//! exiftool-rs-wrapper = { version = "0.1", default-features = false, features = ["minimal"] }
//!
//! # 标准元数据 + Canon 和 Nikon
//! exiftool-rs-wrapper = { version = "0.1", default-features = false, features = ["standard", "canon", "nikon"] }
//!
//! # 视频处理专用
//! exiftool-rs-wrapper = { version = "0.1", default-features = false, features = ["video", "canon", "nikon", "sony"] }
//! ```

// 允许未使用的常量，因为标签是为用户准备的
#![allow(dead_code)]
// 允许未使用的导入，因为这些导出是供用户使用的
#![allow(unused_imports)]

// 核心基础标签（始终编译）
pub mod common;

// 标准元数据标签
#[cfg(feature = "standard")]
pub mod standard;

// 厂商特定标签
#[cfg(feature = "vendors-common")]
pub mod vendors;

// 文件格式标签
#[cfg(feature = "image-formats")]
pub mod formats;

// 视频元数据
#[cfg(feature = "video")]
pub mod video;

// 音频元数据
#[cfg(feature = "audio")]
pub mod audio;

// 其他所有标签
#[cfg(feature = "other")]
pub mod other;

// 统一标签路由模块（解决重复标签的选择困惑）
pub mod unified;

// 重新导出
pub use common::*;

#[cfg(feature = "standard")]
pub use standard::*;

#[cfg(feature = "vendors-common")]
pub use vendors::*;

#[cfg(feature = "image-formats")]
pub use formats::*;

#[cfg(feature = "video")]
pub use video::*;

#[cfg(feature = "audio")]
pub use audio::*;

#[cfg(feature = "other")]
pub use other::*;
