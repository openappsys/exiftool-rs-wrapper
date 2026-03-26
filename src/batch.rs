//! 批处理脚本和管道处理模块
//!
//! 支持批处理脚本执行和管道数据流

use crate::ExifTool;
use crate::error::{Error, Result};
use crate::types::Metadata;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

/// 批处理脚本
#[derive(Debug, Clone)]
pub struct BatchScript {
    /// 脚本命令列表
    commands: Vec<ScriptCommand>,
    /// 是否显示进度
    show_progress: bool,
    /// 遇到错误时是否继续
    continue_on_error: bool,
}

/// 脚本命令
#[derive(Debug, Clone)]
enum ScriptCommand {
    /// 读取文件元数据
    Read {
        path: PathBuf,
        tags: Option<Vec<String>>,
    },
    /// 写入标签
    Write {
        path: PathBuf,
        tag: String,
        value: String,
    },
    /// 删除标签
    Delete { path: PathBuf, tag: String },
    /// 批量读取
    BatchRead { paths: Vec<PathBuf> },
    /// 复制标签
    #[allow(dead_code)]
    CopyTags {
        source: PathBuf,
        target: PathBuf,
        #[allow(dead_code)]
        tags: Vec<String>,
    },
    /// 打印消息
    Print(String),
    /// 设置变量
    #[allow(dead_code)]
    SetVar {
        #[allow(dead_code)]
        name: String,
        #[allow(dead_code)]
        value: String,
    },
}

impl Default for BatchScript {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchScript {
    /// 创建新的批处理脚本
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            show_progress: true,
            continue_on_error: false,
        }
    }

    /// 从文件加载脚本
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(Error::Io)?;
        Self::from_string(content)
    }

    /// 从字符串解析脚本
    pub fn from_string(content: String) -> Result<Self> {
        let mut script = Self::new();

        for line in content.lines() {
            let line = line.trim();

            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let cmd = parts[0].to_lowercase();
            let args = &parts[1..];

            match cmd.as_str() {
                "read" => {
                    if !args.is_empty() {
                        let path = PathBuf::from(args[0]);
                        let tags = if args.len() > 1 {
                            Some(args[1..].iter().map(|s| s.to_string()).collect())
                        } else {
                            None
                        };
                        script.commands.push(ScriptCommand::Read { path, tags });
                    }
                }
                "write" => {
                    if args.len() >= 3 {
                        script.commands.push(ScriptCommand::Write {
                            path: PathBuf::from(args[0]),
                            tag: args[1].to_string(),
                            value: args[2..].join(" "),
                        });
                    }
                }
                "delete" => {
                    if args.len() >= 2 {
                        script.commands.push(ScriptCommand::Delete {
                            path: PathBuf::from(args[0]),
                            tag: args[1].to_string(),
                        });
                    }
                }
                "batch" => {
                    if !args.is_empty() {
                        script.commands.push(ScriptCommand::BatchRead {
                            paths: args.iter().map(PathBuf::from).collect(),
                        });
                    }
                }
                "copy" => {
                    if args.len() >= 3 {
                        script.commands.push(ScriptCommand::CopyTags {
                            source: PathBuf::from(args[0]),
                            target: PathBuf::from(args[1]),
                            tags: args[2..].iter().map(|s| s.to_string()).collect(),
                        });
                    }
                }
                "print" => {
                    if !args.is_empty() {
                        script.commands.push(ScriptCommand::Print(args.join(" ")));
                    }
                }
                "set" => {
                    if args.len() >= 2 {
                        script.commands.push(ScriptCommand::SetVar {
                            name: args[0].to_string(),
                            value: args[1..].join(" "),
                        });
                    }
                }
                "progress" => {
                    script.show_progress = args.first().map(|s| *s != "off").unwrap_or(true);
                }
                "continue_on_error" => {
                    script.continue_on_error = args.first().map(|s| *s == "on").unwrap_or(true);
                }
                _ => {
                    return Err(Error::invalid_arg(format!("未知命令: {}", cmd)));
                }
            }
        }

        Ok(script)
    }

    /// 显示进度
    pub fn show_progress(mut self, yes: bool) -> Self {
        self.show_progress = yes;
        self
    }

    /// 遇到错误时继续
    pub fn continue_on_error(mut self, yes: bool) -> Self {
        self.continue_on_error = yes;
        self
    }

    /// 执行脚本
    pub fn execute(&self, exiftool: &ExifTool) -> Result<BatchResult> {
        let mut result = BatchResult::new();
        let total = self.commands.len();

        for (i, cmd) in self.commands.iter().enumerate() {
            if self.show_progress {
                print!(
                    "\r进度: {}/{} ({:.1}%)",
                    i + 1,
                    total,
                    (i + 1) as f64 / total as f64 * 100.0
                );
                io::stdout().flush().unwrap();
            }

            match self.execute_command(exiftool, cmd) {
                Ok(_) => result.success += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("命令 {:?}: {}", cmd, e));
                    if !self.continue_on_error {
                        if self.show_progress {
                            println!();
                        }
                        return Err(e);
                    }
                }
            }
        }

        if self.show_progress {
            println!();
        }

        result.total = total;
        Ok(result)
    }

    /// 执行单个命令
    fn execute_command(&self, exiftool: &ExifTool, cmd: &ScriptCommand) -> Result<()> {
        match cmd {
            ScriptCommand::Read { path, tags } => {
                if let Some(tags) = tags {
                    for tag in tags {
                        let _: String = exiftool.read_tag(path, tag)?;
                    }
                } else {
                    exiftool.query(path).execute()?;
                }
            }
            ScriptCommand::Write { path, tag, value } => {
                exiftool
                    .write(path)
                    .tag(tag, value)
                    .overwrite_original(true)
                    .execute()?;
            }
            ScriptCommand::Delete { path, tag } => {
                exiftool
                    .write(path)
                    .delete(tag)
                    .overwrite_original(true)
                    .execute()?;
            }
            ScriptCommand::BatchRead { paths } => {
                exiftool.query_batch(paths).execute()?;
            }
            ScriptCommand::CopyTags {
                source,
                target,
                tags: _,
            } => {
                exiftool
                    .write(target)
                    .copy_from(source)
                    .overwrite_original(true)
                    .execute()?;
            }
            ScriptCommand::Print(msg) => {
                println!("{}", msg);
            }
            ScriptCommand::SetVar { name: _, value: _ } => {
                // 变量系统待实现
            }
        }
        Ok(())
    }
}

