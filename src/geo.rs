//! 地理信息模块
//!
//! 支持地理标记、反向地理编码等功能

use crate::error::{Error, Result};
use crate::types::TagId;
use crate::ExifTool;
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

/// 地理编码结果
#[derive(Debug, Clone)]
pub struct GeocodeResult {
    /// 城市
    pub city: Option<String>,
    /// 区域/州
    pub region: Option<String>,
    /// 国家
    pub country: Option<String>,
    /// 国家代码
    pub country_code: Option<String>,
    /// 完整地址
    pub address: Option<String>,
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

    /// 生成 GPS 轨迹文件
    fn generate_tracklog<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        images: &[P],
        output: Q,
    ) -> Result<()>;

    /// 反向地理编码
    fn reverse_geocode<P: AsRef<Path>>(&self, coord: &GpsCoordinate) -> Result<GeocodeResult>;
}

impl GeoOperations for ExifTool {
    fn get_gps<P: AsRef<Path>>(&self, path: P) -> Result<Option<GpsCoordinate>> {
        let metadata = self
            .query(path)
            .tag(TagId::GPS_LATITUDE.name())
            .tag(TagId::GPS_LONGITUDE.name())
            .execute()?;

        let lat_val = metadata.get(TagId::GPS_LATITUDE.name());
        let lon_val = metadata.get(TagId::GPS_LONGITUDE.name());

        if let (Some(lat), Some(lon)) = (lat_val, lon_val) {
            let lat_str = lat.to_string_lossy();
            let lon_str = lon.to_string_lossy();

            // 解析坐标值（简化版，实际需要更复杂的解析）
            if let (Ok(lat_f), Ok(lon_f)) = (lat_str.parse::<f64>(), lon_str.parse::<f64>()) {
                let coord = GpsCoordinate::new(lat_f, lon_f)?;
                return Ok(Some(coord));
            }
        }

        Ok(None)
    }

    fn set_gps<P: AsRef<Path>>(&self, path: P, coord: &GpsCoordinate) -> Result<()> {
        let (lat, lon, alt) = coord.format();

        let mut write = self.write(path);

        write = write
            .tag(TagId::GPS_LATITUDE.name(), &lat)
            .tag(TagId::GPS_LONGITUDE.name(), &lon);

        if let Some(altitude) = alt {
            write = write.tag(TagId::GPS_ALTITUDE.name(), altitude);
        }

        write.overwrite_original(true).execute()?;

        Ok(())
    }

    fn remove_gps<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // 删除所有 GPS 相关标签
        self.write(path)
            .delete(TagId::GPS_LATITUDE.name())
            .delete(TagId::GPS_LONGITUDE.name())
            .delete(TagId::GPS_ALTITUDE.name())
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }

    fn geotag_from_track<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        image: P,
        track_file: Q,
    ) -> Result<()> {
        self.write(image)
            .arg(format!("-geotag {}", track_file.as_ref().display()))
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }

    fn generate_tracklog<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        images: &[P],
        output: Q,
    ) -> Result<()> {
        // 收集所有文件的 GPS 数据
        let mut track_points = Vec::new();

        for image in images {
            if let Some(coord) = self.get_gps(image)? {
                let timestamp = self
                    .read_tag::<String, _, _>(image, TagId::DATE_TIME_ORIGINAL.name())
                    .ok();

                track_points.push((coord, timestamp));
            }
        }

        // 生成 GPX 格式的轨迹文件
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(output)?;
        writeln!(file, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(file, "<gpx version=\"1.1\">")?;
        writeln!(file, "  <trk><trkseg>")?;

        for (coord, _timestamp) in track_points {
            writeln!(
                file,
                "    <trkpt lat=\"{}\" lon=\"{}\">",
                coord.latitude, coord.longitude
            )?;
            if let Some(alt) = coord.altitude {
                writeln!(file, "      <ele>{}</ele>", alt)?;
            }
            writeln!(file, "    </trkpt>")?;
        }

        writeln!(file, "  </trkseg></trk>")?;
        writeln!(file, "</gpx>")?;

        Ok(())
    }

    fn reverse_geocode<P: AsRef<Path>>(&self, coord: &GpsCoordinate) -> Result<GeocodeResult> {
        // 使用 ExifTool 的 -geolocation 功能
        // 需要提供地理编码数据库或调用外部服务

        // 创建临时文件存储坐标
        let mut temp_file = std::env::temp_dir();
        temp_file.push(format!("geocode_{}.txt", std::process::id()));

        // 写入坐标到临时文件
        let coord_str = format!("{:.6},{:.6}", coord.latitude, coord.longitude);
        std::fs::write(&temp_file, &coord_str).map_err(Error::Io)?;

        // 使用 exiftool -geolocation 选项
        // 注意：这需要系统安装了地理编码数据库
        let args = vec![
            "-geolocation".to_string(),
            temp_file.to_string_lossy().to_string(),
        ];

        let response = self.execute_raw(&args)?;

        // 清理临时文件
        let _ = std::fs::remove_file(&temp_file);

        // 解析响应
        let output = response.text();
        parse_geocode_result(&output)
    }
}

/// 解析地理编码结果
fn parse_geocode_result(output: &str) -> Result<GeocodeResult> {
    let mut result = GeocodeResult {
        city: None,
        region: None,
        country: None,
        country_code: None,
        address: None,
    };

    for line in output.lines() {
        let line = line.trim();
        if line.starts_with("City") {
            result.city = extract_value(line);
        } else if line.starts_with("Region") {
            result.region = extract_value(line);
        } else if line.starts_with("Country") {
            result.country = extract_value(line);
        } else if line.starts_with("Country Code") {
            result.country_code = extract_value(line);
        } else if line.starts_with("Address") {
            result.address = extract_value(line);
        }
    }

    Ok(result)
}

/// 从行中提取值
fn extract_value(line: &str) -> Option<String> {
    line.splitn(2, ':')
        .nth(1)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
