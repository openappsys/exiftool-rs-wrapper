//! 标签模块

pub mod audio {
    pub mod id3;
}
pub mod formats {
    pub mod dng;
    pub mod gif;
    pub mod jpeg;
    pub mod pdf;
    pub mod png;
    pub mod raw;
    pub mod tiff;
}
pub mod other;
pub mod standard {
    pub mod exif;
    pub mod gps;
    pub mod iptc;
    pub mod xmp;
}
pub mod vendors {
    pub mod canon;
    pub mod fuji;
    pub mod nikon;
    pub mod olympus;
    pub mod panasonic;
    pub mod sony;
}
pub mod video {
    pub mod quicktime;
}

// 统一导出常用标签
pub mod unified;
