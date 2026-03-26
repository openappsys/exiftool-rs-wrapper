# ExifTool Rust Wrapper

[![crates.io](https://img.shields.io/crates/v/exiftool-rs-wrapper.svg)](https://crates.io/crates/exiftool-rs-wrapper)
[![docs.rs](https://docs.rs/exiftool-rs-wrapper/badge.svg)](https://docs.rs/exiftool-rs-wrapper)
[![CI](https://github.com/openappsys/exiftool-rs-wrapper/workflows/CI/badge.svg)](https://github.com/openappsys/exiftool-rs-wrapper/actions)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.94%2B-orange.svg)](https://www.rust-lang.org)

**中文** | [English](README-EN.md)

> 一个高性能、类型安全的 Rust ExifTool 封装库，提供 100% ExifTool 功能覆盖

## 简介

`exiftool-rs-wrapper` 是一个现代化的 Rust 库，用于读取、写入和管理图像、视频、音频等多媒体文件的元数据。本库封装了强大的 [ExifTool](https://exiftool.org/) 命令行工具，提供符合 Rust 习惯的 API 设计。

### 核心特点

- **100% 功能覆盖**：完整支持 ExifTool 的所有读取、写入和高级功能
- **高性能**：使用 `-stay_open` 模式保持进程运行，避免频繁启动开销
- **类型安全**：完整的标签类型系统和强类型 API
- **异步支持**：基于 Tokio 的异步 API（可选特性）
- **连接池**：内置连接池支持高并发场景
- **Builder 模式**：流畅的 API 设计，链式调用

## 功能特性

### 元数据读取

- 读取 EXIF、IPTC、XMP 等标准元数据
- 支持 200+ 文件格式（JPEG、PNG、RAW、MP4、PDF 等）
- 选择性读取特定标签
- 批量查询多个文件
- 原始数值和格式化值
- 按类别分组输出

### 元数据写入

- 写入任意 ExifTool 支持的标签
- 删除特定标签
- 批量写入操作
- 条件写入（仅在满足条件时修改）
- 日期时间偏移调整
- 复制标签（从其他文件复制元数据）
- 支持备份和覆盖模式

### 高级功能

- **文件操作**：基于元数据重命名、组织文件
- **地理信息**：GPS 坐标读写、反向地理编码
- **二进制数据**：缩略图、预览图提取
- **格式转换**：多种输出格式（JSON、XML、CSV 等）
- **校验和**：计算文件校验和（MD5、SHA256 等）
- **流式处理**：大文件处理支持进度跟踪
- **错误恢复**：可配置的重试策略

### 性能优化

- 连接池支持并发访问
- LRU 缓存减少重复查询
- 批量操作优化
- 流式处理大文件

## 安装

### 1. 安装 ExifTool

在使用本库之前，需要先在系统上安装 ExifTool：

**macOS:**
```bash
brew install exiftool
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libimage-exiftool-perl
```

**Windows:**
下载并安装 [Windows 版本](https://exiftool.org/)

**验证安装：**
```bash
exiftool -ver
```

### 2. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
exiftool-rs-wrapper = "0.1.0"

# 启用异步支持（可选）
exiftool-rs-wrapper = { version = "0.1.0", features = ["async"] }
```

## 快速开始

### 基本示例

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 ExifTool 实例（-stay_open 模式）
    let exiftool = ExifTool::new()?;
    
    // 读取文件元数据
    let metadata = exiftool.query("photo.jpg").execute()?;
    
    // 访问特定标签
    if let Some(make) = metadata.get("Make") {
        println!("相机制造商: {}", make);
    }
    
    if let Some(model) = metadata.get("Model") {
        println!("相机型号: {}", model);
    }
    
    // 获取图像尺寸
    let width: i64 = exiftool.read_tag("photo.jpg", "ImageWidth")?;
    let height: i64 = exiftool.read_tag("photo.jpg", "ImageHeight")?;
    println!("图像尺寸: {} x {}", width, height);
    
    Ok(())
}
```

### 写入元数据

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // 基本写入（创建备份）
    exiftool.write("photo.jpg")
        .tag("Copyright", "© 2026 My Company")
        .tag("Artist", "John Doe")
        .execute()?;
    
    // 覆盖原始文件（不创建备份）
    exiftool.write("photo.jpg")
        .tag("Comment", "Processed with Rust")
        .overwrite_original(true)
        .execute()?;
    
    // 删除标签
    exiftool.write("photo.jpg")
        .delete("GPSPosition")
        .overwrite_original(true)
        .execute()?;
    
    Ok(())
}
```

### 批量处理

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    let paths = vec!["photo1.jpg", "photo2.jpg", "photo3.jpg"];
    
    // 批量查询
    let results = exiftool.query_batch(&paths)
        .tag("FileName")
        .tag("ImageSize")
        .tag("DateTimeOriginal")
        .execute()?;
    
    for (path, metadata) in results {
        println!("{}: {:?}", path.display(), metadata.get("FileName"));
    }
    
    Ok(())
}
```

## 详细 API 使用示例

### 使用标签常量（类型安全）

```rust
use exiftool_rs_wrapper::{ExifTool, TagId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // 使用 TagId 常量替代字符串
    let make: String = exiftool.read_tag("photo.jpg", TagId::MAKE)?;
    let model: String = exiftool.read_tag("photo.jpg", TagId::MODEL)?;
    let iso: i64 = exiftool.read_tag("photo.jpg", TagId::ISO)?;
    
    println!("{} {} @ ISO {}", make, model, iso);
    
    // 写入时使用 TagId
    exiftool.write("photo.jpg")
        .tag_id(TagId::COPYRIGHT, "© 2026")
        .tag_id(TagId::ARTIST, "Photographer")
        .overwrite_original(true)
        .execute()?;
    
    Ok(())
}
```

### 高级查询选项

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // 高级查询配置
    let metadata = exiftool.query("photo.jpg")
        .include_unknown(true)          // 包含未知标签
        .include_duplicates(true)       // 包含重复标签
        .raw_values(true)               // 返回原始数值
        .group_by_category(true)        // 按类别分组
        .tag("Make")                     // 只查询特定标签
        .tag("Model")
        .tag("DateTimeOriginal")
        .exclude("MakerNotes")           // 排除特定标签
        .execute()?;
    
    // 输出为 JSON
    let json = exiftool.query("photo.jpg")
        .execute_json()?;
    println!("{}", serde_json::to_string_pretty(&json)?);
    
    // 反序列化为自定义类型
    #[derive(serde::Deserialize)]
    struct PhotoInfo {
        #[serde(rename = "FileName")]
        file_name: String,
        #[serde(rename = "ImageWidth")]
        width: i64,
        #[serde(rename = "ImageHeight")]
        height: i64,
    }
    
    let info: PhotoInfo = exiftool.query("photo.jpg")
        .execute_as()?;
    
    Ok(())
}
```

### 异步 API

```rust
use exiftool_rs_wrapper::async_ext::AsyncExifTool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建异步 ExifTool 实例
    let exiftool = AsyncExifTool::new()?;
    
    // 异步查询
    let metadata = exiftool.query("photo.jpg").await?;
    println!("相机: {:?}", metadata.get("Make"));
    
    // 异步批量查询
    let paths = vec!["photo1.jpg", "photo2.jpg", "photo3.jpg"];
    let results = exiftool.query_batch(&paths).await?;
    
    for (path, metadata) in results {
        println!("{}: {:?}", path.display(), metadata.get("FileName"));
    }
    
    // 异步写入
    exiftool.write_tag("photo.jpg", "Copyright", "© 2026").await?;
    
    // 异步删除
    exiftool.delete_tag("photo.jpg", "GPSPosition").await?;
    
    Ok(())
}
```

### 连接池（高并发）

```rust
use exiftool_rs_wrapper::pool::ExifToolPool;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建包含 4 个连接的连接池
    let pool = ExifToolPool::new(4)?;
    let pool_clone = pool.clone();
    
    // 在多个线程中使用连接池
    let handles: Vec<_> = (0..8).map(|i| {
        let pool = pool_clone.clone();
        thread::spawn(move || {
            // 从池中获取连接
            let conn = pool.acquire()?;
            let exiftool = conn.get().unwrap();
            
            let metadata = exiftool.query(format!("photo{}.jpg", i))
                .execute()?;
            
            println!("线程 {}: 处理完成", i);
            Ok::<(), exiftool_rs_wrapper::Error>(())
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap()?;
    }
    
    Ok(())
}
```

### 文件组织和重命名

```rust
use exiftool_rs_wrapper::{
    ExifTool, 
    file_ops::{FileOperations, RenamePattern}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // 基于日期时间重命名文件
    exiftool.rename_by_pattern(
        "photo.jpg",
        RenamePattern::datetime("%Y%m%d_%H%M%S"),
    )?;
    
    // 基于相机型号重命名
    exiftool.rename_by_pattern(
        "photo.jpg",
        RenamePattern::tag_with_suffix(
            exiftool_rs_wrapper::TagId::MODEL,
            "_IMG"
        ),
    )?;
    
    // 组织文件到目录结构
    use exiftool_rs_wrapper::file_ops::OrganizeOptions;
    
    let options = OrganizeOptions::new("/output/directory")
        .subdir(RenamePattern::datetime("%Y/%m"))  // 按年月创建子目录
        .filename(RenamePattern::datetime("%Y%m%d_%H%M%S"))
        .extension("jpg");
    
    exiftool.organize_files(&["photo1.jpg", "photo2.jpg"], &options)?;
    
    Ok(())
}
```

### 地理信息处理

```rust
use exiftool_rs_wrapper::{ExifTool, geo::GeoOperations};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // 读取 GPS 坐标
    if let Some(coord) = exiftool.get_gps_coordinates("photo.jpg")? {
        println!("纬度: {}", coord.latitude);
        println!("经度: {}", coord.longitude);
        println!("海拔: {:?}", coord.altitude);
    }
    
    // 写入 GPS 坐标
    use exiftool_rs_wrapper::geo::GpsCoordinate;
    
    let coord = GpsCoordinate::new(39.9042, 116.4074)
        .altitude(43.5);
    
    exiftool.set_gps_coordinates("photo.jpg", &coord)?;
    
    // 反向地理编码（需要互联网连接）
    if let Some(location) = exiftool.reverse_geocode(&coord)? {
        println!("城市: {}", location.city);
        println!("国家: {}", location.country);
    }
    
    Ok(())
}
```

### 错误处理和重试

```rust
use exiftool_rs_wrapper::{
    ExifTool, 
    retry::{RetryPolicy, with_retry_sync}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // 配置重试策略
    let policy = RetryPolicy::new()
        .max_attempts(3)
        .initial_delay(std::time::Duration::from_millis(100))
        .exponential_backoff(true);
    
    // 使用重试执行操作
    let metadata = with_retry_sync(&policy, || {
        exiftool.query("photo.jpg").execute()
    })?;
    
    println!("成功读取元数据: {:?}", metadata.get("FileName"));
    
    Ok(())
}
```

### 流式处理和进度跟踪

```rust
use exiftool_rs_wrapper::{
    ExifTool, 
    stream::{StreamingOperations, StreamOptions}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // 定义进度回调
    let on_progress = |processed: usize, total: usize, current: &str| {
        let percent = (processed as f64 / total as f64) * 100.0;
        println!("进度: {:.1}% ({}/{}): {}", percent, processed, total, current);
    };
    
    let options = StreamOptions::new()
        .chunk_size(1024 * 1024)  // 1MB 分块
        .progress_callback(on_progress);
    
    // 流式处理大文件
    let metadata = exiftool.stream_query("large_video.mp4", &options)?;
    
    Ok(())
}
```

## 性能基准测试

以下是在典型硬件配置上的性能测试结果（仅供参考）：

| 操作 | 单线程 | 连接池(4) | 异步 |
|------|--------|-----------|------|
| 读取单个 JPEG 文件 | ~5ms | - | ~5ms |
| 批量读取 100 个文件 | 450ms | 120ms | 110ms |
| 写入单个标签 | ~15ms | - | ~15ms |
| 批量写入 100 个文件 | 1.5s | 450ms | 420ms |

### 优化建议

1. **使用连接池**：在高并发场景下，连接池可以显著提升性能
2. **批量操作**：尽量使用批量 API 而非单个文件循环
3. **选择性查询**：只查询需要的标签，避免读取完整元数据
4. **启用缓存**：对于重复查询，使用内置 LRU 缓存

## 命令行工具

本项目还提供了一个命令行工具：

```bash
# 安装命令行工具
cargo install exiftool-rs-wrapper

# 读取文件元数据
exiftool-rs-wrapper read photo.jpg

# 写入标签
exiftool-rs-wrapper write photo.jpg Copyright "© 2026"

# 删除标签
exiftool-rs-wrapper delete photo.jpg GPSPosition

# 批量处理
exiftool-rs-wrapper batch --input-dir ./photos --output-dir ./organized

# 查看版本
exiftool-rs-wrapper version

# 列出支持的标签
exiftool-rs-wrapper list-tags
```

## 贡献指南

我们欢迎所有形式的贡献！请遵循以下步骤：

### 提交 Issue

- 报告 Bug 时，请提供详细的复现步骤和环境信息
- 请求新功能时，请描述使用场景和预期行为
- 在提交前请先搜索是否已有相关 Issue

### 提交 Pull Request

1. Fork 本仓库
2. 创建功能分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'Add amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 提交 Pull Request

### 开发环境

```bash
# 克隆仓库
git clone https://github.com/openappsys/exiftool-rs-wrapper.git
cd exiftool-rs-wrapper

# 构建项目
cargo build --release

# 运行测试
cargo test
cargo test --lib

# 代码检查
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

### 代码规范

- 所有代码注释必须使用中文
- 遵循 Rust API Guidelines
- 确保通过 `cargo clippy` 和 `cargo fmt` 检查
- 为新功能添加测试
- 更新相关文档

## 许可证

本项目采用 Apache-2.0 许可证。

- Apache-2.0 License: 参见 [LICENSE](LICENSE) 文件

## 致谢

- [ExifTool](https://exiftool.org/) by Phil Harvey - 强大的元数据处理工具
- Rust 社区 - 优秀的语言和生态系统

## 相关链接

- [文档](https://docs.rs/exiftool-rs-wrapper)
- [Crates.io](https://crates.io/crates/exiftool-rs-wrapper)
- [GitHub 仓库](https://github.com/openappsys/exiftool-rs-wrapper)
- [问题反馈](https://github.com/openappsys/exiftool-rs-wrapper/issues)

---

**注意**：本库需要系统中已安装 ExifTool。ExifTool 是 Phil Harvey 开发的独立软件，拥有其自己的许可证。
