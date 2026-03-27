//! 标准元数据标签

#[cfg(feature = "exif")]
pub mod exif;
#[cfg(feature = "gps")]
pub mod gps;
#[cfg(feature = "iptc")]
pub mod iptc;
#[cfg(feature = "xmp")]
pub mod xmp;

#[cfg(feature = "exif")]
pub use exif::*;
#[cfg(feature = "gps")]
pub use gps::*;
#[cfg(feature = "iptc")]
pub use iptc::*;
#[cfg(feature = "xmp")]
pub use xmp::*;
