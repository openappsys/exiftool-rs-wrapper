//! ExifTool Rust Wrapper 命令行工具

use clap::{Parser, Subcommand};
use exiftool_rs_wrapper::ExifTool;

#[derive(Debug, Parser)]
#[command(name = "exiftool-rs-wrapper")]
#[command(about = "ExifTool Rust Wrapper 命令行工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 读取文件元数据
    Read {
        /// 文件路径
        path: String,
        /// 可选的标签名
        tag: Option<String>,
    },
    /// 写入标签
    Write {
        /// 文件路径
        path: String,
        /// 标签名
        tag: String,
        /// 标签值
        value: String,
        /// 是否直接覆盖原文件（默认 false）
        #[arg(long)]
        overwrite: bool,
    },
    /// 删除标签
    Delete {
        /// 文件路径
        path: String,
        /// 标签名
        tag: String,
        /// 是否直接覆盖原文件（默认 false）
        #[arg(long)]
        overwrite: bool,
    },
    /// 从源文件复制标签到目标文件
    Copy {
        /// 源文件路径
        source: String,
        /// 目标文件路径
        target: String,
        /// 是否直接覆盖原文件（默认 false）
        #[arg(long)]
        overwrite: bool,
    },
    /// 显示 ExifTool 版本
    Version,
    /// 列出常用标签
    List,
}

fn main() {
    let cli = Cli::parse();

    // 创建 ExifTool 实例
    let exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(e) => {
            eprintln!("错误: 无法初始化 ExifTool: {}", e);
            eprintln!("请确保 ExifTool 已安装并添加到 PATH");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Read { path, tag } => cmd_read(&exiftool, &path, tag.as_deref()),
        Commands::Write {
            path,
            tag,
            value,
            overwrite,
        } => cmd_write(&exiftool, &path, &tag, &value, overwrite),
        Commands::Delete {
            path,
            tag,
            overwrite,
        } => cmd_delete(&exiftool, &path, &tag, overwrite),
        Commands::Copy {
            source,
            target,
            overwrite,
        } => cmd_copy(&exiftool, &source, &target, overwrite),
        Commands::Version => cmd_version(&exiftool),
        Commands::List => cmd_list(&exiftool),
    }
}

fn cmd_read(exiftool: &ExifTool, path: &str, tag: Option<&str>) {
    if let Some(tag) = tag {
        // 读取特定标签
        match exiftool.read_tag::<String, _, _>(path, tag) {
            Ok(value) => {
                println!("{}: {}", tag, value);
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // 读取所有元数据
    match exiftool.query(path).execute() {
        Ok(metadata) => {
            println!("文件 '{}' 的元数据:", path);
            println!("{:#?}", metadata);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_write(exiftool: &ExifTool, path: &str, tag: &str, value: &str, overwrite_original: bool) {
    match exiftool
        .write(path)
        .tag(tag, value)
        .overwrite_original(overwrite_original)
        .execute()
    {
        Ok(_) => {
            println!("成功写入标签 '{}' = '{}' 到文件 '{}'", tag, value, path);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_delete(exiftool: &ExifTool, path: &str, tag: &str, overwrite_original: bool) {
    match exiftool
        .write(path)
        .delete(tag)
        .overwrite_original(overwrite_original)
        .execute()
    {
        Ok(_) => {
            println!("成功删除标签 '{}' 从文件 '{}'", tag, path);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_copy(exiftool: &ExifTool, source: &str, target: &str, overwrite_original: bool) {
    match exiftool
        .write(target)
        .copy_from(source)
        .overwrite_original(overwrite_original)
        .execute()
    {
        Ok(_) => {
            println!("成功复制元数据从 '{}' 到 '{}'", source, target);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_version(exiftool: &ExifTool) {
    match exiftool.version() {
        Ok(version) => {
            println!("ExifTool 版本: {}", version);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_list(_exiftool: &ExifTool) {
    println!("常用标签列表:");
    println!();
    println!("文件信息:");
    println!("  - FileName, FileSize, FileType, MIMEType");
    println!("  - FileModifyDate, FileAccessDate");
    println!();
    println!("图像信息:");
    println!("  - ImageWidth, ImageHeight");
    println!("  - XResolution, YResolution");
    println!();
    println!("相机信息:");
    println!("  - Make, Model");
    println!("  - DateTimeOriginal, CreateDate, ModifyDate");
    println!();
    println!("拍摄参数:");
    println!("  - ExposureTime, FNumber, ISO");
    println!("  - FocalLength, FocalLengthIn35mmFormat");
    println!("  - ExposureCompensation, MeteringMode");
    println!();
    println!("GPS 信息:");
    println!("  - GPSLatitude, GPSLongitude, GPSAltitude");
    println!();
    println!("版权信息:");
    println!("  - Artist, Copyright, ImageDescription");
    println!();
    println!("使用 'exiftool -list' 查看所有支持的标签");
}
