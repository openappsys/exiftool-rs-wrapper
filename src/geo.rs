//! 地理信息模块
//!
//! 支持地理标记、反向地理编码等功能

use crate::ExifTool;
use crate::error::{Error, Result};
use crate::types::TagId;
use std::path::Path;

/// GPS 坐标
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpsCoordinate {
    /// 纬度（-90 到 90）
    pub latitude: f64,
    /// 经度（-180 到 180）
    pub longitude: f64,
    /// 海拔（米，可选）
    pub altitude: Option<f64>,
}

impl GpsCoordinate {
    /// 创建新的 GPS 坐标
    pub fn new(latitude: f64, longitude: f64) -> Result<Self> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(Error::invalid_arg("Latitude must be between -90 and 90"));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(Error::invalid_arg("Longitude must be between -180 and 180"));
        }

        Ok(Self {
            latitude,
            longitude,
            altitude: None,
        })
    }

    /// 设置海拔
    pub fn with_altitude(mut self, altitude: f64) -> Self {
        self.altitude = Some(altitude);
        self
    }

    /// 格式化为 ExifTool 格式
    pub fn format(&self) -> (String, String, Option<String>) {
        let lat_ref = if self.latitude >= 0.0 { "N" } else { "S" };
        let lon_ref = if self.longitude >= 0.0 { "E" } else { "W" };

        let lat_val = self.latitude.abs();
        let lon_val = self.longitude.abs();

        let lat_str = format!("{:.6}", lat_val);
        let lon_str = format!("{:.6}", lon_val);

        let alt_str = self.altitude.map(|a| format!("{:.2}", a));

        (
            format!("{} {}", lat_str, lat_ref),
            format!("{} {}", lon_str, lon_ref),
            alt_str,
        )
    }
}

/// 地理信息操作 trait
pub trait GeoOperations {
    /// 获取文件的 GPS 坐标
    fn get_gps<P: AsRef<Path>>(&self, path: P) -> Result<Option<GpsCoordinate>>;

    /// 设置文件的 GPS 坐标
    fn set_gps<P: AsRef<Path>>(&self, path: P, coord: &GpsCoordinate) -> Result<()>;

    /// 删除 GPS 信息
    fn remove_gps<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    /// 从 GPS 轨迹文件地理标记
    fn geotag_from_track<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        image: P,
        track_file: Q,
    ) -> Result<()>;
}

impl GeoOperations for ExifTool {
    fn get_gps<P: AsRef<Path>>(&self, path: P) -> Result<Option<GpsCoordinate>> {
        let metadata = self
            .query(path)
            .tag(TagId::GpsLatitude.name())
            .tag(TagId::GpsLongitude.name())
            .execute()?;

        let lat_val = metadata.get(TagId::GpsLatitude.name());
        let lon_val = metadata.get(TagId::GpsLongitude.name());

        if let (Some(lat), Some(lon)) = (lat_val, lon_val) {
            let lat_str = lat.to_string_lossy();
            let lon_str = lon.to_string_lossy();

            // 尝试解析坐标值：先尝试纯数字格式，再尝试度分秒（DMS）格式
            let lat_f = parse_gps_value(&lat_str);
            let lon_f = parse_gps_value(&lon_str);

            if let (Some(lat_v), Some(lon_v)) = (lat_f, lon_f) {
                let coord = GpsCoordinate::new(lat_v, lon_v)?;
                return Ok(Some(coord));
            }
        }

        Ok(None)
    }

    fn set_gps<P: AsRef<Path>>(&self, path: P, coord: &GpsCoordinate) -> Result<()> {
        let (lat, lon, alt) = coord.format();

        let mut write = self.write(path);

        write = write
            .tag(TagId::GpsLatitude.name(), &lat)
            .tag(TagId::GpsLongitude.name(), &lon);

        if let Some(altitude) = alt {
            write = write.tag(TagId::GpsAltitude.name(), altitude);
        }

        write.overwrite_original(true).execute()?;

        Ok(())
    }

    fn remove_gps<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // 删除所有 GPS 相关标签
        self.write(path)
            .delete(TagId::GpsLatitude.name())
            .delete(TagId::GpsLongitude.name())
            .delete(TagId::GpsAltitude.name())
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }

    fn geotag_from_track<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        image: P,
        track_file: Q,
    ) -> Result<()> {
        // 在 stay_open 模式下，选项名和值必须分开为两个独立参数
        self.write(image)
            .arg("-geotag")
            .arg(track_file.as_ref().to_string_lossy().to_string())
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }
}

