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

        // 语言：在 stay_open 模式下，参数通过 stdin 逐行传递，
        // 必须将选项名和值拆分为两个独立参数
        if let Some(ref lang) = self.lang {
            args.push("-lang".to_string());
            args.push(lang.clone());
        }

        // 字符集：拆分为两个独立参数
        if let Some(ref charset) = self.charset {
            args.push("-charset".to_string());
            args.push(charset.clone());
        }

        // 原始数值
        if self.raw_values {
            args.push("-n".to_string());
        }

        // 递归
        if self.recursive {
            args.push("-r".to_string());
        }

        // 文件扩展名过滤：拆分为两个独立参数
        for ext in &self.extensions {
            args.push("-ext".to_string());
            args.push(ext.clone());
        }

        // 条件过滤：拆分为两个独立参数
        if let Some(ref condition) = self.condition {
            args.push("-if".to_string());
            args.push(condition.clone());
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
        // 使用 JSON 格式获取目录下所有文件的元数据，
        // ExifTool 会返回一个 JSON 数组，每个元素对应一个文件，
        // 这样可以避免之前使用 split("\n[{\\n") 分割文本的脆弱逻辑。
        let mut opts = options.clone();
        opts.recursive = true;
        // 强制使用 JSON 格式以确保解析的可靠性
        opts.format = OutputFormat::Json;

        let args = opts.build_args(&[path.as_ref()]);
        let response = self.execute_raw(&args)?;
        let content = response.text();

        // 解析 JSON 数组，每个元素对应一个文件的元数据
        let json_array: Vec<serde_json::Value> =
            serde_json::from_str(content.trim()).unwrap_or_default();

        let outputs: Vec<FormattedOutput> = json_array
            .into_iter()
            .map(|item| {
                // 将每个文件的元数据转换回用户请求的格式
                let file_content = match options.format {
                    OutputFormat::Json => {
                        // 保持 JSON 格式，将单个对象包装为数组以兼容 ExifTool 输出格式
                        serde_json::to_string_pretty(&vec![&item])
                            .unwrap_or_else(|_| item.to_string())
                    }
                    _ => {
                        // 对于非 JSON 格式，返回 JSON 字符串形式（用户可进一步处理）
                        serde_json::to_string_pretty(&item).unwrap_or_else(|_| item.to_string())
                    }
                };
                FormattedOutput {
                    format: options.format,
                    content: file_content,
                }
            })
            .collect();

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExifTool;
    use crate::error::Error;

    /// 最小有效 JPEG 文件字节数组，用于创建临时测试文件
    const TINY_JPEG: &[u8] = &[
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06,
        0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B,
        0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20,
        0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31,
        0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF,
        0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00,
        0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x09, 0xFF, 0xC4, 0x00, 0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01,
        0x00, 0x00, 0x3F, 0x00, 0xD2, 0xCF, 0x20, 0xFF, 0xD9,
    ];

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

    #[test]
    fn test_build_args_split_options() {
        // 验证 -lang、-charset、-ext、-if 参数被正确拆分为两个独立参数
        let opts = ReadOptions::new()
            .format(OutputFormat::Json)
            .lang("zh-CN")
            .charset("UTF8")
            .extension("jpg")
            .condition("$ImageWidth > 1000");

        let args = opts.build_args(&[std::path::Path::new("test.jpg")]);

        // 验证 -lang 和值是相邻的两个独立参数
        assert!(args.windows(2).any(|w| w == ["-lang", "zh-CN"]));
        // 验证 -charset 和值是相邻的两个独立参数
        assert!(args.windows(2).any(|w| w == ["-charset", "UTF8"]));
        // 验证 -ext 和值是相邻的两个独立参数
        assert!(args.windows(2).any(|w| w == ["-ext", "jpg"]));
        // 验证 -if 和条件表达式是相邻的两个独立参数
        assert!(args.windows(2).any(|w| w == ["-if", "$ImageWidth > 1000"]));
    }

    #[test]
    fn test_read_xml_contains_xml_tags() {
        // 检查 ExifTool 是否可用，不可用则跳过
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时出现意外错误: {:?}", e),
        };

        // 创建临时 JPEG 文件
        let tmp_dir = tempfile::tempdir().expect("创建临时目录失败");
        let test_file = tmp_dir.path().join("format_xml.jpg");
        std::fs::write(&test_file, TINY_JPEG).expect("写入临时 JPEG 文件失败");

        // 读取 XML 格式输出
        let xml_output = et.read_xml(&test_file).expect("读取 XML 格式元数据失败");

        // 验证输出包含 XML 标签特征
        assert!(
            xml_output.contains('<') && xml_output.contains('>'),
            "XML 输出应包含 XML 标签（尖括号），实际输出: {}",
            &xml_output[..xml_output.len().min(200)]
        );
        // ExifTool 的 -X 输出通常包含 rdf:RDF 或 rdf:Description
        assert!(
            xml_output.contains("rdf:")
                || xml_output.contains("<?xml")
                || xml_output.contains("et:"),
            "XML 输出应包含 XML 命名空间（如 rdf: 或 et:），实际输出: {}",
            &xml_output[..xml_output.len().min(300)]
        );
    }

    #[test]
    fn test_read_text_non_empty() {
        // 检查 ExifTool 是否可用，不可用则跳过
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时出现意外错误: {:?}", e),
        };

        // 创建临时 JPEG 文件
        let tmp_dir = tempfile::tempdir().expect("创建临时目录失败");
        let test_file = tmp_dir.path().join("format_text.jpg");
        std::fs::write(&test_file, TINY_JPEG).expect("写入临时 JPEG 文件失败");

        // 读取纯文本格式输出
        let text_output = et.read_text(&test_file).expect("读取纯文本格式元数据失败");

        // 验证输出非空
        let trimmed = text_output.trim();
        assert!(!trimmed.is_empty(), "纯文本格式输出不应为空");

        // 纯文本输出至少应包含文件相关信息（如 FileName、FileSize、MIMEType 等）
        // ExifTool -s 格式输出的每行格式为 "TagName: Value"
        assert!(
            trimmed.contains("FileName")
                || trimmed.contains("FileSize")
                || trimmed.contains("MIMEType"),
            "纯文本输出应包含基本文件信息标签，实际输出: {}",
            &trimmed[..trimmed.len().min(300)]
        );
    }
}
