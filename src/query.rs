//! 查询构建器 - Builder 模式实现

use std::path::{Path, PathBuf};

use crate::ExifTool;
use crate::error::{Error, Result};
use crate::process::Response;
use crate::types::{Metadata, TagId};

/// 转义格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeFormat {
    /// HTML 转义 (-E)
    Html,
    /// XML 转义 (-ex)
    Xml,
    /// C 语言转义 (-ec)
    C,
}

/// 查询构建器
pub struct QueryBuilder<'et> {
    exiftool: &'et ExifTool,
    path: PathBuf,
    args: Vec<String>,
    include_unknown: bool,
    include_duplicates: bool,
    raw_values: bool,
    group_by_category: bool,
    no_composite: bool,
    extract_embedded: Option<u8>,
    extensions: Vec<String>,
    ignore_dirs: Vec<String>,
    recursive: bool,
    progress_interval: Option<u32>,
    progress_title: Option<String>,
    specific_tags: Vec<String>,
    excluded_tags: Vec<String>,
    // 输出格式选项
    decimal: bool,
    escape_format: Option<EscapeFormat>,
    force_print: bool,
    group_names: Option<u8>,
    html_format: bool,
    hex: bool,
    long_format: bool,
    latin: bool,
    short_format: Option<u8>,
    tab_format: bool,
    table_format: bool,
    text_out: Option<String>,
    tag_out: Option<String>,
    tag_out_ext: Vec<String>,
    list_item: Option<u32>,
    file_order: Option<(String, bool)>,
    quiet: bool,
    // 高级输出选项
    html_dump: Option<u32>,
    php_format: bool,
    plot_format: bool,
    args_format: bool,
    // 其他选项
    common_args: Vec<String>,
    echo: Vec<(String, Option<String>)>,
    efile: Option<String>,
}

impl<'et> QueryBuilder<'et> {
    /// 创建新的查询构建器
    pub(crate) fn new<P: AsRef<Path>>(exiftool: &'et ExifTool, path: P) -> Self {
        Self {
            exiftool,
            path: path.as_ref().to_path_buf(),
            args: Vec::new(),
            include_unknown: false,
            include_duplicates: false,
            raw_values: false,
            group_by_category: false,
            no_composite: false,
            extract_embedded: None,
            extensions: Vec::new(),
            ignore_dirs: Vec::new(),
            recursive: false,
            progress_interval: None,
            progress_title: None,
            specific_tags: Vec::new(),
            excluded_tags: Vec::new(),
            // 输出格式选项
            decimal: false,
            escape_format: None,
            force_print: false,
            group_names: None,
            html_format: false,
            hex: false,
            long_format: false,
            latin: false,
            short_format: None,
            tab_format: false,
            table_format: false,
            text_out: None,
            tag_out: None,
            tag_out_ext: Vec::new(),
            list_item: None,
            file_order: None,
            quiet: false,
            // 高级输出选项
            html_dump: None,
            php_format: false,
            plot_format: false,
            args_format: false,
            // 其他选项
            common_args: Vec::new(),
            echo: Vec::new(),
            efile: None,
        }
    }

    /// 包含未知标签
    pub fn include_unknown(mut self, yes: bool) -> Self {
        self.include_unknown = yes;
        self
    }

    /// 包含重复标签
    pub fn include_duplicates(mut self, yes: bool) -> Self {
        self.include_duplicates = yes;
        self
    }

    /// 显示原始数值（而非格式化后的值）
    pub fn raw_values(mut self, yes: bool) -> Self {
        self.raw_values = yes;
        self
    }

    /// 按类别分组（-g1 选项）
    pub fn group_by_category(mut self, yes: bool) -> Self {
        self.group_by_category = yes;
        self
    }

