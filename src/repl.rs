//! REPL 交互式命令行工具
//!
//! 提供交互式的 ExifTool shell

use crate::error::Result;
use crate::ExifTool;
use std::io::{self, BufRead, Write};

/// REPL 交互式 shell
pub struct ReplShell {
    exiftool: ExifTool,
    verbose: bool,
    show_help_on_start: bool,
}

impl ReplShell {
    /// 创建新的 REPL shell
    pub fn new(exiftool: ExifTool) -> Self {
        Self {
            exiftool,
            verbose: false,
            show_help_on_start: true,
        }
    }

    /// 启用详细模式
    pub fn verbose(mut self, yes: bool) -> Self {
        self.verbose = yes;
        self
    }

    /// 启动时显示帮助
    pub fn show_help_on_start(mut self, yes: bool) -> Self {
        self.show_help_on_start = yes;
        self
    }

    /// 运行 REPL 循环
    pub fn run(&self) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        if self.show_help_on_start {
            self.print_help(&mut stdout)?;
        }

        writeln!(stdout, "ExifTool REPL v0.1.0")?;
        writeln!(stdout, "输入 'help' 查看帮助，输入 'quit' 退出。\n")?;

        loop {
            write!(stdout, "exiftool> ")?;
            stdout.flush()?;

            let mut line = String::new();
            if stdin.read_line(&mut line).is_err() {
                break;
            }

            let input = line.trim();
            if input.is_empty() {
                continue;
            }

            match self.process_command(input) {
                Ok(should_quit) => {
                    if should_quit {
                        writeln!(stdout, "再见！")?;
                        break;
                    }
                }
                Err(e) => {
                    writeln!(stdout, "错误: {}", e)?;
                }
            }
        }

        Ok(())
    }

    /// 处理单个命令
    fn process_command(&self, input: &str) -> Result<bool> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(false);
        }

        let cmd = parts[0].to_lowercase();
        let args = &parts[1..];

        match cmd.as_str() {
            "quit" | "exit" | "q" => Ok(true),
            "help" | "h" => {
                self.print_help(&mut io::stdout())?;
                Ok(false)
            }
            "read" | "r" => {
                if args.is_empty() {
                    println!("用法: read <文件路径> [标签名]");
                } else {
                    self.cmd_read(args)?;
                }
                Ok(false)
            }
            "write" | "w" => {
                if args.len() < 3 {
                    println!("用法: write <文件路径> <标签名> <值>");
                } else {
                    self.cmd_write(args)?;
                }
                Ok(false)
            }
            "delete" | "d" => {
                if args.len() < 2 {
                    println!("用法: delete <文件路径> <标签名>");
                } else {
                    self.cmd_delete(args)?;
                }
                Ok(false)
            }
            "batch" | "b" => {
                if args.is_empty() {
                    println!("用法: batch <文件路径1> [<文件路径2> ...]");
                } else {
                    self.cmd_batch(args)?;
                }
                Ok(false)
            }
            "tags" | "t" => {
                self.cmd_list_tags()?;
                Ok(false)
            }
            "verbose" | "v" => {
                println!("详细模式: {}", self.verbose);
                Ok(false)
            }
            "version" => {
                match self.exiftool.version() {
                    Ok(ver) => println!("ExifTool 版本: {}", ver),
                    Err(e) => println!("无法获取版本: {}", e),
                }
                Ok(false)
            }
            _ => {
                println!("未知命令: {}", cmd);
                println!("输入 'help' 查看可用命令");
                Ok(false)
            }
        }
    }

    /// read 命令
    fn cmd_read(&self, args: &[&str]) -> Result<()> {
        let path = args[0];

        if args.len() > 1 {
            // 读取特定标签
            let tag_name = args[1];
            match self.exiftool.read_tag::<String, _, _>(path, tag_name) {
                Ok(value) => println!("{}: {}", tag_name, value),
                Err(e) => println!("读取标签失败: {}", e),
            }
        } else {
            // 读取所有元数据
            match self.exiftool.query(path).execute() {
                Ok(metadata) => {
                    println!("文件: {}", path);
                    for (key, value) in metadata.iter() {
                        println!("  {}: {}", key, value);
                    }
                }
                Err(e) => println!("读取元数据失败: {}", e),
            }
        }

        Ok(())
    }

    /// write 命令
    fn cmd_write(&self, args: &[&str]) -> Result<()> {
        let path = args[0];
        let tag = args[1];
        let value = args[2..].join(" ");

        match self
            .exiftool
            .write(path)
            .tag(tag, &value)
            .overwrite_original(true)
            .execute()
        {
            Ok(_) => println!("写入成功: {} = {}", tag, value),
            Err(e) => println!("写入失败: {}", e),
        }

        Ok(())
    }

    /// delete 命令
    fn cmd_delete(&self, args: &[&str]) -> Result<()> {
        let path = args[0];
        let tag = args[1];

        match self
            .exiftool
            .write(path)
            .delete(tag)
            .overwrite_original(true)
            .execute()
        {
            Ok(_) => println!("删除标签成功: {}", tag),
            Err(e) => println!("删除标签失败: {}", e),
        }

        Ok(())
    }

    /// batch 命令
    fn cmd_batch(&self, args: &[&str]) -> Result<()> {
        let paths: Vec<&str> = args.to_vec();

        match self.exiftool.query_batch(&paths).execute() {
            Ok(results) => {
                for (path, metadata) in results {
                    println!("\n文件: {}", path.display());
                    if self.verbose {
                        for (key, value) in metadata.iter() {
                            println!("  {}: {}", key, value);
                        }
                    } else {
                        // 只显示关键信息
                        if let Some(make) = metadata.get("Make") {
                            println!("  制造商: {}", make);
                        }
                        if let Some(model) = metadata.get("Model") {
                            println!("  型号: {}", model);
                        }
                        if let Some(date) = metadata.get("DateTimeOriginal") {
                            println!("  拍摄时间: {}", date);
                        }
                    }
                }
            }
            Err(e) => println!("批量读取失败: {}", e),
        }

        Ok(())
    }

    /// tags 命令
    fn cmd_list_tags(&self) -> Result<()> {
        println!("常用标签:");
        println!("  EXIF: Make, Model, DateTimeOriginal, ImageWidth, ImageHeight");
        println!("  IPTC: Keywords, Caption-Abstract, City, Country");
        println!("  XMP:  Title, Creator, Description, Rights");
        println!("  GPS:  GPSLatitude, GPSLongitude, GPSAltitude");
        println!("\n使用 'read <文件> <标签名>' 读取特定标签");
        Ok(())
    }

    /// 打印帮助信息
    fn print_help(&self, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "\n=== ExifTool REPL 帮助 ===\n")?;
        writeln!(writer, "命令列表:")?;
        writeln!(writer, "  read, r    <路径> [标签]    读取文件元数据")?;
        writeln!(writer, "  write, w   <路径> <标签> <值> 写入标签")?;
        writeln!(writer, "  delete, d  <路径> <标签>    删除标签")?;
        writeln!(writer, "  batch, b   <路径1> ...      批量处理")?;
        writeln!(writer, "  tags, t                   列出常用标签")?;
        writeln!(writer, "  verbose, v                切换详细模式")?;
        writeln!(writer, "  version                   显示版本")?;
        writeln!(writer, "  help, h                   显示帮助")?;
        writeln!(writer, "  quit, exit, q             退出\n")?;
        Ok(())
    }
}

/// 启动 REPL shell 的便捷函数
pub fn run_repl(exiftool: ExifTool) -> Result<()> {
    let shell = ReplShell::new(exiftool);
    shell.run()
}
