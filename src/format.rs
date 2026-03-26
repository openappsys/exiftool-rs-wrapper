//! 输出格式支持模块

use crate::ExifTool;
use crate::error::Result;
use std::path::Path;

/// 输出格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// JSON 格式（默认）
    #[default]
    Json,
    /// XML 格式
    Xml,
    /// CSV 格式
    Csv,
    /// TSV 格式
    Tsv,
    /// HTML 表格格式
    Html,
    /// 纯文本格式
    Text,
    /// 结构化数据
    Struct,
}

impl OutputFormat {
    /// 获取格式对应的参数
    fn arg(&self) -> &'static str {
        match self {
            Self::Json => "-json",
            Self::Xml => "-X", // 或 -xml
            Self::Csv => "-csv",
            Self::Tsv => "-t",  // 或 -tab
            Self::Html => "-h", // 或 -html
            Self::Text => "-s", // 短格式
            Self::Struct => "-struct",
        }
    }
}

/// 高级读取选项
#[derive(Debug, Clone, Default)]
pub struct ReadOptions {
    /// 输出格式
    pub format: OutputFormat,
    /// 排除特定标签
    pub exclude_tags: Vec<String>,
    /// 仅显示特定标签
    pub specific_tags: Vec<String>,
    /// 表格格式输出
    pub table_format: bool,
    /// 十六进制转储
    pub hex_dump: bool,
    /// 详细级别 (0-5)
    pub verbose: Option<u8>,
    /// 语言设置
    pub lang: Option<String>,
    /// 字符集
    pub charset: Option<String>,
    /// 显示原始数值
    pub raw_values: bool,
    /// 递归处理目录
    pub recursive: bool,
    /// 文件扩展名过滤
    pub extensions: Vec<String>,
    /// 条件过滤
    pub condition: Option<String>,
}

impl ReadOptions {
    /// 创建新的读取选项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置输出格式
    pub fn format(mut self, format: OutputFormat) -> Self {
        self.format = format;
        self
    }

    /// 排除标签
    pub fn exclude(mut self, tag: impl Into<String>) -> Self {
        self.exclude_tags.push(tag.into());
        self
    }

    /// 仅显示特定标签
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.specific_tags.push(tag.into());
        self
    }

    /// 表格格式输出
    pub fn table(mut self, yes: bool) -> Self {
        self.table_format = yes;
        self
    }

    /// 十六进制转储
    pub fn hex(mut self, yes: bool) -> Self {
        self.hex_dump = yes;
        self
    }

    /// 设置详细级别
    pub fn verbose(mut self, level: u8) -> Self {
        self.verbose = Some(level.min(5));
        self
    }

    /// 设置语言
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    /// 设置字符集
    pub fn charset(mut self, charset: impl Into<String>) -> Self {
        self.charset = Some(charset.into());
        self
    }

    /// 显示原始数值
    pub fn raw(mut self, yes: bool) -> Self {
        self.raw_values = yes;
        self
    }

    /// 递归处理
    pub fn recursive(mut self, yes: bool) -> Self {
        self.recursive = yes;
        self
    }

    /// 添加文件扩展名过滤
    pub fn extension(mut self, ext: impl Into<String>) -> Self {
        self.extensions.push(ext.into());
        self
    }

    /// 条件过滤
    pub fn condition(mut self, expr: impl Into<String>) -> Self {
        self.condition = Some(expr.into());
        self
    }

    /// 构建参数列表
    pub(crate) fn build_args(&self, paths: &[impl AsRef<Path>]) -> Vec<String> {
        let mut args = vec![self.format.arg().to_string()];

        // 表格格式
        if self.table_format {
            args.push("-T".to_string());
        }

        // 十六进制转储
        if self.hex_dump {
            args.push("-H".to_string());
        }

        // 详细级别
        if let Some(level) = self.verbose {
            args.push(format!("-v{}", level));
        }

        // 语言
        if let Some(ref lang) = self.lang {
            args.push(format!("-lang {}", lang));
        }

        // 字符集
        if let Some(ref charset) = self.charset {
            args.push(format!("-charset {}", charset));
        }

        // 原始数值
        if self.raw_values {
            args.push("-n".to_string());
        }

        // 递归
        if self.recursive {
            args.push("-r".to_string());
        }

        // 文件扩展名过滤
        for ext in &self.extensions {
            args.push(format!("-ext {}", ext));
        }

        // 条件过滤
        if let Some(ref condition) = self.condition {
            args.push(format!("-if {}", condition));
        }

        // 排除标签
        for tag in &self.exclude_tags {
            args.push(format!("-{}=", tag));
        }

        // 特定标签
        for tag in &self.specific_tags {
            args.push(format!("-{}", tag));
        }

        // 文件路径
        for path in paths {
            args.push(path.as_ref().to_string_lossy().to_string());
        }

        args
    }
}

