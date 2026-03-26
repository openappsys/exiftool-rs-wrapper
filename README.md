# ExifTool Rust Wrapper

一个用于 [ExifTool](https://exiftool.org/) 的 Rust 封装库，提供类型安全的 API 来读取和写入文件元数据。

## 功能特性

- **读取元数据**：支持读取各种文件格式的 EXIF、IPTC、XMP 等元数据
- **写入标签**：可以写入和修改元数据标签
- **删除标签**：支持删除特定的元数据标签
- **复制元数据**：可以在不同文件之间复制元数据
- **类型安全**：使用 Rust 类型系统确保 API 安全
- **错误处理**：完善的错误处理机制

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

### 2. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
exiftool-rs-wrapper = "0.1.0"
```

## 使用方法

### 基本示例

```rust
use exiftool_rs_wrapper::{ExifTool, ExifError};

fn main() -> Result<(), ExifError> {
    // 创建 ExifTool 实例
    let mut exiftool = ExifTool::new()?;
    
    // 读取文件元数据
    let metadata = exiftool.read_metadata("photo.jpg")?;
    
    // 打印所有元数据
    for (key, value) in metadata {
        println!("{}: {}", key, value);
    }
    
    Ok(())
}
```

### 读取特定标签

```rust
use exiftool_rs_wrapper::ExifTool;

let mut exiftool = ExifTool::new()?;
let make = exiftool.read_tag("photo.jpg", "Make")?;
println!("相机制造商: {:?}", make);
```

### 写入标签

```rust
use exiftool_rs_wrapper::ExifTool;

let mut exiftool = ExifTool::new()?;
exiftool.write_tag("photo.jpg", "Copyright", "© 2024 My Company")?;
```

### 删除标签

```rust
use exiftool_rs_wrapper::ExifTool;

let mut exiftool = ExifTool::new()?;
exiftool.delete_tag("photo.jpg", "GPSPosition")?;
```

### 复制元数据

```rust
use exiftool_rs_wrapper::ExifTool;

let mut exiftool = ExifTool::new()?;
exiftool.copy_metadata("source.jpg", "target.jpg")?;
```

## 命令行工具

本项目还提供了一个命令行工具：

```bash
# 读取文件元数据
exiftool-rs-wrapper read photo.jpg

# 写入标签
exiftool-rs-wrapper write photo.jpg Copyright "© 2024"

# 删除标签
exiftool-rs-wrapper delete photo.jpg GPSPosition

# 复制元数据
exiftool-rs-wrapper copy source.jpg target.jpg

# 查看版本
exiftool-rs-wrapper version

# 列出支持的标签
exiftool-rs-wrapper list
```

## API 文档

### `ExifTool`

主要的 ExifTool 包装器结构体。

#### 方法

- `new()` - 创建新的 ExifTool 实例
- `read_metadata(path)` - 读取文件的元数据
- `read_tag(path, tag)` - 读取特定标签的值
- `write_tag(path, tag, value)` - 写入标签值
- `delete_tag(path, tag)` - 删除标签
- `copy_metadata(source, target)` - 复制元数据
- `list_tags()` - 列出所有支持的标签
- `version()` - 获取 ExifTool 版本

### `ExifError`

错误类型枚举，包含：

- `Io` - IO 错误
- `Json` - JSON 解析错误
- `Execution` - ExifTool 执行错误
- `NotFound` - 未找到 ExifTool
- `InvalidPath` - 无效的路径

## 支持的文件格式

ExifTool 支持超过 200 种文件格式，包括但不限于：

- 图像：JPEG, TIFF, PNG, GIF, BMP, RAW 等
- 视频：MP4, AVI, MOV, MKV 等
- 音频：MP3, FLAC, AAC 等
- 文档：PDF, DOCX, EPUB 等

完整列表请参考 [ExifTool 官方文档](https://exiftool.org/#supported)。

## 开发

### 构建

```bash
cargo build --release
```

### 测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## 许可证

本项目采用 MIT 或 Apache-2.0 双许可证。

## 致谢

- [ExifTool](https://exiftool.org/) by Phil Harvey - 强大的元数据处理工具
