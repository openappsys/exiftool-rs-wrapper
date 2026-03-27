//! Serde 结构体使用示例
//!
//! 本示例展示如何使用 serde 结构体来反序列化元数据。
//!
//! 需要在 Cargo.toml 中启用 "serde-structs" feature：
//!
//! ```toml
//! [dependencies]
//! exiftool-rs-wrapper = { version = "0.1.4", features = ["serde-structs"] }
//! ```
//!
//! 运行方式:
//! ```bash
//! cargo run --example serde_example --features serde-structs
//! ```

use exiftool_rs_wrapper::structs::{Metadata, SimpleMetadata};
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 ExifTool 实例
    let exiftool = ExifTool::new()?;

    // 示例 1: 使用完整 Metadata 结构体
    println!("=== 示例 1: 完整 Metadata 结构体 ===");
    match exiftool.read_struct::<Metadata, _>("data/image.jpg") {
        Ok(meta) => {
            println!("文件: {}", meta.file.file_name);
            println!("大小: {}", meta.file.file_size);
            println!("MIME: {}", meta.file.mime_type);

            if let Some(exif) = meta.exif {
                println!("\nEXIF 信息:");
                if let Some(make) = exif.make {
                    println!("  制造商: {}", make);
                }
                if let Some(model) = exif.model {
                    println!("  型号: {}", model);
                }
                if let Some(width) = exif.image_width {
                    println!("  宽度: {}", width);
                }
                if let Some(height) = exif.image_height {
                    println!("  高度: {}", height);
                }
                if let Some(date) = exif.date_time_original {
                    println!("  拍摄时间: {}", date);
                }
            }

            if let Some(gps) = meta.gps {
                println!("\nGPS 信息:");
                if let Some(lat) = gps.latitude {
                    println!("  纬度: {}", lat);
                }
                if let Some(lon) = gps.longitude {
                    println!("  经度: {}", lon);
                }
                if let Some(alt) = gps.altitude {
                    println!("  海拔: {}", alt);
                }
            }
        }
        Err(e) => eprintln!("读取失败: {}", e),
    }

    // 示例 2: 使用简化 SimpleMetadata
    println!("\n=== 示例 2: 简化结构体 ===");
    match exiftool.read_struct::<SimpleMetadata, _>("data/image.jpg") {
        Ok(meta) => {
            println!("文件: {}", meta.file_name);
            println!("大小: {}", meta.file_size);
            println!("MIME: {}", meta.mime_type);
            if let Some(width) = meta.width {
                println!("宽度: {}", width);
            }
            if let Some(height) = meta.height {
                println!("高度: {}", height);
            }
            if let Some(ref make) = meta.make {
                println!("制造商: {}", make);
            }
            if let Some(ref model) = meta.model {
                println!("型号: {}", model);
            }
            if let Some(ref date) = meta.date_taken {
                println!("拍摄时间: {}", date);
            }
            if let Some(lat) = meta.gps_latitude {
                println!("纬度: {}", lat);
            }
            if let Some(lon) = meta.gps_longitude {
                println!("经度: {}", lon);
            }
        }
        Err(e) => eprintln!("读取失败: {}", e),
    }

    // 示例 3: Builder 模式指定 exiftool 路径
    println!("\n=== 示例 3: Builder 模式 ===");
    let exiftool2 = exiftool_rs_wrapper::ExifTool::builder()
        .executable("exiftool") // 可指定自定义路径
        .build()?;

    match exiftool2.read_struct::<SimpleMetadata, _>("data/image.jpg") {
        Ok(meta) => {
            println!("使用 Builder 模式成功读取文件: {}", meta.file_name);
        }
        Err(e) => eprintln!("读取失败: {}", e),
    }

    Ok(())
}