/// 格式化输出结果
#[derive(Debug, Clone)]
pub struct FormattedOutput {
    /// 输出格式
    pub format: OutputFormat,
    /// 内容
    pub content: String,
}

impl FormattedOutput {
    /// 解析为 JSON
    pub fn to_json<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_str(&self.content).map_err(|e| e.into())
    }

    /// 获取纯文本内容
    pub fn text(&self) -> &str {
        &self.content
    }
}

/// 扩展 ExifTool 以支持格式化输出
pub trait FormatOperations {
    /// 使用自定义格式读取元数据
    fn read_formatted<P: AsRef<Path>>(
        &self,
        path: P,
        options: &ReadOptions,
    ) -> Result<FormattedOutput>;

    /// 读取为 XML
    fn read_xml<P: AsRef<Path>>(&self, path: P) -> Result<String>;

    /// 读取为 CSV
    fn read_csv<P: AsRef<Path>>(&self, path: P) -> Result<String>;

    /// 读取为 HTML 表格
    fn read_html<P: AsRef<Path>>(&self, path: P) -> Result<String>;

    /// 读取为纯文本
    fn read_text<P: AsRef<Path>>(&self, path: P) -> Result<String>;

    /// 递归读取目录
    fn read_directory<P: AsRef<Path>>(
        &self,
        path: P,
        options: &ReadOptions,
    ) -> Result<Vec<FormattedOutput>>;
}

impl FormatOperations for ExifTool {
    fn read_formatted<P: AsRef<Path>>(
        &self,
        path: P,
        options: &ReadOptions,
    ) -> Result<FormattedOutput> {
        let args = options.build_args(&[path.as_ref()]);
        let response = self.execute_raw(&args)?;

        Ok(FormattedOutput {
            format: options.format,
            content: response.text(),
        })
    }

    fn read_xml<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let options = ReadOptions::new().format(OutputFormat::Xml);
        let output = self.read_formatted(path, &options)?;
        Ok(output.content)
    }

    fn read_csv<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let options = ReadOptions::new().format(OutputFormat::Csv);
        let output = self.read_formatted(path, &options)?;
        Ok(output.content)
    }

    fn read_html<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let options = ReadOptions::new().format(OutputFormat::Html);
        let output = self.read_formatted(path, &options)?;
        Ok(output.content)
    }

    fn read_text<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let options = ReadOptions::new().format(OutputFormat::Text);
        let output = self.read_formatted(path, &options)?;
        Ok(output.content)
    }

    fn read_directory<P: AsRef<Path>>(
        &self,
        path: P,
        options: &ReadOptions,
    ) -> Result<Vec<FormattedOutput>> {
        let mut opts = options.clone();
        opts.recursive = true;
        let args = opts.build_args(&[path.as_ref()]);
        let response = self.execute_raw(&args)?;

        // 解析多文件响应
        let content = response.text();
        let outputs: Vec<FormattedOutput> = content
            .split("\n[{\\n") // 简化的分隔符，实际可能更复杂
            .filter(|s| !s.is_empty())
            .map(|s| FormattedOutput {
                format: options.format,
                content: s.to_string(),
            })
            .collect();

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format() {
        assert_eq!(OutputFormat::Json.arg(), "-json");
        assert_eq!(OutputFormat::Xml.arg(), "-X");
        assert_eq!(OutputFormat::Csv.arg(), "-csv");
    }

    #[test]
    fn test_read_options() {
        let opts = ReadOptions::new()
            .format(OutputFormat::Json)
            .tag("Make")
            .tag("Model")
            .verbose(2)
            .raw(true);

        assert_eq!(opts.format, OutputFormat::Json);
        assert_eq!(opts.specific_tags.len(), 2);
        assert_eq!(opts.verbose, Some(2));
        assert!(opts.raw_values);
    }

    #[test]
    fn test_read_options_build_args() {
        let opts = ReadOptions::new()
            .format(OutputFormat::Json)
            .tag("Make")
            .raw(true);

        let args = opts.build_args(&[std::path::Path::new("test.jpg")]);

        assert!(args.contains(&"-json".to_string()));
        assert!(args.contains(&"-Make".to_string()));
        assert!(args.contains(&"-n".to_string()));
        assert!(args.contains(&"test.jpg".to_string()));
    }
}