    /// 禁用复合标签生成
    ///
    /// 使用 `-e` 选项禁用复合标签（Composite tags）的生成。
    /// 复合标签是由 ExifTool 根据其他标签计算得出的派生标签。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 只读取原始标签，不生成复合标签
    /// let metadata = exiftool.query("photo.jpg")
    ///     .no_composite(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn no_composite(mut self, yes: bool) -> Self {
        self.no_composite = yes;
        self
    }

    /// 提取嵌入文件信息
    ///
    /// 使用 `-ee` 选项从文件中提取嵌入的文件信息。
    /// 例如从 RAW 文件中提取 JPEG 预览图的元数据。
    ///
    /// # 级别
    ///
    /// - `None` - 不提取嵌入文件（默认）
    /// - `Some(1)` - `-ee` 提取直接嵌入的文件
    /// - `Some(2)` - `-ee2` 提取所有层级的嵌入文件
    /// - `Some(3+)` - `-ee3` 及以上更深入的提取
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 提取嵌入文件信息
    /// let metadata = exiftool.query("photo.raw")
    ///     .extract_embedded(Some(1))
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn extract_embedded(mut self, level: Option<u8>) -> Self {
        self.extract_embedded = level;
        self
    }

    /// 设置文件扩展名过滤
    ///
    /// 使用 `-ext` 选项只处理指定扩展名的文件。
    /// 可以使用多次来指定多个扩展名。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 只处理 jpg 文件
    /// let metadata = exiftool.query("/photos")
    ///     .extension("jpg")
    ///     .recursive(true)
    ///     .execute()?;
    ///
    /// // 处理多个扩展名
    /// let metadata = exiftool.query("/photos")
    ///     .extension("jpg")
    ///     .extension("png")
    ///     .extension("raw")
    ///     .recursive(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn extension(mut self, ext: impl Into<String>) -> Self {
        self.extensions.push(ext.into());
        self
    }

    /// 设置要忽略的目录
    ///
    /// 使用 `-i` 选项忽略指定的目录名称。
    /// 在递归处理时，匹配的目录将被跳过。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 忽略 .git 和 node_modules 目录
    /// let metadata = exiftool.query("/project")
    ///     .ignore(".git")
    ///     .ignore("node_modules")
    ///     .recursive(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ignore(mut self, dir: impl Into<String>) -> Self {
        self.ignore_dirs.push(dir.into());
        self
    }

    /// 递归处理子目录
    ///
    /// 使用 `-r` 选项递归处理目录中的所有文件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 递归处理整个目录树
    /// let metadata = exiftool.query("/photos")
    ///     .recursive(true)
    ///     .extension("jpg")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn recursive(mut self, yes: bool) -> Self {
        self.recursive = yes;
        self
    }

    /// 启用进度显示
    ///
    /// 使用 `-progress` 选项在处理文件时显示进度信息。
    /// 可以指定间隔（每隔多少文件显示一次）和标题。
    ///
    /// # 参数
    ///
    /// - `interval` - 每隔多少文件显示一次进度（None 表示每个文件都显示）
    /// - `title` - 进度信息的标题（可选）
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 每 10 个文件显示一次进度
    /// let metadata = exiftool.query("/photos")
    ///     .recursive(true)
    ///     .progress(Some(10), Some("Processing"))
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn progress(mut self, interval: Option<u32>, title: Option<impl Into<String>>) -> Self {
        self.progress_interval = interval;
        self.progress_title = title.map(|t| t.into());
        self
    }

    /// 添加特定标签查询
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.specific_tags.push(tag.into());
        self
    }

    /// 添加多个标签查询
    pub fn tags(mut self, tags: &[impl AsRef<str>]) -> Self {
        for tag in tags {
            self.specific_tags.push(tag.as_ref().to_string());
        }
        self
    }

    /// 添加特定标签查询（使用 TagId）
    pub fn tag_id(self, tag: TagId) -> Self {
        self.tag(tag.name())
    }

    /// 排除特定标签
    pub fn exclude(mut self, tag: impl Into<String>) -> Self {
        self.excluded_tags.push(tag.into());
        self
    }

    /// 排除多个标签
    pub fn excludes(mut self, tags: &[impl AsRef<str>]) -> Self {
        for tag in tags {
            self.excluded_tags.push(tag.as_ref().to_string());
        }
        self
    }

    /// 排除特定标签（使用 TagId）
    pub fn exclude_id(self, tag: TagId) -> Self {
        self.exclude(tag.name())
    }

    /// 使用特定编码
    pub fn charset(mut self, charset: impl Into<String>) -> Self {
        self.args.push("-charset".to_string());
        self.args.push(charset.into());
        self
    }

    /// 使用特定语言
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.args.push("-lang".to_string());
        self.args.push(lang.into());
        self
    }

    /// 设置 GPS 坐标格式
    ///
    /// 使用 `-c` 选项设置 GPS 坐标的输出格式
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 使用小数度格式
    /// let metadata = exiftool.query("photo.jpg")
    ///     .coord_format("%.6f")
    ///     .execute()?;
    ///
    /// // 使用度分秒格式
    /// let metadata = exiftool.query("photo.jpg")
    ///     .coord_format("%d deg %d' %.2f\"")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn coord_format(mut self, format: impl Into<String>) -> Self {
        self.args.push("-c".to_string());
        self.args.push(format.into());
        self
    }

    /// 设置日期/时间格式
    ///
    /// 使用 `-d` 选项设置日期/时间值的输出格式
    ///
    /// # 预设格式
    ///
    /// - `"%Y:%m:%d %H:%M:%S"` - 标准 EXIF 格式（默认）
    /// - `"%Y-%m-%d"` - ISO 日期格式
    /// - `"%c"` - 本地时间格式
    /// - `"%F %T"` - ISO 8601 格式
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 使用 ISO 格式
    /// let metadata = exiftool.query("photo.jpg")
    ///     .date_format("%Y-%m-%d %H:%M:%S")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn date_format(mut self, format: impl Into<String>) -> Self {
        self.args.push("-d".to_string());
        self.args.push(format.into());
        self
    }

    /// 添加原始参数（高级用法）
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// 从文件读取参数
    ///
    /// 对应 ExifTool 的 `-@` 选项，从指定文件中读取命令行参数，
    /// 每行一个参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .args_file("args.txt")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn args_file(mut self, path: impl Into<String>) -> Self {
        self.args.push("-@".to_string());
        self.args.push(path.into());
        self
    }

    /// 设置 CSV 分隔符
    ///
    /// 对应 ExifTool 的 `-csvDelim` 选项，设置 CSV 输出中使用的分隔符字符。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let output = exiftool.query("photo.jpg")
    ///     .csv_delimiter("\t")
    ///     .execute_text()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn csv_delimiter(mut self, delim: impl Into<String>) -> Self {
        self.args.push("-csvDelim".to_string());
        self.args.push(delim.into());
        self
    }

    /// 加载替代文件的标签信息
    ///
    /// 对应 ExifTool 的 `-fileNUM` 选项，从替代文件中加载标签。
    /// `num` 为文件编号（1-9），`path` 为替代文件路径。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .alternate_file(1, "other.jpg")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn alternate_file(mut self, num: u8, path: impl Into<String>) -> Self {
        self.args.push(format!("-file{}", num));
        self.args.push(path.into());
        self
    }

    /// 递归处理子目录（包含隐藏目录）
    ///
    /// 对应 ExifTool 的 `-r.` 选项，递归处理时包含以 `.` 开头的隐藏目录。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("/photos")
    ///     .recursive_hidden()
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn recursive_hidden(mut self) -> Self {
        self.args.push("-r.".to_string());
        self
    }

    /// 设置源文件格式
    ///
    /// 对应 ExifTool 的 `-srcfile` 选项，指定处理时使用的源文件格式字符串。
    /// 支持使用 `%d`、`%f`、`%e` 等占位符来匹配不同的源文件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 从 XMP sidecar 文件读取标签
    /// let metadata = exiftool.query("photo.jpg")
    ///     .source_file("%d%f.xmp")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn source_file(mut self, fmt: impl Into<String>) -> Self {
        self.args.push("-srcfile".to_string());
        self.args.push(fmt.into());
        self
    }

    /// 提取未知二进制标签
    ///
    /// 对应 ExifTool 的 `-U` 选项，提取未知的二进制标签值。
    /// 比 `-u` 更激进，会尝试解码未知的二进制数据。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .unknown_binary()
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn unknown_binary(mut self) -> Self {
        self.args.push("-U".to_string());
        self
    }

    /// 加载 ExifTool 插件模块
    ///
    /// 对应 ExifTool 的 `-use` 选项，加载指定的 ExifTool 插件模块。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .use_module("MWG")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn use_module(mut self, module: impl Into<String>) -> Self {
        self.args.push("-use".to_string());
        self.args.push(module.into());
        self
    }

    /// 设置自定义打印格式
    ///
    /// 使用 `-p` 选项按指定格式打印输出。
    /// 使用 `$TAGNAME` 语法引用标签值。
    ///
    /// # 格式语法
    ///
    /// - `$TAGNAME` - 插入标签值
    /// - `$TAGNAME#` - 插入原始数值（无格式化）
    /// - `${TAGNAME:FMT}` - 使用指定格式
    /// - `$$` - 插入 `$` 字符
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 自定义输出格式
    /// let output = exiftool.query("photo.jpg")
    ///     .print_format("$FileName: $DateTimeOriginal ($Make $Model)")
    ///     .execute_text()?;
    ///
    /// println!("{}", output);
    /// # Ok(())
    /// # }
    /// ```
    pub fn print_format(mut self, format: impl Into<String>) -> Self {
        self.args.push("-p".to_string());
        self.args.push(format.into());
        self
    }

    /// 禁用打印转换
    ///
    /// 使用 `-n` 选项禁用所有打印转换，显示原始数值。
    /// 与 `raw_values()` 功能相同，但提供更直观的命名。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 禁用打印转换，获取原始数值
    /// let metadata = exiftool.query("photo.jpg")
    ///     .no_print_conv()
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn no_print_conv(mut self) -> Self {
        self.raw_values = true;
        self
    }

    /// 二进制输出
    ///
    /// 使用 `-b` 选项以二进制格式输出标签值。
    /// 通常用于提取缩略图、预览图等二进制数据。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 以二进制格式输出
    /// let output = exiftool.query("photo.jpg")
    ///     .binary()
    ///     .tag("ThumbnailImage")
    ///     .execute_text()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn binary(mut self) -> Self {
        self.args.push("-b".to_string());
        self
    }

    /// 按组分类输出
    ///
    /// 使用 `-g` 选项按组分类显示标签。
    /// 可选参数指定分组级别（0-7），默认为 0。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 按默认分组级别显示
    /// let metadata = exiftool.query("photo.jpg")
    ///     .group_headings(None)
    ///     .execute()?;
    ///
    /// // 按指定分组级别显示
    /// let metadata = exiftool.query("photo.jpg")
    ///     .group_headings(Some(1))
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn group_headings(mut self, num: Option<u8>) -> Self {
        match num {
            Some(n) => self.args.push(format!("-g{}", n)),
            None => self.args.push("-g".to_string()),
        }
        self
    }

    /// 短输出格式
    ///
    /// 使用 `-s` 选项以短格式输出标签名。
    /// 可选参数指定短格式级别：
    /// - `None` 或 `Some(1)` - `-s` 使用标签名而非描述
    /// - `Some(2)` - `-s2` 更短的输出
    /// - `Some(3)` - `-s3` 最短输出
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .short(Some(2))
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn short(mut self, level: Option<u8>) -> Self {
        match level {
            Some(n) if n > 1 => self.args.push(format!("-s{}", n)),
            _ => self.args.push("-s".to_string()),
        }
        self
    }

    /// 极短输出格式
    ///
    /// 使用 `-S` 选项以极短格式输出（仅标签名和值）
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let output = exiftool.query("photo.jpg")
    ///     .very_short()
    ///     .execute_text()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn very_short(mut self) -> Self {
        self.args.push("-S".to_string());
        self
    }

    /// 允许重复标签
    ///
    /// 使用 `-a` 选项允许输出中包含重复的标签。
    /// 与 `include_duplicates(true)` 功能相同。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .allow_duplicates()
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn allow_duplicates(mut self) -> Self {
        self.include_duplicates = true;
        self
    }

    /// 提取未知标签
    ///
    /// 使用 `-u` 选项提取未知标签。
    /// 与 `include_unknown(true)` 功能相同。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .unknown()
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn unknown(mut self) -> Self {
        self.include_unknown = true;
        self
    }

    /// XML 格式输出
    ///
    /// 使用 `-X` 选项以 XML/RDF 格式输出。
    /// 通常与 `execute_text()` 配合使用获取 XML 文本。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let xml = exiftool.query("photo.jpg")
    ///     .xml_format()
    ///     .execute_text()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn xml_format(mut self) -> Self {
        self.args.push("-X".to_string());
        self
    }

    /// 忽略次要错误
    ///
    /// 使用 `-m` 选项忽略次要错误和警告，继续处理。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let metadata = exiftool.query("photo.jpg")
    ///     .ignore_minor_errors()
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ignore_minor_errors(mut self) -> Self {
        self.args.push("-m".to_string());
        self
    }

    /// 按字母顺序排序输出
    ///
    /// 使用 `-sort` 选项对标签进行字母排序
    pub fn sort(mut self, yes: bool) -> Self {
        if yes {
            self.args.push("-sort".to_string());
        }
        self
    }

    /// 设置列表项分隔符
    ///
    /// 使用 `-sep` 选项设置列表项的分隔符字符串
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 使用逗号分隔列表项
    /// let metadata = exiftool.query("photo.jpg")
    ///     .separator(", ")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.args.push("-sep".to_string());
        self.args.push(sep.into());
        self
    }

    /// 启用快速模式
    ///
    /// 使用 `-fast` 选项提高元数据提取速度。
    /// 这会跳过某些处理步骤，可能遗漏某些信息。
    ///
    /// # 级别
    ///
    /// - `None` - 不使用快速模式（默认）
    /// - `Some(1)` - `-fast` 基础快速模式
    /// - `Some(2)` - `-fast2` 更激进的快速模式
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 使用快速模式处理大量文件
    /// let metadata = exiftool.query("photo.jpg")
    ///     .fast(Some(1))
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fast(mut self, level: Option<u8>) -> Self {
        match level {
            Some(1) => self.args.push("-fast".to_string()),
            Some(l) if l > 1 => self.args.push(format!("-fast{}", l)),
            _ => {}
        }
        self
    }

    /// 强制扫描 XMP 数据
    ///
    /// 使用 `-scanForXMP` 选项暴力扫描文件中的 XMP 数据
    pub fn scan_for_xmp(mut self, yes: bool) -> Self {
        if yes {
            self.args.push("-scanForXMP".to_string());
        }
        self
    }

    /// 设置 API 选项
    ///
    /// 使用 `-api` 选项设置 ExifTool API 选项。
    /// 常见选项包括：`QuickTimeUTC`, `SystemTags`, `largefilesupport`
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 启用 QuickTimeUTC
    /// let metadata = exiftool.query("video.mp4")
    ///     .api_option("QuickTimeUTC", None::<&str>)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn api_option(mut self, opt: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        let option = opt.into();
        self.args.push("-api".to_string());
        match value {
            Some(v) => self.args.push(format!("{}={}", option, v.into())),
            None => self.args.push(option),
        }
        self
    }

    /// 设置用户参数
    ///
    /// 使用 `-userParam` 选项设置用户参数，可在配置文件中使用
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 设置自定义参数
    /// let metadata = exiftool.query("photo.jpg")
    ///     .user_param("MyParam", Some("value"))
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn user_param(
        mut self,
        param: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        let param = param.into();
        self.args.push("-userParam".to_string());
        match value {
            Some(v) => self.args.push(format!("{}={}", param, v.into())),
            None => self.args.push(param),
        }
        self
    }

    /// 设置密码
    ///
    /// 使用 `-password` 选项处理受密码保护的文件
    ///
    /// # 安全性警告
    ///
    /// 密码将以纯文本形式传递给 ExifTool 进程。
    /// 在多用户系统中使用时请注意安全性。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 读取受密码保护的 PDF
    /// let metadata = exiftool.query("protected.pdf")
    ///     .password("secret123")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn password(mut self, passwd: impl Into<String>) -> Self {
        self.args.push("-password".to_string());
        self.args.push(passwd.into());
        self
    }

    /// 十进制显示标签 ID
    ///
    /// 使用 `-D` 选项以十进制格式显示标签 ID 编号
    pub fn decimal(mut self, yes: bool) -> Self {
        self.decimal = yes;
        self
    }

    /// 转义格式
    ///
    /// 使用 `-E`、`-ex` 或 `-ec` 选项转义标签值
    pub fn escape(mut self, format: EscapeFormat) -> Self {
        self.escape_format = Some(format);
        self
    }

    /// 强制打印
    ///
    /// 使用 `-f` 选项强制打印所有指定标签
    pub fn force_print(mut self, yes: bool) -> Self {
        self.force_print = yes;
        self
    }

    /// 打印组名
    ///
    /// 使用 `-G` 选项打印每个标签的组名
    pub fn group_names(mut self, level: Option<u8>) -> Self {
        self.group_names = level;
        self
    }

    /// HTML 格式
    ///
    /// 使用 `-h` 选项以 HTML 格式输出
    pub fn html_format(mut self, yes: bool) -> Self {
        self.html_format = yes;
        self
    }

    /// 十六进制显示
    ///
    /// 使用 `-H` 选项以十六进制显示标签 ID
    pub fn hex(mut self, yes: bool) -> Self {
        self.hex = yes;
        self
    }

    /// 长格式输出
    ///
    /// 使用 `-l` 选项以长格式（2行）输出
    pub fn long_format(mut self, yes: bool) -> Self {
        self.long_format = yes;
        self
    }

    /// Latin1 编码
    ///
    /// 使用 `-L` 选项使用 Windows Latin1 编码
    pub fn latin(mut self, yes: bool) -> Self {
        self.latin = yes;
        self
    }

    /// 短格式输出
    ///
    /// 使用 `-s` 或 `-S` 选项以短格式输出
    pub fn short_format(mut self, level: Option<u8>) -> Self {
        self.short_format = level;
        self
    }

    /// Tab 分隔格式
    ///
    /// 使用 `-t` 选项以 Tab 分隔格式输出
    pub fn tab_format(mut self, yes: bool) -> Self {
        self.tab_format = yes;
        self
    }

    /// 表格格式
    ///
    /// 使用 `-T` 选项以表格格式输出
    pub fn table_format(mut self, yes: bool) -> Self {
        self.table_format = yes;
        self
    }

    /// 文本输出到文件
    ///
    /// 使用 `-w` 选项将输出写入文件
    pub fn text_out(mut self, ext: impl Into<String>) -> Self {
        self.text_out = Some(ext.into());
        self
    }

    /// 标签输出到文件
    ///
    /// 使用 `-W` 选项为每个标签创建输出文件
    pub fn tag_out(mut self, format: impl Into<String>) -> Self {
        self.tag_out = Some(format.into());
        self
    }

    /// 标签输出扩展名过滤
    ///
    /// 使用 `-Wext` 选项指定 `-W` 输出的文件类型
    pub fn tag_out_ext(mut self, ext: impl Into<String>) -> Self {
        self.tag_out_ext.push(ext.into());
        self
    }

    /// 提取列表项
    ///
    /// 使用 `-listItem` 选项提取列表中的特定项
    pub fn list_item(mut self, index: u32) -> Self {
        self.list_item = Some(index);
        self
    }

    /// 文件处理顺序
    ///
    /// 使用 `-fileOrder` 选项设置文件处理顺序
    pub fn file_order(mut self, tag: impl Into<String>, descending: bool) -> Self {
        self.file_order = Some((tag.into(), descending));
        self
    }

    /// 静默模式
    ///
    /// 使用 `-q` 选项减少输出信息
    pub fn quiet(mut self, yes: bool) -> Self {
        self.quiet = yes;
        self
    }

    /// HTML二进制转储
    ///
    /// 使用 `-htmlDump` 选项生成HTML格式的二进制转储
    /// 可以指定可选的偏移量
    pub fn html_dump(mut self, offset: Option<u32>) -> Self {
        self.html_dump = offset;
        self
    }

    /// PHP数组格式输出
    ///
    /// 使用 `-php` 选项导出为PHP数组格式
    pub fn php_format(mut self, yes: bool) -> Self {
        self.php_format = yes;
        self
    }

    /// SVG plot格式输出
    ///
    /// 使用 `-plot` 选项输出为SVG plot文件
    pub fn plot_format(mut self, yes: bool) -> Self {
        self.plot_format = yes;
        self
    }

    /// 格式化为exiftool参数
    ///
    /// 使用 `-args` 选项将元数据格式化为exiftool参数格式
    pub fn args_format(mut self, yes: bool) -> Self {
        self.args_format = yes;
        self
    }

    /// 设置公共参数
    ///
    /// 使用 `-common_args` 选项定义在多个命令之间共享的参数
    pub fn common_args(mut self, args: &[impl AsRef<str>]) -> Self {
        for arg in args {
            self.common_args.push(arg.as_ref().to_string());
        }
        self
    }

    /// 输出文本到stdout或stderr
    ///
    /// 使用 `-echo` 选项在处理期间输出文本
    /// - `text`: 要输出的文本
    /// - `target`: 输出目标，None表示stdout，Some("stderr")表示stderr
    pub fn echo(mut self, text: impl Into<String>, target: Option<impl Into<String>>) -> Self {
        self.echo.push((text.into(), target.map(|t| t.into())));
        self
    }

    /// 保存错误文件名到文件
    ///
    /// 使用 `-efile` 选项将处理失败的文件名保存到指定文件
    pub fn efile(mut self, filename: impl Into<String>) -> Self {
        self.efile = Some(filename.into());
        self
    }

    /// 执行查询
    pub fn execute(self) -> Result<Metadata> {
        let args = self.build_args(true);

        // 发送命令并获取响应
        let response = self.exiftool.execute_raw(&args)?;

        // 解析响应
        self.parse_response(response)
    }

    /// 执行查询并返回 JSON
    pub fn execute_json(self) -> Result<serde_json::Value> {
        let args = self.build_args(true);
        let response = self.exiftool.execute_raw(&args)?;
        response.json()
    }

    /// 执行查询并反序列化为自定义类型
    pub fn execute_as<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let args = self.build_args(true);
        let response = self.exiftool.execute_raw(&args)?;
        response.json()
    }

    /// 执行查询并返回纯文本
    ///
    /// 当使用 `-p` (print_format) 选项时，
    /// 使用此方法获取纯文本输出而非 JSON
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// let output = exiftool.query("photo.jpg")
    ///     .print_format("$FileName: $DateTimeOriginal")
    ///     .execute_text()?;
    ///
    /// println!("{}", output);
    /// # Ok(())
    /// # }
    /// ```
    pub fn execute_text(self) -> Result<String> {
        let args = self.build_args(false);
        let response = self.exiftool.execute_raw(&args)?;
        Ok(response.text().trim().to_string())
    }

    /// 构建参数列表
    fn build_args(&self, json_output: bool) -> Vec<String> {
        let mut args = Vec::new();

        if json_output {
            args.push("-json".to_string());
        }

        // 分组选项
        if self.group_by_category {
            args.push("-g1".to_string());
        }

        // 未知标签
        if self.include_unknown {
            args.push("-u".to_string());
        }

        // 重复标签
        if self.include_duplicates {
            args.push("-a".to_string());
        }

        // 原始数值
        if self.raw_values {
            args.push("-n".to_string());
        }

        // 十进制显示
        if self.decimal {
            args.push("-D".to_string());
        }

        // 转义格式
        if let Some(format) = self.escape_format {
            let flag = match format {
                EscapeFormat::Html => "-E",
                EscapeFormat::Xml => "-ex",
                EscapeFormat::C => "-ec",
            };
            args.push(flag.to_string());
        }

        // 强制打印
        if self.force_print {
            args.push("-f".to_string());
        }

        // 组名
        if let Some(level) = self.group_names {
            if level == 1 {
                args.push("-G".to_string());
            } else {
                args.push(format!("-G{}", level));
            }
        }

        // HTML 格式
        if self.html_format {
            args.push("-h".to_string());
        }

        // 十六进制
        if self.hex {
            args.push("-H".to_string());
        }

        // 长格式
        if self.long_format {
            args.push("-l".to_string());
        }

        // Latin1 编码
        if self.latin {
            args.push("-L".to_string());
        }

        // 短格式
        if let Some(level) = self.short_format {
            if level == 0 {
                args.push("-S".to_string());
            } else {
                args.push(format!("-s{}", level));
            }
        }

        // Tab 格式
        if self.tab_format {
            args.push("-t".to_string());
        }

        // 表格格式
        if self.table_format {
            args.push("-T".to_string());
        }

        // 文本输出
        if let Some(ref ext) = self.text_out {
            args.push("-w".to_string());
            args.push(ext.clone());
        }

        // 标签输出
        if let Some(ref format) = self.tag_out {
            args.push("-W".to_string());
            args.push(format.clone());
        }

        // 标签输出扩展名
        for ext in &self.tag_out_ext {
            args.push("-Wext".to_string());
            args.push(ext.clone());
        }

        // 列表项
        if let Some(index) = self.list_item {
            args.push("-listItem".to_string());
            args.push(index.to_string());
        }

        // 文件顺序
        if let Some((ref tag, descending)) = self.file_order {
            let order = if descending { "-" } else { "" };
            args.push("-fileOrder".to_string());
            args.push(format!("{}{}", order, tag));
        }

        // 静默模式
        if self.quiet {
            args.push("-q".to_string());
        }

        // HTML二进制转储
        if let Some(offset) = self.html_dump {
            args.push(format!("-htmlDump{}", offset));
        }

        // PHP数组格式
        if self.php_format {
            args.push("-php".to_string());
        }

        // SVG plot格式
        if self.plot_format {
            args.push("-plot".to_string());
        }

        // 格式化为exiftool参数
        if self.args_format {
            args.push("-args".to_string());
        }

        // 公共参数
        for arg in &self.common_args {
            args.push("-common_args".to_string());
            args.push(arg.clone());
        }

        // echo输出
        for (text, target) in &self.echo {
            let option = match target {
                Some(t) if t == "stderr" => "-echo2",
                _ => "-echo",
            };
            args.push(option.to_string());
            args.push(text.clone());
        }

        // 错误文件
        if let Some(ref filename) = self.efile {
            args.push("-efile".to_string());
            args.push(filename.clone());
        }

        // 禁用复合标签
        if self.no_composite {
            args.push("-e".to_string());
        }

        // 提取嵌入文件
        if let Some(level) = self.extract_embedded {
            if level == 1 {
                args.push("-ee".to_string());
            } else {
                args.push(format!("-ee{}", level));
            }
        }

        // 文件扩展名过滤
        for ext in &self.extensions {
            args.push("-ext".to_string());
            args.push(ext.clone());
        }

        // 忽略目录
        for dir in &self.ignore_dirs {
            args.push("-i".to_string());
            args.push(dir.clone());
        }

        // 递归处理
        if self.recursive {
            args.push("-r".to_string());
        }

        // 进度显示
        if let Some(interval) = self.progress_interval {
            let progress_arg = if let Some(ref title) = self.progress_title {
                format!("-progress{}:{}", interval, title)
            } else {
                format!("-progress{}", interval)
            };
            args.push(progress_arg);
        }

        // 添加自定义参数
        args.extend(self.args.clone());

        // 特定标签
        for tag in &self.specific_tags {
            args.push(format!("-{}", tag));
        }

        // 排除标签
        for tag in &self.excluded_tags {
            args.push(format!("--{}", tag));
        }

        // 文件路径
        args.push(self.path.to_string_lossy().to_string());

        args
    }

    /// 解析响应
    fn parse_response(&self, response: Response) -> Result<Metadata> {
        parse_query_response(response)
    }
}