/// 批处理结果
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// 总命令数
    pub total: usize,
    /// 成功数
    pub success: usize,
    /// 失败数
    pub failed: usize,
    /// 错误信息
    pub errors: Vec<String>,
}

impl BatchResult {
    /// 创建新的结果
    fn new() -> Self {
        Self {
            total: 0,
            success: 0,
            failed: 0,
            errors: Vec::new(),
        }
    }

    /// 检查是否全部成功
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.success as f64 / self.total as f64
        }
    }
}

/// 管道处理器
pub struct PipeProcessor {
    exiftool: ExifTool,
    delimiter: String,
}

impl PipeProcessor {
    /// 创建新的管道处理器
    pub fn new(exiftool: ExifTool) -> Self {
        Self {
            exiftool,
            delimiter: "\n".to_string(),
        }
    }

    /// 设置分隔符
    pub fn delimiter(mut self, delim: impl Into<String>) -> Self {
        self.delimiter = delim.into();
        self
    }

    /// 从 stdin 读取并处理
    pub fn process_stdin(
        &self,
        processor: impl Fn(&ExifTool, &str) -> Result<String>,
    ) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line.map_err(Error::Io)?;

            if line.trim().is_empty() {
                continue;
            }

            match processor(&self.exiftool, &line) {
                Ok(output) => {
                    writeln!(stdout, "{}", output)?;
                }
                Err(e) => {
                    eprintln!("处理失败 '{}': {}", line, e);
                }
            }
        }

        Ok(())
    }

    /// 处理文件列表
    pub fn process_file_list<P: AsRef<Path>>(
        &self,
        list_file: P,
        processor: impl Fn(&ExifTool, &Path) -> Result<Metadata>,
    ) -> Result<Vec<(PathBuf, Metadata)>> {
        let content = std::fs::read_to_string(list_file.as_ref()).map_err(Error::Io)?;

        let mut results = Vec::new();

        for line in content.lines() {
            let path = PathBuf::from(line.trim());
            if path.exists() {
                match processor(&self.exiftool, &path) {
                    Ok(metadata) => {
                        results.push((path, metadata));
                    }
                    Err(e) => {
                        eprintln!("处理失败 '{}': {}", path.display(), e);
                    }
                }
            }
        }

        Ok(results)
    }
}

/// 批处理脚本示例
pub fn example_script() -> &'static str {
    r#"# ExifTool 批处理脚本示例
# 这是一个注释

# 设置进度显示
progress on

# 读取文件
read photo1.jpg
read photo2.jpg Make Model

# 写入标签
write photo1.jpg Copyright "© 2026 Photographer"
write photo2.jpg Artist "My Name"

# 批量读取
batch photo1.jpg photo2.jpg photo3.jpg

# 打印消息
print "批处理完成"
"#
}
