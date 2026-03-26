//! ExifTool Rust Wrapper 示例程序

use exiftool_rs_wrapper::{ExifError, ExifTool};
use std::env;

fn main() {
    // 检查命令行参数
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("用法: exiftool-rs-wrapper <命令> [参数...]");
        println!();
        println!("命令:");
        println!("  read <文件路径>       - 读取文件元数据");
        println!("  write <文件路径> <标签> <值>  - 写入元数据标签");
        println!("  delete <文件路径> <标签>    - 删除元数据标签");
        println!("  copy <源文件> <目标文件> - 复制元数据");
        println!("  version              - 显示 ExifTool 版本");
        println!("  list                 - 列出所有支持的标签");
        return;
    }

    // 初始化 ExifTool
    let mut exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(ExifError::NotFound) => {
            eprintln!("错误: 未找到 ExifTool。请先安装 ExifTool 并确保它在 PATH 中。");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("错误: 无法初始化 ExifTool: {}", e);
            std::process::exit(1);
        }
    };

    match args[1].as_str() {
        "read" => {
            if args.len() < 3 {
                eprintln!("错误: 请提供文件路径");
                std::process::exit(1);
            }
            let path = &args[2];
            match exiftool.read_metadata(path) {
                Ok(metadata) => {
                    println!("文件 '{}' 的元数据:", path);
                    for (key, value) in metadata {
                        println!("  {}: {}", key, value);
                    }
                }
                Err(e) => {
                    eprintln!("错误: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "write" => {
            if args.len() < 5 {
                eprintln!("错误: 用法: write <文件路径> <标签> <值>");
                std::process::exit(1);
            }
            let path = &args[2];
            let tag = &args[3];
            let value = &args[4];
            match exiftool.write_tag(path, tag, value) {
                Ok(_) => {
                    println!("成功写入标签 '{}' = '{}' 到文件 '{}'", tag, value, path);
                }
                Err(e) => {
                    eprintln!("错误: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "delete" => {
            if args.len() < 4 {
                eprintln!("错误: 用法: delete <文件路径> <标签>");
                std::process::exit(1);
            }
            let path = &args[2];
            let tag = &args[3];
            match exiftool.delete_tag(path, tag) {
                Ok(_) => {
                    println!("成功删除标签 '{}' 从文件 '{}'", tag, path);
                }
                Err(e) => {
                    eprintln!("错误: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("错误: 用法: copy <源文件> <目标文件>");
                std::process::exit(1);
            }
            let source = &args[2];
            let target = &args[3];
            match exiftool.copy_metadata(source, target) {
                Ok(_) => {
                    println!("成功复制元数据从 '{}' 到 '{}'", source, target);
                }
                Err(e) => {
                    eprintln!("错误: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "version" => match exiftool.version() {
            Ok(version) => {
                println!("ExifTool 版本: {}", version);
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        },
        "list" => match exiftool.list_tags() {
            Ok(tags) => {
                println!("支持的标签列表 (前 50 个):");
                for (i, tag) in tags.iter().take(50).enumerate() {
                    println!("  {}. {}", i + 1, tag);
                }
                if tags.len() > 50 {
                    println!("  ... 还有 {} 个标签", tags.len() - 50);
                }
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("错误: 未知命令 '{}'", args[1]);
            std::process::exit(1);
        }
    }
}