/// 异步查询执行方法
///
/// 在 `async` feature 开启时，为 `QueryBuilder` 提供异步执行方法。
/// Builder 的链式调用仍然是同步的（仅收集参数），
/// 只有最终的 `async_execute` 才通过 `spawn_blocking` 异步执行。
#[cfg(feature = "async")]
impl QueryBuilder<'_> {
    /// 异步执行查询
    ///
    /// 内部先收集参数（纯数据），然后在阻塞线程池中执行 ExifTool 命令。
    /// 适用于 `AsyncExifTool::query_builder()` 返回的构建器。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::AsyncExifTool;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let async_et = AsyncExifTool::new()?;
    ///
    /// let metadata = async_et.query_builder("photo.jpg")
    ///     .binary()
    ///     .group_headings(None)
    ///     .tag("Make")
    ///     .async_execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn async_execute(self) -> Result<Metadata> {
        // 先收集参数（纯数据，无引用）
        let args = self.build_args(true);
        // clone ExifTool（内部是 Arc，开销极小）
        let exiftool = self.exiftool.clone();
        tokio::task::spawn_blocking(move || {
            let response = exiftool.execute_raw(&args)?;
            parse_query_response(response)
        })
        .await
        .map_err(|e| Error::process(format!("异步查询任务执行失败: {}", e)))?
    }

    /// 异步执行查询并返回 JSON
    ///
    /// 与 `execute_json` 相同，但在阻塞线程池中异步执行。
    pub async fn async_execute_json(self) -> Result<serde_json::Value> {
        let args = self.build_args(true);
        let exiftool = self.exiftool.clone();
        tokio::task::spawn_blocking(move || {
            let response = exiftool.execute_raw(&args)?;
            response.json()
        })
        .await
        .map_err(|e| Error::process(format!("异步查询任务执行失败: {}", e)))?
    }

    /// 异步执行查询并反序列化为自定义类型
    ///
    /// 与 `execute_as` 相同，但在阻塞线程池中异步执行。
    pub async fn async_execute_as<T: serde::de::DeserializeOwned + Send + 'static>(
        self,
    ) -> Result<T> {
        let args = self.build_args(true);
        let exiftool = self.exiftool.clone();
        tokio::task::spawn_blocking(move || {
            let response = exiftool.execute_raw(&args)?;
            response.json()
        })
        .await
        .map_err(|e| Error::process(format!("异步查询任务执行失败: {}", e)))?
    }

    /// 异步执行查询并返回纯文本
    ///
    /// 与 `execute_text` 相同，但在阻塞线程池中异步执行。
    pub async fn async_execute_text(self) -> Result<String> {
        let args = self.build_args(false);
        let exiftool = self.exiftool.clone();
        tokio::task::spawn_blocking(move || {
            let response = exiftool.execute_raw(&args)?;
            Ok(response.text().trim().to_string())
        })
        .await
        .map_err(|e| Error::process(format!("异步查询任务执行失败: {}", e)))?
    }
}

