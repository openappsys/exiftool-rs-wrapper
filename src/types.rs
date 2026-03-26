//! 核心类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// 标签标识符 - 提供类型安全的标签访问
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(&'static str);

impl TagId {
    /// 创建新的标签标识符
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// 获取标签名称
    pub fn name(&self) -> &str {
        self.0
    }

    // === 常用 EXIF 标签 ===
    pub const MAKE: Self = Self("Make");
    pub const MODEL: Self = Self("Model");
    pub const DATE_TIME_ORIGINAL: Self = Self("DateTimeOriginal");
    pub const CREATE_DATE: Self = Self("CreateDate");
    pub const MODIFY_DATE: Self = Self("ModifyDate");
    pub const IMAGE_WIDTH: Self = Self("ImageWidth");
    pub const IMAGE_HEIGHT: Self = Self("ImageHeight");
    pub const ORIENTATION: Self = Self("Orientation");
    pub const X_RESOLUTION: Self = Self("XResolution");
    pub const Y_RESOLUTION: Self = Self("YResolution");
    pub const RESOLUTION_UNIT: Self = Self("ResolutionUnit");
    pub const SOFTWARE: Self = Self("Software");
    pub const COPYRIGHT: Self = Self("Copyright");
    pub const ARTIST: Self = Self("Artist");
    pub const IMAGE_DESCRIPTION: Self = Self("ImageDescription");

    // === 相机设置标签 ===
    pub const EXPOSURE_TIME: Self = Self("ExposureTime");
    pub const F_NUMBER: Self = Self("FNumber");
    pub const EXPOSURE_PROGRAM: Self = Self("ExposureProgram");
    pub const ISO: Self = Self("ISO");
    pub const SENSITIVITY_TYPE: Self = Self("SensitivityType");
    pub const RECOMMENDED_EXPOSURE_INDEX: Self = Self("RecommendedExposureIndex");
    pub const EXIF_VERSION: Self = Self("ExifVersion");
    pub const DATE_TIME_DIGITIZED: Self = Self("DateTimeDigitized");
    pub const COMPONENT_CONFIGURATION: Self = Self("ComponentConfiguration");
    pub const SHUTTER_SPEED_VALUE: Self = Self("ShutterSpeedValue");
    pub const APERTURE_VALUE: Self = Self("ApertureValue");
    pub const BRIGHTNESS_VALUE: Self = Self("BrightnessValue");
    pub const EXPOSURE_COMPENSATION: Self = Self("ExposureCompensation");
    pub const MAX_APERTURE_VALUE: Self = Self("MaxApertureValue");
    pub const SUBJECT_DISTANCE: Self = Self("SubjectDistance");
    pub const METERING_MODE: Self = Self("MeteringMode");
    pub const LIGHT_SOURCE: Self = Self("LightSource");
    pub const FLASH: Self = Self("Flash");
    pub const FOCAL_LENGTH: Self = Self("FocalLength");
    pub const FOCAL_LENGTH_IN_35MM_FORMAT: Self = Self("FocalLengthIn35mmFormat");
    pub const FLASH_ENERGY: Self = Self("FlashEnergy");
    pub const SPATIAL_FREQUENCY_RESPONSE: Self = Self("SpatialFrequencyResponse");
    pub const FOCAL_PLANE_X_RESOLUTION: Self = Self("FocalPlaneXResolution");
    pub const FOCAL_PLANE_Y_RESOLUTION: Self = Self("FocalPlaneYResolution");
    pub const FOCAL_PLANE_RESOLUTION_UNIT: Self = Self("FocalPlaneResolutionUnit");
    pub const SUBJECT_LOCATION: Self = Self("SubjectLocation");
    pub const EXPOSURE_INDEX: Self = Self("ExposureIndex");
    pub const SENSING_METHOD: Self = Self("SensingMethod");
    pub const FILE_SOURCE: Self = Self("FileSource");
    pub const SCENE_TYPE: Self = Self("SceneType");
    pub const CFA_PATTERN: Self = Self("CFAPattern");
    pub const CUSTOM_RENDERED: Self = Self("CustomRendered");
    pub const EXPOSURE_MODE: Self = Self("ExposureMode");
    pub const WHITE_BALANCE: Self = Self("WhiteBalance");
    pub const DIGITAL_ZOOM_RATIO: Self = Self("DigitalZoomRatio");
    pub const FOCAL_LENGTH_35EFL: Self = Self("FocalLength35efl");
    pub const SCENE_CAPTURE_TYPE: Self = Self("SceneCaptureType");
    pub const GAIN_CONTROL: Self = Self("GainControl");
    pub const CONTRAST: Self = Self("Contrast");
    pub const SATURATION: Self = Self("Saturation");
    pub const SHARPNESS: Self = Self("Sharpness");
    pub const DEVICE_SETTING_DESCRIPTION: Self = Self("DeviceSettingDescription");
    pub const SUBJECT_DISTANCE_RANGE: Self = Self("SubjectDistanceRange");

    // === GPS 标签 ===
    pub const GPS_LATITUDE_REF: Self = Self("GPSLatitudeRef");
    pub const GPS_LATITUDE: Self = Self("GPSLatitude");
    pub const GPS_LONGITUDE_REF: Self = Self("GPSLongitudeRef");
    pub const GPS_LONGITUDE: Self = Self("GPSLongitude");
    pub const GPS_ALTITUDE_REF: Self = Self("GPSAltitudeRef");
    pub const GPS_ALTITUDE: Self = Self("GPSAltitude");
    pub const GPS_TIMESTAMP: Self = Self("GPSTimeStamp");
    pub const GPS_SATELLITES: Self = Self("GPSSatellites");
    pub const GPS_STATUS: Self = Self("GPSStatus");
    pub const GPS_MEASURE_MODE: Self = Self("GPSMeasureMode");
    pub const GPS_DOP: Self = Self("GPSDOP");
    pub const GPS_SPEED_REF: Self = Self("GPSSpeedRef");
    pub const GPS_SPEED: Self = Self("GPSSpeed");
    pub const GPS_TRACK_REF: Self = Self("GPSTrackRef");
    pub const GPS_TRACK: Self = Self("GPSTrack");
    pub const GPS_IMG_DIRECTION_REF: Self = Self("GPSImgDirectionRef");
    pub const GPS_IMG_DIRECTION: Self = Self("GPSImgDirection");
    pub const GPS_MAP_DATUM: Self = Self("GPSMapDatum");
    pub const GPS_DEST_LATITUDE_REF: Self = Self("GPSDestLatitudeRef");
    pub const GPS_DEST_LATITUDE: Self = Self("GPSDestLatitude");
    pub const GPS_DEST_LONGITUDE_REF: Self = Self("GPSDestLongitudeRef");
    pub const GPS_DEST_LONGITUDE: Self = Self("GPSDestLongitude");
    pub const GPS_DEST_BEARING_REF: Self = Self("GPSDestBearingRef");
    pub const GPS_DEST_BEARING: Self = Self("GPSDestBearing");
    pub const GPS_DEST_DISTANCE_REF: Self = Self("GPSDestDistanceRef");
    pub const GPS_DEST_DISTANCE: Self = Self("GPSDestDistance");
    pub const GPS_PROCESSING_METHOD: Self = Self("GPSProcessingMethod");
    pub const GPS_AREA_INFORMATION: Self = Self("GPSAreaInformation");
    pub const GPS_DATE_STAMP: Self = Self("GPSDateStamp");
    pub const GPS_DIFFERENTIAL: Self = Self("GPSDifferential");
    pub const GPS_H_POSITIONING_ERROR: Self = Self("GPSHPositioningError");

    // === 文件信息标签 ===
    pub const FILE_NAME: Self = Self("FileName");
    pub const DIRECTORY: Self = Self("Directory");
    pub const FILE_SIZE: Self = Self("FileSize");
    pub const FILE_MODIFY_DATE: Self = Self("FileModifyDate");
    pub const FILE_ACCESS_DATE: Self = Self("FileAccessDate");
    pub const FILE_INODE_CHANGE_DATE: Self = Self("FileInodeChangeDate");
    pub const FILE_PERMISSIONS: Self = Self("FilePermissions");
    pub const FILE_TYPE: Self = Self("FileType");
    pub const FILE_TYPE_EXTENSION: Self = Self("FileTypeExtension");
    pub const MIME_TYPE: Self = Self("MIMEType");
    pub const EXIF_BYTE_ORDER: Self = Self("ExifByteOrder");
    pub const CURRENT_ICC_PROFILE: Self = Self("CurrentICCProfile");
    pub const PROFILE_DATE_TIME: Self = Self("ProfileDateTime");
    pub const PROFILE_FILE_SIGNATURE: Self = Self("ProfileFileSignature");
    pub const PRIMARY_PLATFORM: Self = Self("PrimaryPlatform");
    pub const CMM_TYPE: Self = Self("CMMType");
    pub const PROFILE_VERSION: Self = Self("ProfileVersion");
    pub const PROFILE_CLASS: Self = Self("ProfileClass");
    pub const COLOR_SPACE_DATA: Self = Self("ColorSpaceData");
    pub const PROFILE_CONNECTION_SPACE: Self = Self("ProfileConnectionSpace");
    pub const PROFILE_CONNECTION_SPACE_ILLUMINANT: Self = Self("ProfileConnectionSpaceIlluminant");
    pub const ICC_PROFILE_CREATOR: Self = Self("ICCProfileCreator");
    pub const ICC_PROFILE_DESCRIPTION: Self = Self("ICCProfileDescription");
    pub const ICC_VIEWING_CONDITIONS_DESCRIPTION: Self = Self("ICCViewingConditionsDescription");
    pub const ICC_DEVICE_MODEL: Self = Self("ICCDeviceModel");
    pub const ICC_DEVICE_MANUFACTURER: Self = Self("ICCDeviceManufacturer");
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for TagId {
    fn from(name: &'static str) -> Self {
        Self(name)
    }
}

/// 标签值类型 - 支持 ExifTool 返回的所有数据类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagValue {
    /// 字符串值
    String(String),

    /// 整数值
    Integer(i64),

    /// 浮点数值
    Float(f64),

    /// 布尔值
    Boolean(bool),

    /// 数组值
    Array(Vec<TagValue>),

    /// 二进制数据（Base64 编码）
    Binary(String),

    /// 空值
    Null,
}

impl TagValue {
    /// 尝试获取字符串值
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// 尝试获取整数值
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            Self::Float(f) => Some(*f as i64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// 尝试获取浮点数值
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Integer(i) => Some(*i as f64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// 尝试获取布尔值
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            Self::Integer(0) => Some(false),
            Self::Integer(_) => Some(true),
            Self::String(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" | "on" => Some(true),
                "false" | "no" | "0" | "off" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }

    /// 尝试获取数组
    pub fn as_array(&self) -> Option<&Vec<TagValue>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// 转换为字符串表示
    pub fn to_string_lossy(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Integer(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_lossy()).collect();
                format!("[{}]", items.join(", "))
            }
            Self::Binary(b) => format!("[binary: {} bytes]", b.len()),
            Self::Null => "null".to_string(),
        }
    }

    /// 检查是否为空
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl fmt::Display for TagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

/// 元数据结构
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// 顶层标签
    #[serde(flatten)]
    tags: HashMap<String, TagValue>,

    /// 分组标签（如 EXIF、IPTC、XMP 等）
    #[serde(skip)]
    groups: HashMap<String, Metadata>,
}

impl Metadata {
    /// 创建空的元数据
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取标签值
    pub fn get(&self, tag: &str) -> Option<&TagValue> {
        self.tags.get(tag)
    }

    /// 获取标签值（使用 TagId）
    pub fn get_tag(&self, tag: TagId) -> Option<&TagValue> {
        self.get(tag.name())
    }

    /// 设置标签值
    pub fn set(&mut self, tag: impl Into<String>, value: impl Into<TagValue>) {
        self.tags.insert(tag.into(), value.into());
    }

    /// 设置标签值（使用 TagId）
    pub fn set_tag(&mut self, tag: TagId, value: impl Into<TagValue>) {
        self.set(tag.name(), value);
    }

    /// 获取所有标签
    pub fn tags(&self) -> &HashMap<String, TagValue> {
        &self.tags
    }

    /// 获取所有标签（可变）
    pub fn tags_mut(&mut self) -> &mut HashMap<String, TagValue> {
        &mut self.tags
    }

    /// 获取分组
    pub fn group(&self, name: &str) -> Option<&Metadata> {
        self.groups.get(name)
    }

    /// 设置分组
    pub fn set_group(&mut self, name: impl Into<String>, metadata: Metadata) {
        self.groups.insert(name.into(), metadata);
    }

    /// 获取所有分组
    pub fn groups(&self) -> &HashMap<String, Metadata> {
        &self.groups
    }

    /// 检查是否包含标签
    pub fn contains(&self, tag: &str) -> bool {
        self.tags.contains_key(tag)
    }

    /// 检查是否包含标签（使用 TagId）
    pub fn contains_tag(&self, tag: TagId) -> bool {
        self.contains(tag.name())
    }

    /// 获取标签数量
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// 合并另一个元数据
    pub fn merge(&mut self, other: Metadata) {
        self.tags.extend(other.tags);
        self.groups.extend(other.groups);
    }

    /// 遍历所有标签
    pub fn iter(&self) -> impl Iterator<Item = (&String, &TagValue)> {
        self.tags.iter()
    }
}

impl IntoIterator for Metadata {
    type Item = (String, TagValue);
    type IntoIter = std::collections::hash_map::IntoIter<String, TagValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags.into_iter()
    }
}

