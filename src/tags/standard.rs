//! 标准 EXIF、IPTC、XMP、GPS 标签
//! 
//! 这些标签总是可用，不需要 feature flag

use crate::TagId;

impl TagId {
    // === EXIF 标准标签 ===
    pub const MAKE: Self = Self("Make");
    pub const MODEL: Self = Self("Model");
    pub const DATE_TIME_ORIGINAL: Self = Self("DateTimeOriginal");
    pub const DATE_TIME: Self = Self("DateTime");
    // ... 更多标准标签
}
