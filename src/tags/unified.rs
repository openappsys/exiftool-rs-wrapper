//! 统一标签路由模块
//!
//! 本模块为常用标签提供统一访问接口，解决同一标签在多个模块中定义时的选择困惑。
//!
//! 路由优先级：
//! 1. standard/exif - 标准 EXIF 标签（最高优先级）
//! 2. 特定模块 - 按需暴露底层模块常量
//!
//! 使用示例：
//! ```rust,ignore
//! use exiftool_rs_wrapper::tags::unified::{EXPOSURE_TIME, ISO, F_NUMBER};
//! // 或者统一导入所有
//! use exiftool_rs_wrapper::tags::unified::*;
//! ```

// ============================================================================
// 核心标准标签（优先使用 standard/exif）
// ============================================================================

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::EXPOSURE_TIME;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::F_NUMBER;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::ISO;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::FOCAL_LENGTH;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::FLASH;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::WHITE_BALANCE;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::METERING_MODE;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::EXPOSURE_PROGRAM;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::EXPOSURE_COMPENSATION;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::ORIENTATION;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::RESOLUTION_UNIT;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::DATE_TIME_ORIGINAL;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::CREATE_DATE;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::MODIFY_DATE;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::MAKE;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::MODEL;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::SOFTWARE;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::COPYRIGHT;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::ARTIST;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::IMAGE_WIDTH;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::IMAGE_HEIGHT;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::BITS_PER_SAMPLE;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::Y_CB_CR_SUB_SAMPLING;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::Y_CB_CR_POSITIONING;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::X_RESOLUTION;

#[cfg(feature = "exif")]
pub use crate::tags::standard::exif::Y_RESOLUTION;

// ============================================================================
// GPS 标签（优先使用 standard/gps）
// ============================================================================

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_LATITUDE;

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_LONGITUDE;

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_ALTITUDE;

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_LATITUDE_REF;

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_LONGITUDE_REF;

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_ALTITUDE_REF;

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_TIMESTAMP;

#[cfg(feature = "gps")]
pub use crate::tags::standard::gps::GPS_DATE_STAMP;

// ============================================================================
// IPTC 标签（优先使用 standard/iptc）
// ============================================================================

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_OBJECT_NAME as OBJECT_NAME;

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_CAPTION_ABSTRACT as CAPTION;

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_KEYWORDS as KEYWORDS;

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_CITY as CITY;

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_PROVINCE_STATE as PROVINCE_STATE;

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_COUNTRY_PRIMARY_LOCATION_NAME as COUNTRY;

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_BY_LINE as BY_LINE;

#[cfg(feature = "iptc")]
pub use crate::tags::standard::iptc::IPTC_COPYRIGHT_NOTICE as COPYRIGHT_NOTICE;