impl<'a> IntoIterator for &'a Metadata {
    type Item = (&'a String, &'a TagValue);
    type IntoIter = std::collections::hash_map::Iter<'a, String, TagValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags.iter()
    }
}

// 类型转换实现
impl From<String> for TagValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for TagValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<i64> for TagValue {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<i32> for TagValue {
    fn from(i: i32) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<f64> for TagValue {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<f32> for TagValue {
    fn from(f: f32) -> Self {
        Self::Float(f as f64)
    }
}

impl From<bool> for TagValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<Vec<TagValue>> for TagValue {
    fn from(arr: Vec<TagValue>) -> Self {
        Self::Array(arr)
    }
}

impl<T: Into<TagValue>> From<Option<T>> for TagValue {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Self::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_id() {
        assert_eq!(TagId::MAKE.name(), "Make");
        assert_eq!(TagId::MODEL.name(), "Model");
    }

    #[test]
    fn test_tag_value_conversions() {
        let str_val: TagValue = "test".into();
        assert_eq!(str_val.as_string(), Some(&"test".to_string()));

        let int_val: TagValue = 42i64.into();
        assert_eq!(int_val.as_integer(), Some(42));

        let float_val: TagValue = std::f64::consts::PI.into();
        assert_eq!(float_val.as_float(), Some(std::f64::consts::PI));

        let bool_val: TagValue = true.into();
        assert_eq!(bool_val.as_bool(), Some(true));
    }

    #[test]
    fn test_metadata() {
        let mut meta = Metadata::new();
        meta.set("Make", "Canon");
        meta.set("Model", "EOS 5D");

        assert_eq!(meta.len(), 2);
        assert!(meta.contains("Make"));
        assert_eq!(
            meta.get("Make"),
            Some(&TagValue::String("Canon".to_string()))
        );
    }

    #[test]
    fn test_metadata_iteration() {
        let mut meta = Metadata::new();
        meta.set("A", 1);
        meta.set("B", 2);

        let mut count = 0;
        for (key, _value) in &meta {
            count += 1;
            assert!(key == "A" || key == "B");
        }
        assert_eq!(count, 2);
    }
}
