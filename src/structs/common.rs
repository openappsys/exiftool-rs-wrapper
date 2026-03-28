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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_simple_metadata_from_exiftool_json() {
        // 模拟 ExifTool -json 输出中单个文件的 JSON 对象
        let json_str = r#"{
            "SourceFile": "/tmp/test_photo.jpg",
            "FileName": "test_photo.jpg",
            "FileSize": "2.5 MB",
            "MIMEType": "image/jpeg",
            "ImageWidth": 4000,
            "ImageHeight": 3000,
            "Make": "Canon",
            "Model": "EOS R5",
            "DateTimeOriginal": "2026:01:15 10:30:00",
            "GPSLatitude": 39.9042,
            "GPSLongitude": 116.4074
        }"#;

        // 反序列化为 SimpleMetadata 结构体
        let meta: SimpleMetadata =
            serde_json::from_str(json_str).expect("反序列化 SimpleMetadata 失败");

        // 验证必填字段
        assert_eq!(meta.file_name, "test_photo.jpg");
        assert_eq!(meta.file_size, "2.5 MB");
        assert_eq!(meta.mime_type, "image/jpeg");

        // 验证可选字段
        assert_eq!(meta.width, Some(4000));
        assert_eq!(meta.height, Some(3000));
        assert_eq!(meta.make, Some("Canon".to_string()));
        assert_eq!(meta.model, Some("EOS R5".to_string()));
        assert_eq!(meta.date_taken, Some("2026:01:15 10:30:00".to_string()));
        assert_eq!(meta.gps_latitude, Some(39.9042));
        assert_eq!(meta.gps_longitude, Some(116.4074));
    }

    #[test]
    fn test_deserialize_metadata_with_groups() {
        // 模拟 ExifTool -json -g2 输出（分层结构）
        let json_str = r#"{
            "SourceFile": "/tmp/grouped.jpg",
            "File": {
                "FileName": "grouped.jpg",
                "FileSize": "1.2 MB",
                "MIMEType": "image/jpeg"
            },
            "EXIF": {
                "Make": "Nikon",
                "Model": "Z 9",
                "ImageWidth": 8256,
                "ImageHeight": 5504,
                "DateTimeOriginal": "2026:03:28 14:00:00",
                "ISO": 200,
                "FNumber": 2.8,
                "ExposureTime": 0.004,
                "FocalLength": 50.0
            }
        }"#;

        // 反序列化为分层 Metadata 结构体
        let meta: Metadata = serde_json::from_str(json_str).expect("反序列化分层 Metadata 失败");

        // 验证源文件路径
        assert_eq!(meta.source_file, "/tmp/grouped.jpg");

        // 验证文件信息
        assert_eq!(meta.file.file_name, "grouped.jpg");
        assert_eq!(meta.file.file_size, "1.2 MB");
        assert_eq!(meta.file.mime_type, "image/jpeg");

        // 验证 EXIF 信息
        let exif = meta.exif.expect("EXIF 信息应存在");
        assert_eq!(exif.make, Some("Nikon".to_string()));
        assert_eq!(exif.model, Some("Z 9".to_string()));
        assert_eq!(exif.image_width, Some(8256));
        assert_eq!(exif.image_height, Some(5504));
        assert_eq!(exif.iso, Some(200));
        assert_eq!(exif.f_number, Some(2.8));

        // 验证可选分组未提供时为 None
        assert!(meta.gps.is_none());
        assert!(meta.iptc.is_none());
        assert!(meta.xmp.is_none());
    }
}
