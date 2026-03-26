//! ExifTool Rust Wrapper 命令行工具

use exiftool_rs_wrapper::ExifTool;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    // 创建 ExifTool 实例
    let exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(e) => {
            eprintln!("错误: 无法初始化 ExifTool: {}", e);
            eprintln!("请确保 ExifTool 已安装并添加到 PATH");
            std::process::exit(1);
        }
    };

    match args[1].as_str() {
        "read" => cmd_read(&exiftool, &args),
        "write" => cmd_write(&exiftool, &args),
        "delete" => cmd_delete(&exiftool, &args),
        "copy" => cmd_copy(&exiftool, &args),
        "version" => cmd_version(&exiftool),
        "list" => cmd_list(&exiftool),
        _ => {
            eprintln!("错误: 未知命令 '{}'", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("ExifTool Rust Wrapper - 命令行工具");
    println!();
    println!("用法: exiftool-rs-wrapper <命令> [参数...]");
    println!();
    println!("命令:");
    println!("  read <文件路径>              读取文件所有元数据");
    println!("  read <文件路径> <标签>       读取特定标签");
    println!("  write <文件路径> <标签> <值>  写入标签值");
    println!("  delete <文件路径> <标签>     删除标签");
    println!("  copy <源文件> <目标文件>    从源文件复制标签到目标文件");
    println!("  version                      显示 ExifTool 版本");
    println!("  list                         列出常用标签");
    println!();
    println!("示例:");
    println!("  exiftool-rs-wrapper read photo.jpg");
    println!("  exiftool-rs-wrapper read photo.jpg Make");
    println!("  exiftool-rs-wrapper write photo.jpg Copyright \"© 2024\"");
    println!("  exiftool-rs-wrapper delete photo.jpg Comment");
}

fn cmd_read(exiftool: &ExifTool, args: &[String]) {
    if args.len() < 3 {
        eprintln!("错误: 请提供文件路径");
        eprintln!("用法: read <文件路径> [标签]");
        std::process::exit(1);
    }

    let path = &args[2];

    if args.len() >= 4 {
        // 读取特定标签
        let tag = &args[3];
        match exiftool.read_tag::<String, _, _>(path, tag) {
            Ok(value) => {
                println!("{}: {}", tag, value);
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        }
    } else {
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
}

fn cmd_write(exiftool: &ExifTool, args: &[String]) {
    if args.len() < 5 {
        eprintln!("错误: 参数不足");
        eprintln!("用法: write <文件路径> <标签> <值>");
        std::process::exit(1);
    }

    let path = &args[2];
    let tag = &args[3];
    let value = &args[4];

    match exiftool
        .write(path)
        .tag(tag, value)
        .overwrite_original(true)
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

fn cmd_delete(exiftool: &ExifTool, args: &[String]) {
    if args.len() < 4 {
        eprintln!("错误: 参数不足");
        eprintln!("用法: delete <文件路径> <标签>");
        std::process::exit(1);
    }

    let path = &args[2];
    let tag = &args[3];

    match exiftool
        .write(path)
        .delete(tag)
        .overwrite_original(true)
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

fn cmd_copy(exiftool: &ExifTool, args: &[String]) {
    if args.len() < 4 {
        eprintln!("错误: 参数不足");
        eprintln!("用法: copy <源文件> <目标文件>");
        std::process::exit(1);
    }

    let source = &args[2];
    let target = &args[3];

    match exiftool
        .write(target)
        .copy_from(source)
        .overwrite_original(true)
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