/// 解析查询响应为 Metadata
///
/// 从 ExifTool 的 JSON 响应中解析出 Metadata 对象。
/// 此函数被同步 `execute` 和异步 `async_execute` 共用。
fn parse_query_response(response: Response) -> Result<Metadata> {
    if response.is_error() {
        return Err(Error::process(
            response
                .error_message()
                .unwrap_or_else(|| "Unknown error".to_string()),
        ));
    }

    let json_value: Vec<serde_json::Value> = response.json()?;

    if json_value.is_empty() {
        return Ok(Metadata::new());
    }

    // 取第一个（也是唯一一个）结果
    let metadata: Metadata = serde_json::from_value(json_value[0].clone())?;

    Ok(metadata)
}

/// 批量查询构建器
pub struct BatchQueryBuilder<'et> {
    exiftool: &'et ExifTool,
    paths: Vec<PathBuf>,
    args: Vec<String>,
    include_unknown: bool,
    include_duplicates: bool,
    raw_values: bool,
    group_by_category: bool,
    specific_tags: Vec<String>,
}

impl<'et> BatchQueryBuilder<'et> {
    /// 创建新的批量查询构建器
    pub(crate) fn new(exiftool: &'et ExifTool, paths: Vec<PathBuf>) -> Self {
        Self {
            exiftool,
            paths,
            args: Vec::new(),
            include_unknown: false,
            include_duplicates: false,
            raw_values: false,
            group_by_category: false,
            specific_tags: Vec::new(),
        }
    }

