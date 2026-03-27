//! 标签模块 - 按类别组织的预定义标签常量
//! 
//! 使用 Cargo features 控制编译：
//! - common: 基础标签（总是包含）
//! - exif: EXIF 标准标签
//! - iptc: IPTC 标签
//! - xmp: XMP 标签
//! - gps: GPS 标签
//! - canon: Canon MakerNotes
//! - nikon: Nikon MakerNotes
//! - sony: Sony MakerNotes
//! - fuji: FujiFilm MakerNotes
//! - olympus: Olympus MakerNotes
//! - panasonic: Panasonic MakerNotes
//! - other: 其他所有标签（15000+个）

// 基础标签（总是包含）
pub mod common;

// 标准标签
#[cfg(feature = "exif")]
pub mod exif;

#[cfg(feature = "iptc")]
pub mod iptc;

#[cfg(feature = "xmp")]
pub mod xmp;

#[cfg(feature = "gps")]
pub mod gps;

// 厂商标签
#[cfg(feature = "canon")]
pub mod canon;

#[cfg(feature = "nikon")]
pub mod nikon;

#[cfg(feature = "sony")]
pub mod sony;

#[cfg(feature = "fuji")]
pub mod fuji;

#[cfg(feature = "olympus")]
pub mod olympus;

#[cfg(feature = "panasonic")]
pub mod panasonic;

// 其他所有标签
#[cfg(feature = "other")]
pub mod other;
