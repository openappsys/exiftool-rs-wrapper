//! 常用元数据结构体
//!
//! 这些结构体对应 ExifTool 的 `-g2` 分组输出格式。

use serde::Deserialize;

/// 完整的元数据结构体（分层）
#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    /// 源文件路径
    #[serde(rename = "SourceFile")]
    pub source_file: String,

    /// 文件信息（必定存在）
    #[serde(rename = "File")]
    pub file: FileInfo,

    /// EXIF 信息（可能不存在）
    #[serde(rename = "EXIF")]
    pub exif: Option<ExifInfo>,

    /// GPS 信息（可能不存在）
    #[serde(rename = "GPS")]
    pub gps: Option<GpsInfo>,

    /// IPTC 信息（可能不存在）
    #[serde(rename = "IPTC")]
    pub iptc: Option<IptcInfo>,

    /// XMP 信息（可能不存在）
    #[serde(rename = "XMP")]
    pub xmp: Option<XmpInfo>,
}

/// 文件信息
#[derive(Debug, Clone, Deserialize)]
pub struct FileInfo {
    /// 文件名
    #[serde(rename = "FileName")]
    pub file_name: String,

    /// 文件大小
    #[serde(rename = "FileSize")]
    pub file_size: String,

    /// MIME 类型
    #[serde(rename = "MIMEType")]
    pub mime_type: String,

    /// 文件修改日期
    #[serde(rename = "FileModifyDate")]
    pub modify_date: Option<String>,

    /// 文件访问日期
    #[serde(rename = "FileAccessDate")]
    pub access_date: Option<String>,

    /// 文件创建日期
    #[serde(rename = "FileCreateDate")]
    pub create_date: Option<String>,
}

/// EXIF 信息
#[derive(Debug, Clone, Deserialize)]
pub struct ExifInfo {
    /// 制造商
    #[serde(rename = "Make")]
    pub make: Option<String>,

    /// 型号
    #[serde(rename = "Model")]
    pub model: Option<String>,

    /// 图像宽度
    #[serde(rename = "ImageWidth")]
    pub image_width: Option<u32>,

    /// 图像高度
    #[serde(rename = "ImageHeight")]
    pub image_height: Option<u32>,

    /// 原始日期时间
    #[serde(rename = "DateTimeOriginal")]
    pub date_time_original: Option<String>,

    /// 数字化日期时间
    #[serde(rename = "DateTimeDigitized")]
    pub date_time_digitized: Option<String>,

    /// ISO 感光度
    #[serde(rename = "ISO")]
    pub iso: Option<u32>,

    /// 光圈值
    #[serde(rename = "FNumber")]
    pub f_number: Option<f64>,

    /// 曝光时间
    #[serde(rename = "ExposureTime")]
    pub exposure_time: Option<f64>,

    /// 焦距
    #[serde(rename = "FocalLength")]
    pub focal_length: Option<f64>,

    /// 曝光程序
    #[serde(rename = "ExposureProgram")]
    pub exposure_program: Option<String>,

    /// 测光模式
    #[serde(rename = "MeteringMode")]
    pub metering_mode: Option<String>,

    /// 闪光灯
    #[serde(rename = "Flash")]
    pub flash: Option<String>,

    /// 白平衡
    #[serde(rename = "WhiteBalance")]
    pub white_balance: Option<String>,
}

/// GPS 信息
#[derive(Debug, Clone, Deserialize)]
pub struct GpsInfo {
    /// GPS 纬度
    #[serde(rename = "GPSLatitude")]
    pub latitude: Option<f64>,

    /// GPS 经度
    #[serde(rename = "GPSLongitude")]
    pub longitude: Option<f64>,

    /// GPS 高度
    #[serde(rename = "GPSAltitude")]
    pub altitude: Option<f64>,

    /// GPS 时间戳
    #[serde(rename = "GPSTimeStamp")]
    pub time_stamp: Option<String>,

    /// GPS 日期戳
    #[serde(rename = "GPSDateStamp")]
    pub date_stamp: Option<String>,

    /// GPS 纬度引用（N/S）
    #[serde(rename = "GPSLatitudeRef")]
    pub latitude_ref: Option<String>,

    /// GPS 经度引用（E/W）
    #[serde(rename = "GPSLongitudeRef")]
    pub longitude_ref: Option<String>,
}

/// IPTC 信息
#[derive(Debug, Clone, Deserialize)]
pub struct IptcInfo {
    /// 对象名称
    #[serde(rename = "ObjectName")]
    pub object_name: Option<String>,

    /// 关键词
    #[serde(rename = "Keywords")]
    pub keywords: Option<Vec<String>>,

    /// 标题/说明
    #[serde(rename = "Caption-Abstract")]
    pub caption: Option<String>,

    /// 创建者
    #[serde(rename = "By-line")]
    pub by_line: Option<String>,

    /// 版权声明
    #[serde(rename = "CopyrightNotice")]
    pub copyright: Option<String>,

    /// 城市
    #[serde(rename = "City")]
    pub city: Option<String>,

    /// 省/州
    #[serde(rename = "Province-State")]
    pub state: Option<String>,

    /// 国家
    #[serde(rename = "Country-PrimaryLocationName")]
    pub country: Option<String>,
}

/// XMP 信息
#[derive(Debug, Clone, Deserialize)]
pub struct XmpInfo {
    /// 创建日期
    #[serde(rename = "CreateDate")]
    pub create_date: Option<String>,

    /// 修改日期
    #[serde(rename = "ModifyDate")]
    pub modify_date: Option<String>,

    /// 元数据日期
    #[serde(rename = "MetadataDate")]
    pub metadata_date: Option<String>,

    /// 创建工具
    #[serde(rename = "CreatorTool")]
    pub creator_tool: Option<String>,

    /// 评级
    #[serde(rename = "Rating")]
    pub rating: Option<i32>,

    /// 标签
    #[serde(rename = "Label")]
    pub label: Option<String>,
}

/// 简化的元数据结构体（仅常用字段）
#[derive(Debug, Clone, Deserialize)]
pub struct SimpleMetadata {
    /// 文件名
    #[serde(rename = "FileName")]
    pub file_name: String,

    /// 文件大小
    #[serde(rename = "FileSize")]
    pub file_size: String,

    /// MIME 类型
    #[serde(rename = "MIMEType")]
    pub mime_type: String,

    /// 图像宽度
    #[serde(rename = "ImageWidth")]
    pub width: Option<u32>,

    /// 图像高度
    #[serde(rename = "ImageHeight")]
    pub height: Option<u32>,

    /// 制造商
    #[serde(rename = "Make")]
    pub make: Option<String>,

    /// 型号
    #[serde(rename = "Model")]
    pub model: Option<String>,

    /// 拍摄日期
    #[serde(rename = "DateTimeOriginal")]
    pub date_taken: Option<String>,

    /// GPS 纬度
    #[serde(rename = "GPSLatitude")]
    pub gps_latitude: Option<f64>,

    /// GPS 经度
    #[serde(rename = "GPSLongitude")]
    pub gps_longitude: Option<f64>,
}