    /// 包含未知标签
    pub fn include_unknown(mut self, yes: bool) -> Self {
        self.include_unknown = yes;
        self
    }

    /// 包含重复标签
    pub fn include_duplicates(mut self, yes: bool) -> Self {
        self.include_duplicates = yes;
        self
    }

    /// 显示原始数值
    pub fn raw_values(mut self, yes: bool) -> Self {
        self.raw_values = yes;
        self
    }

    /// 按类别分组
    pub fn group_by_category(mut self, yes: bool) -> Self {
        self.group_by_category = yes;
        self
    }

    /// 添加特定标签查询
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.specific_tags.push(tag.into());
        self
    }

    /// 添加多个标签查询
    pub fn tags(mut self, tags: &[impl AsRef<str>]) -> Self {
        for tag in tags {
            self.specific_tags.push(tag.as_ref().to_string());
        }
        self
    }

    /// 设置 GPS 坐标格式
    pub fn coord_format(mut self, format: impl Into<String>) -> Self {
        self.args.push("-c".to_string());
        self.args.push(format.into());
        self
    }

    /// 设置日期/时间格式
    pub fn date_format(mut self, format: impl Into<String>) -> Self {
        self.args.push("-d".to_string());
        self.args.push(format.into());
        self
    }

    /// 设置密码
    pub fn password(mut self, passwd: impl Into<String>) -> Self {
        self.args.push("-password".to_string());
        self.args.push(passwd.into());
        self
    }

    /// 启用快速模式
    pub fn fast(mut self, level: Option<u8>) -> Self {
        match level {
            Some(1) => self.args.push("-fast".to_string()),
            Some(l) if l > 1 => self.args.push(format!("-fast{}", l)),
            _ => {}
        }
        self
    }

    /// 强制扫描 XMP 数据
    pub fn scan_for_xmp(mut self, yes: bool) -> Self {
        if yes {
            self.args.push("-scanForXMP".to_string());
        }
        self
    }

    /// 设置 API 选项
    pub fn api_option(mut self, opt: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        let option = opt.into();
        self.args.push("-api".to_string());
        match value {
            Some(v) => self.args.push(format!("{}={}", option, v.into())),
            None => self.args.push(option),
        }
        self
    }

    /// 设置用户参数
    pub fn user_param(
        mut self,
        param: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        let param = param.into();
        self.args.push("-userParam".to_string());
        match value {
            Some(v) => self.args.push(format!("{}={}", param, v.into())),
            None => self.args.push(param),
        }
        self
    }

    /// 设置自定义打印格式
    pub fn print_format(mut self, format: impl Into<String>) -> Self {
        self.args.push("-p".to_string());
        self.args.push(format.into());
        self
    }

    /// 按字母顺序排序输出
    pub fn sort(mut self, yes: bool) -> Self {
        if yes {
            self.args.push("-sort".to_string());
        }
        self
    }

    /// 设置列表项分隔符
    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.args.push("-sep".to_string());
        self.args.push(sep.into());
        self
    }

    /// 执行批量查询
    pub fn execute(self) -> Result<Vec<(PathBuf, Metadata)>> {
        if self.paths.is_empty() {
            return Ok(Vec::new());
        }

        let args = self.build_args();
        let response = self.exiftool.execute_raw(&args)?;

        // 解析 JSON 数组响应
        let json_values: Vec<serde_json::Value> = response.json()?;

        let mut results = Vec::with_capacity(json_values.len());

        for (i, json) in json_values.into_iter().enumerate() {
            let path = self.paths.get(i).cloned().unwrap_or_default();
            let metadata: Metadata = serde_json::from_value(json)?;
            results.push((path, metadata));
        }

        Ok(results)
    }

    /// 构建参数列表
    fn build_args(&self) -> Vec<String> {
        let mut args = vec!["-json".to_string()];

        if self.group_by_category {
            args.push("-g1".to_string());
        }

        if self.include_unknown {
            args.push("-u".to_string());
        }

        if self.include_duplicates {
            args.push("-a".to_string());
        }

        if self.raw_values {
            args.push("-n".to_string());
        }

        args.extend(self.args.clone());

        for tag in &self.specific_tags {
            args.push(format!("-{}", tag));
        }

        // 添加所有文件路径
        for path in &self.paths {
            args.push(path.to_string_lossy().to_string());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[test]
    fn test_response_warning_not_error() {
        let response = crate::Response::new(vec!["Warning: test".to_string()]);
        assert!(!response.is_error());
    }

    #[test]
    fn test_query_builder_args() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let args = exiftool
            .query("photo.jpg")
            .charset("utf8")
            .lang("zh")
            .coord_format("%.6f")
            .date_format("%Y-%m-%d")
            .print_format("$FileName")
            .separator(",")
            .api_option("QuickTimeUTC", Some("1"))
            .user_param("k", Some("v"))
            .password("p")
            .build_args(true);

        assert!(args.windows(2).any(|w| w == ["-charset", "utf8"]));
        assert!(args.windows(2).any(|w| w == ["-lang", "zh"]));
        assert!(args.windows(2).any(|w| w == ["-c", "%.6f"]));
        assert!(args.windows(2).any(|w| w == ["-d", "%Y-%m-%d"]));
        assert!(args.windows(2).any(|w| w == ["-p", "$FileName"]));
        assert!(args.windows(2).any(|w| w == ["-sep", ","]));
        assert!(args.windows(2).any(|w| w == ["-api", "QuickTimeUTC=1"]));
        assert!(args.windows(2).any(|w| w == ["-userParam", "k=v"]));
        assert!(args.windows(2).any(|w| w == ["-password", "p"]));
    }

    /// 测试 args_file 方法：验证 -@ 参数文件选项构建正确
    #[test]
    fn test_args_file() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        // 创建临时参数文件，写入 -FileName
        let tmp_dir = std::env::temp_dir();
        let args_path = tmp_dir.join("exiftool_test_args.txt");
        std::fs::write(&args_path, "-FileName\n").expect("写入临时参数文件失败");

        let args = exiftool
            .query("photo.jpg")
            .args_file(args_path.to_string_lossy().as_ref())
            .build_args(false);

        // 验证 args 包含 ["-@", "<路径>"]
        assert!(
            args.windows(2)
                .any(|w| w[0] == "-@" && w[1] == args_path.to_string_lossy().as_ref()),
            "参数列表应包含 -@ 和参数文件路径，实际: {:?}",
            args
        );

        // 清理临时文件
        let _ = std::fs::remove_file(&args_path);
    }

    /// 测试 csv_delimiter 方法：验证 -csvDelim 参数构建正确
    #[test]
    fn test_csv_delimiter() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        let args = exiftool
            .query("photo.jpg")
            .csv_delimiter(",")
            .build_args(false);

        assert!(
            args.windows(2).any(|w| w == ["-csvDelim", ","]),
            "参数列表应包含 [\"-csvDelim\", \",\"]，实际: {:?}",
            args
        );
    }

    /// 测试 unknown_binary 方法：验证 -U 参数构建正确
    #[test]
    fn test_unknown_binary() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        let args = exiftool
            .query("photo.jpg")
            .unknown_binary()
            .build_args(false);

        assert!(
            args.iter().any(|a| a == "-U"),
            "参数列表应包含 \"-U\"，实际: {:?}",
            args
        );
    }

    /// 测试 recursive_hidden 方法：验证 -r. 参数构建正确
    #[test]
    fn test_recursive_hidden() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        let args = exiftool
            .query("/photos")
            .recursive_hidden()
            .build_args(false);

        assert!(
            args.iter().any(|a| a == "-r."),
            "参数列表应包含 \"-r.\"，实际: {:?}",
            args
        );
    }

    /// 测试 source_file 方法：验证 -srcfile 参数构建正确
    #[test]
    fn test_source_file() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        let args = exiftool
            .query("photo.jpg")
            .source_file("%d%f.xmp")
            .build_args(false);

        assert!(
            args.windows(2).any(|w| w == ["-srcfile", "%d%f.xmp"]),
            "参数列表应包含 [\"-srcfile\", \"%d%f.xmp\"]，实际: {:?}",
            args
        );
    }
}