/// 解析 GPS 坐标值，支持纯数字格式和度分秒（DMS）格式
///
/// 支持的格式示例：
/// - 纯数字：`"39.9042"`, `"-116.4074"`
/// - 度分秒：`"54 deg 59' 22.80\" N"`, `"1 deg 54' 57.60\" W"`
/// - 带符号的度分秒：`"54 deg 59' 22.80\""` （无方向后缀时保留原始正负）
fn parse_gps_value(s: &str) -> Option<f64> {
    let trimmed = s.trim();

    // 首先尝试直接解析为浮点数（纯数字格式）
    if let Ok(v) = trimmed.parse::<f64>() {
        return Some(v);
    }

    // 尝试解析度分秒（DMS）格式
    // 典型格式：`54 deg 59' 22.80" N` 或 `1 deg 54' 57.60" W`
    parse_dms(trimmed)
}

/// 解析度分秒（DMS）格式的 GPS 坐标字符串
///
/// 支持的格式：
/// - `54 deg 59' 22.80" N`
/// - `1 deg 54' 57.60" W`
/// - `39 deg 54' 15.12"`（无方向后缀）
fn parse_dms(s: &str) -> Option<f64> {
    // 判断方向后缀（N/S/E/W），确定正负号
    let upper = s.to_uppercase();
    let direction = if upper.ends_with('N') || upper.ends_with('E') {
        Some(1.0)
    } else if upper.ends_with('S') || upper.ends_with('W') {
        Some(-1.0)
    } else {
        None
    };

    // 移除方向后缀字符，保留数字部分
    let cleaned = if direction.is_some() {
        s[..s.len() - 1].trim()
    } else {
        s.trim()
    };

    // 将常见的分隔符替换为空格，方便统一解析
    // 移除 `deg`、`°`、`'`、`"`、`′`、`″` 等符号
    let normalized = cleaned
        .replace("deg", " ")
        .replace(['\u{00B0}', '\'', '"', '\u{2032}', '\u{2033}'], " ");

    // 将连续空格分割为数字 token
    let parts: Vec<&str> = normalized.split_whitespace().collect();

    if parts.is_empty() || parts.len() > 3 {
        return None;
    }

    // 解析度
    let degrees: f64 = parts.first()?.parse().ok()?;
    // 解析分（可选）
    let minutes: f64 = if parts.len() >= 2 {
        parts[1].parse().ok()?
    } else {
        0.0
    };
    // 解析秒（可选）
    let seconds: f64 = if parts.len() >= 3 {
        parts[2].parse().ok()?
    } else {
        0.0
    };

    // 计算十进制度数
    let decimal = degrees.abs() + minutes / 60.0 + seconds / 3600.0;

    // 应用方向符号
    let sign = direction.unwrap_or(if degrees < 0.0 { -1.0 } else { 1.0 });

    Some(decimal * sign)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExifTool;
    use crate::error::Error;

    /// 最小有效 JPEG 文件字节数组，用于创建临时测试文件
    const TINY_JPEG: &[u8] = &[
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06,
        0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B,
        0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20,
        0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31,
        0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF,
        0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00,
        0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x09, 0xFF, 0xC4, 0x00, 0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01,
        0x00, 0x00, 0x3F, 0x00, 0xD2, 0xCF, 0x20, 0xFF, 0xD9,
    ];

    #[test]
    fn test_gps_coordinate() {
        let coord = GpsCoordinate::new(39.9042, 116.4074).unwrap();
        assert_eq!(coord.latitude, 39.9042);
        assert_eq!(coord.longitude, 116.4074);
        assert_eq!(coord.altitude, None);

        let coord = coord.with_altitude(50.0);
        assert_eq!(coord.altitude, Some(50.0));
    }

    #[test]
    fn test_gps_coordinate_format() {
        let coord = GpsCoordinate::new(39.9042, 116.4074).unwrap();
        let (lat, lon, alt) = coord.format();

        assert!(lat.contains("N"));
        assert!(lon.contains("E"));
        assert_eq!(alt, None);
    }

    #[test]
    fn test_gps_coordinate_validation() {
        // 无效的纬度
        assert!(GpsCoordinate::new(100.0, 0.0).is_err());
        // 无效的经度
        assert!(GpsCoordinate::new(0.0, 200.0).is_err());
        // 有效坐标
        assert!(GpsCoordinate::new(0.0, 0.0).is_ok());
    }

    #[test]
    fn test_parse_gps_decimal() {
        // 纯数字格式
        assert_eq!(parse_gps_value("39.9042"), Some(39.9042));
        assert_eq!(parse_gps_value("-116.4074"), Some(-116.4074));
        assert_eq!(parse_gps_value("0.0"), Some(0.0));
    }

    #[test]
    fn test_parse_gps_dms() {
        // 度分秒格式：54 deg 59' 22.80" N
        let result = parse_gps_value("54 deg 59' 22.80\" N").unwrap();
        // 54 + 59/60 + 22.80/3600 = 54.989666...
        assert!((result - 54.9896667).abs() < 0.0001);

        // 度分秒格式：1 deg 54' 57.60" W（西经，结果为负）
        let result = parse_gps_value("1 deg 54' 57.60\" W").unwrap();
        assert!((result - (-1.916)).abs() < 0.001);

        // 南纬
        let result = parse_gps_value("33 deg 51' 54.00\" S").unwrap();
        assert!(result < 0.0);
        assert!((result - (-33.865)).abs() < 0.001);
    }

    #[test]
    fn test_set_and_get_gps_roundtrip() {
        // 检查 ExifTool 是否可用，不可用则跳过
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时出现意外错误: {:?}", e),
        };

        // 创建临时 JPEG 文件
        let tmp_dir = tempfile::tempdir().expect("创建临时目录失败");
        let test_file = tmp_dir.path().join("gps_roundtrip.jpg");
        std::fs::write(&test_file, TINY_JPEG).expect("写入临时 JPEG 文件失败");

        // 写入 GPS 坐标（北京天安门广场附近）
        let coord = GpsCoordinate::new(39.9042, 116.4074)
            .expect("创建 GPS 坐标失败")
            .with_altitude(50.0);
        et.set_gps(&test_file, &coord).expect("写入 GPS 坐标失败");

        // 读回 GPS 坐标并验证
        let read_coord = et
            .get_gps(&test_file)
            .expect("读取 GPS 坐标失败")
            .expect("GPS 坐标应存在，但返回了 None");

        // 验证纬度误差在合理范围内（ExifTool 可能有精度损失）
        assert!(
            (read_coord.latitude - 39.9042).abs() < 0.01,
            "纬度应接近 39.9042，实际为: {}",
            read_coord.latitude
        );
        assert!(
            (read_coord.longitude - 116.4074).abs() < 0.01,
            "经度应接近 116.4074，实际为: {}",
            read_coord.longitude
        );
    }

    #[test]
    fn test_remove_gps_clears_coordinates() {
        // 检查 ExifTool 是否可用，不可用则跳过
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时出现意外错误: {:?}", e),
        };

        // 创建临时 JPEG 文件
        let tmp_dir = tempfile::tempdir().expect("创建临时目录失败");
        let test_file = tmp_dir.path().join("gps_remove.jpg");
        std::fs::write(&test_file, TINY_JPEG).expect("写入临时 JPEG 文件失败");

        // 先写入 GPS 坐标
        let coord = GpsCoordinate::new(51.5074, -0.1278).expect("创建 GPS 坐标失败（伦敦坐标）");
        et.set_gps(&test_file, &coord).expect("写入 GPS 坐标失败");

        // 确认 GPS 坐标已写入
        let before = et.get_gps(&test_file).expect("写入后读取 GPS 坐标失败");
        assert!(before.is_some(), "写入 GPS 后应能读取到坐标");

        // 删除 GPS 信息
        et.remove_gps(&test_file).expect("删除 GPS 坐标失败");

        // 验证 GPS 坐标已被删除
        let after = et.get_gps(&test_file).expect("删除后读取 GPS 坐标失败");
        assert!(
            after.is_none(),
            "删除 GPS 后应返回 None，但实际返回了: {:?}",
            after
        );
    }
}
