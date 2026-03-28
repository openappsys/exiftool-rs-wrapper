//! 写入操作构建器

use crate::ExifTool;
use crate::error::{Error, Result};
use crate::types::TagId;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 写入模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode {
    /// w - 写入标签（默认值）
    Write,
    /// c - 仅创建标签（不修改现有标签）
    Create,
    /// wc - 写入或创建
    WriteCreate,
}

impl WriteMode {
    /// 获取模式字符串
    fn as_str(&self) -> &'static str {
        match self {
            WriteMode::Write => "w",
            WriteMode::Create => "c",
            WriteMode::WriteCreate => "wc",
        }
    }
}

/// 写入构建器
pub struct WriteBuilder<'et> {
    exiftool: &'et ExifTool,
    path: PathBuf,
    tags: HashMap<String, String>,
    delete_tags: Vec<String>,
    overwrite_original: bool,
    backup: bool,
    output_path: Option<PathBuf>,
    condition: Option<String>,
    ignore_minor_errors: bool,
    preserve_time: bool,
    quiet: bool,
    zip_compression: bool,
    fix_base_enabled: bool,
    fix_base_offset: Option<i64>,
    raw_args: Vec<String>,
}

impl<'et> WriteBuilder<'et> {
    /// 创建新的写入构建器
    pub(crate) fn new<P: AsRef<Path>>(exiftool: &'et ExifTool, path: P) -> Self {
        Self {
            exiftool,
            path: path.as_ref().to_path_buf(),
            tags: HashMap::new(),
            delete_tags: Vec::new(),
            overwrite_original: false,
            backup: true,
            output_path: None,
            condition: None,
            ignore_minor_errors: false,
            preserve_time: false,
            quiet: false,
            zip_compression: false,
            fix_base_enabled: false,
            fix_base_offset: None,
            raw_args: Vec::new(),
        }
    }

    /// 设置标签值
    pub fn tag(mut self, tag: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(tag.into(), value.into());
        self
    }

    /// 设置标签值（使用 TagId）
    pub fn tag_id(self, tag: TagId, value: impl Into<String>) -> Self {
        self.tag(tag.name(), value)
    }

    /// 设置多个标签
    pub fn tags(mut self, tags: HashMap<impl Into<String>, impl Into<String>>) -> Self {
        for (k, v) in tags {
            self.tags.insert(k.into(), v.into());
        }
        self
    }

    /// 追加值到标签（`-TAG+=VALUE`）
    ///
    /// 使用 `+=` 运算符将值追加到现有标签值后面。
    /// 适用于列表类型的标签，如 Keywords 等。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 追加关键词到现有列表
    /// exiftool.write("photo.jpg")
    ///     .tag_append("Keywords", "landscape")
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tag_append(mut self, tag: impl Into<String>, value: impl Into<String>) -> Self {
        self.raw_args
            .push(format!("-{}+={}", tag.into(), value.into()));
        self
    }

    /// 从标签中移除值（`-TAG-=VALUE`）
    ///
    /// 使用 `-=` 运算符从现有标签值中移除指定值。
    /// 适用于列表类型的标签。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 从关键词列表中移除某个关键词
    /// exiftool.write("photo.jpg")
    ///     .tag_remove("Keywords", "old-keyword")
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tag_remove(mut self, tag: impl Into<String>, value: impl Into<String>) -> Self {
        self.raw_args
            .push(format!("-{}-={}", tag.into(), value.into()));
        self
    }

    /// 前置值到标签（`-TAG^=VALUE`）
    ///
    /// 使用 `^=` 运算符将值前置到现有标签值之前。
    /// 适用于列表类型的标签。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 在关键词列表前面插入关键词
    /// exiftool.write("photo.jpg")
    ///     .tag_prepend("Keywords", "important")
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tag_prepend(mut self, tag: impl Into<String>, value: impl Into<String>) -> Self {
        self.raw_args
            .push(format!("-{}^={}", tag.into(), value.into()));
        self
    }

    /// 从文件读取值写入标签（`-TAG<=FILE`）
    ///
    /// 使用 `<=` 运算符从指定文件中读取数据作为标签值。
    /// 常用于写入二进制数据（如缩略图、预览图）。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 从文件读取缩略图写入
    /// exiftool.write("photo.jpg")
    ///     .tag_from_file("ThumbnailImage", "thumb.jpg")
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tag_from_file(mut self, tag: impl Into<String>, file_path: impl Into<String>) -> Self {
        self.raw_args
            .push(format!("-{}<={}", tag.into(), file_path.into()));
        self
    }

    /// 追加从文件读取的值到标签（`-TAG+<=FILE`）
    ///
    /// 使用 `+<=` 运算符从指定文件中读取数据追加到现有标签值。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 从文件追加数据到标签
    /// exiftool.write("photo.jpg")
    ///     .tag_append_from_file("Comment", "comment.txt")
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tag_append_from_file(
        mut self,
        tag: impl Into<String>,
        file_path: impl Into<String>,
    ) -> Self {
        self.raw_args
            .push(format!("-{}+<={}", tag.into(), file_path.into()));
        self
    }

    /// 删除标签
    pub fn delete(mut self, tag: impl Into<String>) -> Self {
        self.delete_tags.push(tag.into());
        self
    }

    /// 删除标签（使用 TagId）
    pub fn delete_id(self, tag: TagId) -> Self {
        self.delete(tag.name())
    }

    /// 覆盖原始文件（不创建备份）
    pub fn overwrite_original(mut self, yes: bool) -> Self {
        self.overwrite_original = yes;
        self
    }

    /// 创建备份
    pub fn backup(mut self, yes: bool) -> Self {
        self.backup = yes;
        self
    }

    /// 输出到不同文件
    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// 设置条件（仅在条件满足时写入）
    pub fn condition(mut self, expr: impl Into<String>) -> Self {
        self.condition = Some(expr.into());
        self
    }

    /// 添加原始参数（高级用法）
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.raw_args.push(arg.into());
        self
    }

    /// 设置写入模式
    ///
    /// 使用 `-wm` 选项设置写入/创建标签的模式
    ///
    /// # 模式
    ///
    /// - `WriteMode::Write` (w) - 写入标签（默认）
    /// - `WriteMode::Create` (c) - 仅创建标签（不修改现有标签）
    /// - `WriteMode::WriteCreate` (wc) - 写入或创建
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::{ExifTool, WriteMode};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 仅创建新标签，不修改现有标签
    /// exiftool.write("photo.jpg")
    ///     .tag("NewTag", "value")
    ///     .write_mode(WriteMode::Create)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_mode(mut self, mode: WriteMode) -> Self {
        self.raw_args.push("-wm".to_string());
        self.raw_args.push(mode.as_str().to_string());
        self
    }

    /// 设置密码
    ///
    /// 使用 `-password` 选项处理受密码保护的文件
    ///
    /// # 安全性警告
    ///
    /// 密码将以纯文本形式传递给 ExifTool 进程。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 写入受密码保护的 PDF
    /// exiftool.write("protected.pdf")
    ///     .tag("Title", "New Title")
    ///     .password("secret123")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn password(mut self, passwd: impl Into<String>) -> Self {
        self.raw_args.push("-password".to_string());
        self.raw_args.push(passwd.into());
        self
    }

    /// 设置列表项分隔符
    ///
    /// 使用 `-sep` 选项设置列表项的分隔符字符串
    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.raw_args.push("-sep".to_string());
        self.raw_args.push(sep.into());
        self
    }

    /// 设置 API 选项
    ///
    /// 使用 `-api` 选项设置 ExifTool API 选项
    pub fn api_option(mut self, opt: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        let option = opt.into();
        self.raw_args.push("-api".to_string());
        match value {
            Some(v) => self.raw_args.push(format!("{}={}", option, v.into())),
            None => self.raw_args.push(option),
        }
        self
    }

    /// 设置用户参数
    ///
    /// 使用 `-userParam` 选项设置用户参数
    pub fn user_param(
        mut self,
        param: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        let param = param.into();
        self.raw_args.push("-userParam".to_string());
        match value {
            Some(v) => self.raw_args.push(format!("{}={}", param, v.into())),
            None => self.raw_args.push(param),
        }
        self
    }

    /// 忽略次要错误
    ///
    /// 使用 `-m` 选项忽略次要错误和警告，继续处理其他文件。
    /// 这在批量处理时很有用，可以避免单个文件错误导致整个操作失败。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 批量写入时忽略次要错误
    /// exiftool.write("photo.jpg")
    ///     .tag("Copyright", "© 2026")
    ///     .ignore_minor_errors(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ignore_minor_errors(mut self, yes: bool) -> Self {
        self.ignore_minor_errors = yes;
        self
    }

    /// 保留文件修改时间
    ///
    /// 使用 `-P` 选项保留文件的原始修改时间。
    /// 默认情况下，ExifTool 会更新文件的修改时间为当前时间。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 写入元数据但保留原始修改时间
    /// exiftool.write("photo.jpg")
    ///     .tag("Copyright", "© 2026")
    ///     .preserve_time(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn preserve_time(mut self, yes: bool) -> Self {
        self.preserve_time = yes;
        self
    }

    /// 静默模式
    ///
    /// 使用 `-q` 选项启用静默模式，减少输出信息。
    /// 可以使用多次来增加静默程度。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 静默模式下写入
    /// exiftool.write("photo.jpg")
    ///     .tag("Copyright", "© 2026")
    ///     .quiet(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn quiet(mut self, yes: bool) -> Self {
        self.quiet = yes;
        self
    }

    /// 启用 ZIP 压缩
    ///
    /// 使用 `-z` 选项读写压缩的元数据信息。
    /// 某些文件格式支持压缩元数据存储。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 使用压缩写入元数据
    /// exiftool.write("photo.jpg")
    ///     .tag("Copyright", "© 2026")
    ///     .zip_compression(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn zip_compression(mut self, yes: bool) -> Self {
        self.zip_compression = yes;
        self
    }

    /// 修复 MakerNotes 偏移
    ///
    /// 使用 `-F` 选项修复 MakerNotes 的基准偏移。
    /// 这在处理某些损坏或格式异常的图像文件时很有用。
    ///
    /// # 参数
    ///
    /// - `offset` - 可选的偏移量修正值（以字节为单位）
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 自动修复 MakerNotes 偏移
    /// exiftool.write("photo.jpg")
    ///     .tag("Copyright", "© 2026")
    ///     .fix_base(None)
    ///     .execute()?;
    ///
    /// // 指定偏移量修复
    /// exiftool.write("photo.jpg")
    ///     .fix_base(Some(1024))
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fix_base(mut self, offset: Option<i64>) -> Self {
        self.fix_base_enabled = true;
        self.fix_base_offset = offset;
        self
    }

    /// 修复 MakerNotes 偏移（带指定偏移量）
    ///
    /// 使用 `-FOFFSET` 选项指定具体偏移量修复 MakerNotes。
    /// 这是 `fix_base(Some(offset))` 的便捷方法。
    pub fn fix_base_offset(mut self, offset: i64) -> Self {
        self.fix_base_enabled = true;
        self.fix_base_offset = Some(offset);
        self
    }

    /// 全局时间偏移
    ///
    /// 对应 ExifTool 的 `-globalTimeShift` 选项，对所有日期/时间标签
    /// 应用统一的时间偏移。格式为 `[+|-]Y:M:D H:M:S`。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 将所有时间标签向前偏移 1 小时
    /// exiftool.write("photo.jpg")
    ///     .global_time_shift("+0:0:0 1:0:0")
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn global_time_shift(mut self, shift: impl Into<String>) -> Self {
        self.raw_args.push("-globalTimeShift".to_string());
        self.raw_args.push(shift.into());
        self
    }

    /// 日期/时间偏移
    ///
    /// 示例: `.offset("DateTimeOriginal", "+1:0:0 0:0:0")` 表示增加 1 天
    pub fn offset(self, tag: impl Into<String>, offset: impl Into<String>) -> Self {
        let tag = tag.into();
        let offset = offset.into();
        self.arg(format!("-{}+={}", tag, offset))
    }

    /// 从文件复制标签
    ///
    /// 从源文件复制所有标签到目标文件
    pub fn copy_from<P: AsRef<Path>>(mut self, source: P) -> Self {
        self.raw_args.push("-tagsFromFile".to_string());
        self.raw_args
            .push(source.as_ref().to_string_lossy().to_string());
        self
    }

    /// 从文件复制标签（带重定向映射）
    ///
    /// 使用 `-tagsFromFile SRCFILE` 配合 `-DSTTAG<SRCTAG` 重定向语法。
    /// 可以指定源标签到目标标签的映射关系。
    ///
    /// # 参数
    ///
    /// - `source` - 源文件路径
    /// - `redirects` - 标签重定向列表，每项为 `(目标标签, 源标签)`。
    ///   生成 `-DSTTAG<SRCTAG` 参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 从源文件复制 Artist 到 Author，DateTimeOriginal 到 CreateDate
    /// exiftool.write("target.jpg")
    ///     .copy_from_with_redirect("source.jpg", &[
    ///         ("Author", "Artist"),
    ///         ("CreateDate", "DateTimeOriginal"),
    ///     ])
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_from_with_redirect<P: AsRef<Path>>(
        mut self,
        source: P,
        redirects: &[(&str, &str)],
    ) -> Self {
        self.raw_args.push("-tagsFromFile".to_string());
        self.raw_args
            .push(source.as_ref().to_string_lossy().to_string());
        for (dst, src) in redirects {
            self.raw_args.push(format!("-{}<{}", dst, src));
        }
        self
    }

    /// 从文件复制标签（带追加重定向）
    ///
    /// 使用 `-+DSTTAG<SRCTAG` 追加方式复制标签。
    ///
    /// # 参数
    ///
    /// - `source` - 源文件路径
    /// - `redirects` - 标签重定向列表，每项为 `(目标标签, 源标签)`。
    ///   生成 `-+DSTTAG<SRCTAG` 参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 从源文件追加 Keywords
    /// exiftool.write("target.jpg")
    ///     .copy_from_with_append("source.jpg", &[
    ///         ("Keywords", "Keywords"),
    ///     ])
    ///     .overwrite_original(true)
    ///     .execute()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_from_with_append<P: AsRef<Path>>(
        mut self,
        source: P,
        redirects: &[(&str, &str)],
    ) -> Self {
        self.raw_args.push("-tagsFromFile".to_string());
        self.raw_args
            .push(source.as_ref().to_string_lossy().to_string());
        for (dst, src) in redirects {
            self.raw_args.push(format!("-+{}<{}", dst, src));
        }
        self
    }

    /// 条件过滤（带编号）
    ///
    /// 使用 `-ifNUM` 选项设置条件过滤。
    /// `-if2` 在第一个条件失败时仍然继续检查。
    ///
    /// # 参数
    ///
    /// - `num` - 条件编号（如 2 表示 `-if2`）
    /// - `expr` - 条件表达式
    pub fn condition_num(mut self, num: u8, expr: impl Into<String>) -> Self {
        self.raw_args.push(format!("-if{}", num));
        self.raw_args.push(expr.into());
        self
    }

    /// 详细模式
    ///
    /// 使用 `-v` 或 `-vNUM` 选项设置写入时的详细输出级别。
    pub fn verbose(mut self, level: Option<u8>) -> Self {
        match level {
            Some(n) => self.raw_args.push(format!("-v{}", n)),
            None => self.raw_args.push("-v".to_string()),
        }
        self
    }

    /// 自定义打印格式（不追加换行符）
    ///
    /// 使用 `-p-` 选项按指定格式打印输出，不自动追加换行。
    pub fn print_format_no_newline(mut self, format: impl Into<String>) -> Self {
        self.raw_args.push("-p-".to_string());
        self.raw_args.push(format.into());
        self
    }

    /// 自定义打印格式
    ///
    /// 使用 `-p` 选项按指定格式打印输出。
    pub fn print_format(mut self, format: impl Into<String>) -> Self {
        self.raw_args.push("-p".to_string());
        self.raw_args.push(format.into());
        self
    }

    /// 保存错误文件名到文件（带级别和强制标志）
    ///
    /// 使用 `-efileNUM` 或 `-efile!` 或 `-efileNUM!` 变体。
    ///
    /// # 参数
    ///
    /// - `filename` - 输出文件路径
    /// - `num` - 可选级别（2、3 等），`None` 表示默认级别
    /// - `force` - 是否使用 `!` 后缀（强制覆盖）
    pub fn efile_variant(
        mut self,
        filename: impl Into<String>,
        num: Option<u8>,
        force: bool,
    ) -> Self {
        let num_str = num.map_or(String::new(), |n| n.to_string());
        let force_str = if force { "!" } else { "" };
        self.raw_args
            .push(format!("-efile{}{}", num_str, force_str));
        self.raw_args.push(filename.into());
        self
    }

    /// 导入 JSON 文件中的标签
    ///
    /// 使用 `-j=JSONFILE` 选项从 JSON 文件中导入标签数据。
    pub fn json_import(mut self, path: impl Into<String>) -> Self {
        self.raw_args.push(format!("-j={}", path.into()));
        self
    }

    /// 追加导入 JSON 文件中的标签
    ///
    /// 使用 `-j+=JSONFILE` 选项从 JSON 文件中追加导入标签数据。
    pub fn json_append(mut self, path: impl Into<String>) -> Self {
        self.raw_args.push(format!("-j+={}", path.into()));
        self
    }

    /// 导入 CSV 文件中的标签
    ///
    /// 使用 `-csv=CSVFILE` 选项从 CSV 文件中导入标签数据。
    pub fn csv_import(mut self, path: impl Into<String>) -> Self {
        self.raw_args.push(format!("-csv={}", path.into()));
        self
    }

    /// 追加导入 CSV 文件中的标签
    ///
    /// 使用 `-csv+=CSVFILE` 选项从 CSV 文件中追加导入标签数据。
    pub fn csv_append(mut self, path: impl Into<String>) -> Self {
        self.raw_args.push(format!("-csv+={}", path.into()));
        self
    }

    /// 文本输出到文件（追加模式）
    ///
    /// 使用 `-w+` 选项将输出追加到已有文件。
    pub fn text_out_append(mut self, ext: impl Into<String>) -> Self {
        self.raw_args.push("-w+".to_string());
        self.raw_args.push(ext.into());
        self
    }

    /// 文本输出到文件（仅创建新文件）
    ///
    /// 使用 `-w!` 选项将输出写入新文件，但不覆盖已有文件。
    pub fn text_out_create(mut self, ext: impl Into<String>) -> Self {
        self.raw_args.push("-w!".to_string());
        self.raw_args.push(ext.into());
        self
    }

    /// 标签输出到文件（追加模式）
    ///
    /// 使用 `-W+` 选项为每个标签创建输出文件，追加到已有文件。
    pub fn tag_out_append(mut self, format: impl Into<String>) -> Self {
        self.raw_args.push("-W+".to_string());
        self.raw_args.push(format.into());
        self
    }

    /// 标签输出到文件（仅创建新文件）
    ///
    /// 使用 `-W!` 选项为每个标签创建输出文件，但不覆盖已有文件。
    pub fn tag_out_create(mut self, format: impl Into<String>) -> Self {
        self.raw_args.push("-W!".to_string());
        self.raw_args.push(format.into());
        self
    }

    /// 追加扩展名过滤
    ///
    /// 使用 `-ext+` 选项追加文件扩展名过滤。
    pub fn extension_add(mut self, ext: impl Into<String>) -> Self {
        self.raw_args.push("-ext+".to_string());
        self.raw_args.push(ext.into());
        self
    }

    /// 文件扩展名过滤
    ///
    /// 使用 `-ext` 选项只处理指定扩展名的文件。
    pub fn extension(mut self, ext: impl Into<String>) -> Self {
        self.raw_args.push("-ext".to_string());
        self.raw_args.push(ext.into());
        self
    }

    /// 递归处理子目录
    ///
    /// 使用 `-r` 选项递归处理目录中的所有文件。
    pub fn recursive(mut self, yes: bool) -> Self {
        if yes {
            self.raw_args.push("-r".to_string());
        }
        self
    }

    /// 递归处理子目录（包含隐藏目录）
    ///
    /// 使用 `-r.` 选项递归处理时包含以 `.` 开头的隐藏目录。
    pub fn recursive_hidden(mut self) -> Self {
        self.raw_args.push("-r.".to_string());
        self
    }

    /// 执行写入操作
    pub fn execute(self) -> Result<WriteResult> {
        let args = self.build_args();
        let response = self.exiftool.execute_raw(&args)?;
        parse_write_response(response, self.path)
    }

    /// 构建参数列表
    fn build_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        // 覆盖原始文件
        if self.overwrite_original {
            args.push("-overwrite_original".to_string());
        }

        // 不创建备份
        if !self.backup {
            args.push("-overwrite_original_in_place".to_string());
        }

        // 输出到不同文件
        if let Some(ref output) = self.output_path {
            args.push("-o".to_string());
            args.push(output.to_string_lossy().to_string());
        }

        // 条件
        if let Some(ref condition) = self.condition {
            args.push("-if".to_string());
            args.push(condition.clone());
        }

        // 忽略次要错误
        if self.ignore_minor_errors {
            args.push("-m".to_string());
        }

        // 保留文件修改时间
        if self.preserve_time {
            args.push("-P".to_string());
        }

        // 静默模式
        if self.quiet {
            args.push("-q".to_string());
        }

        // ZIP 压缩
        if self.zip_compression {
            args.push("-z".to_string());
        }

        // 修复 MakerNotes 偏移
        if self.fix_base_enabled {
            match self.fix_base_offset {
                Some(offset) => args.push(format!("-F{}", offset)),
                None => args.push("-F".to_string()),
            }
        }

        // 原始参数
        args.extend(self.raw_args.clone());

        // 删除标签
        for tag in &self.delete_tags {
            args.push(format!("-{}=", tag));
        }

        // 标签写入
        for (tag, value) in &self.tags {
            args.push(format!("-{}={}", tag, value));
        }

        // 文件路径
        args.push(self.path.to_string_lossy().to_string());

        args
    }
}

/// 异步写入执行方法
///
/// 在 `async` feature 开启时，为 `WriteBuilder` 提供异步执行方法。
/// Builder 的链式调用仍然是同步的（仅收集参数），
/// 只有最终的 `async_execute` 才通过 `spawn_blocking` 异步执行。
#[cfg(feature = "async")]
impl WriteBuilder<'_> {
    /// 异步执行写入操作
    ///
    /// 内部先收集参数（纯数据），然后在阻塞线程池中执行 ExifTool 命令。
    /// 适用于 `AsyncExifTool::write_builder()` 返回的构建器。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::AsyncExifTool;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let async_et = AsyncExifTool::new()?;
    ///
    /// let result = async_et.write_builder("photo.jpg")
    ///     .tag("Artist", "Photographer")
    ///     .tag("Copyright", "© 2026")
    ///     .overwrite_original(true)
    ///     .async_execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn async_execute(self) -> Result<WriteResult> {
        // 先收集参数（纯数据，无引用）
        let args = self.build_args();
        let path = self.path.clone();
        // clone ExifTool（内部是 Arc，开销极小）
        let exiftool = self.exiftool.clone();
        tokio::task::spawn_blocking(move || {
            let response = exiftool.execute_raw(&args)?;
            parse_write_response(response, path)
        })
        .await
        .map_err(|e| Error::process(format!("异步写入任务执行失败: {}", e)))?
    }
}

/// 解析写入响应为 WriteResult
///
/// 从 ExifTool 的响应中解析写入结果。
/// 此函数被同步 `execute` 和异步 `async_execute` 共用。
fn parse_write_response(response: crate::process::Response, path: PathBuf) -> Result<WriteResult> {
    if response.is_error() {
        return Err(Error::process(
            response
                .error_message()
                .unwrap_or_else(|| "Unknown write error".to_string()),
        ));
    }

    Ok(WriteResult {
        path,
        lines: response.lines().to_vec(),
    })
}

/// 写入操作结果
#[derive(Debug, Clone)]
pub struct WriteResult {
    /// 被修改的文件路径
    pub path: PathBuf,

    /// ExifTool 输出信息
    pub lines: Vec<String>,
}

impl WriteResult {
    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.lines.iter().any(|line| {
            line.contains("image files updated") || line.contains("image files unchanged")
        })
    }

    /// 获取修改的文件数量
    pub fn updated_count(&self) -> Option<u32> {
        for line in &self.lines {
            if let Some(pos) = line.find("image files updated") {
                let num_str: String = line[..pos].chars().filter(|c| c.is_ascii_digit()).collect();
                return num_str.parse().ok();
            }
        }
        None
    }

    /// 获取创建的备份文件路径
    pub fn backup_path(&self) -> Option<PathBuf> {
        let backup = self.path.with_extension(format!(
            "{}_original",
            self.path.extension()?.to_string_lossy()
        ));
        if backup.exists() { Some(backup) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn test_write_builder_args() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let args = exiftool
            .write("photo.jpg")
            .write_mode(WriteMode::Create)
            .password("p")
            .separator(",")
            .api_option("QuickTimeUTC", Some("1"))
            .user_param("k", Some("v"))
            .condition("$FileType eq 'JPEG'")
            .delete("Comment")
            .tag("Artist", "Alice")
            .build_args();

        assert!(args.windows(2).any(|w| w == ["-wm", "c"]));
        assert!(args.windows(2).any(|w| w == ["-password", "p"]));
        assert!(args.windows(2).any(|w| w == ["-sep", ","]));
        assert!(args.windows(2).any(|w| w == ["-api", "QuickTimeUTC=1"]));
        assert!(args.windows(2).any(|w| w == ["-userParam", "k=v"]));
        assert!(args.windows(2).any(|w| w == ["-if", "$FileType eq 'JPEG'"]));
        assert!(args.iter().any(|a| a == "-Comment="));
        assert!(args.iter().any(|a| a == "-Artist=Alice"));
    }

    #[test]
    fn test_write_result_parsing() {
        let result = WriteResult {
            path: PathBuf::from("test.jpg"),
            lines: vec!["    1 image files updated".to_string()],
        };

        assert!(result.is_success());
        assert_eq!(result.updated_count(), Some(1));
    }

    /// 测试标签写入运算符变体：追加、移除、前置
    #[test]
    fn test_tag_write_operators() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        // 测试追加运算符 +=
        let args = exiftool
            .write("photo.jpg")
            .tag_append("Keywords", "landscape")
            .build_args();
        assert!(
            args.iter().any(|a| a == "-Keywords+=landscape"),
            "参数列表应包含 \"-Keywords+=landscape\"，实际: {:?}",
            args
        );

        // 测试移除运算符 -=
        let args = exiftool
            .write("photo.jpg")
            .tag_remove("Keywords", "old")
            .build_args();
        assert!(
            args.iter().any(|a| a == "-Keywords-=old"),
            "参数列表应包含 \"-Keywords-=old\"，实际: {:?}",
            args
        );

        // 测试前置运算符 ^=
        let args = exiftool
            .write("photo.jpg")
            .tag_prepend("Keywords", "important")
            .build_args();
        assert!(
            args.iter().any(|a| a == "-Keywords^=important"),
            "参数列表应包含 \"-Keywords^=important\"，实际: {:?}",
            args
        );
    }

    /// 测试从文件读取标签值写入
    #[test]
    fn test_tag_from_file() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        // 测试从文件读取 <=
        let args = exiftool
            .write("photo.jpg")
            .tag_from_file("ThumbnailImage", "thumb.jpg")
            .build_args();
        assert!(
            args.iter().any(|a| a == "-ThumbnailImage<=thumb.jpg"),
            "参数列表应包含 \"-ThumbnailImage<=thumb.jpg\"，实际: {:?}",
            args
        );

        // 测试追加从文件读取 +<=
        let args = exiftool
            .write("photo.jpg")
            .tag_append_from_file("Comment", "comment.txt")
            .build_args();
        assert!(
            args.iter().any(|a| a == "-Comment+<=comment.txt"),
            "参数列表应包含 \"-Comment+<=comment.txt\"，实际: {:?}",
            args
        );
    }

    /// 测试 global_time_shift 方法：验证 -globalTimeShift 参数构建正确
    #[test]
    fn test_global_time_shift() {
        let exiftool = match crate::ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
        };

        let args = exiftool
            .write("photo.jpg")
            .global_time_shift("+02:00")
            .build_args();

        assert!(
            args.windows(2).any(|w| w == ["-globalTimeShift", "+02:00"]),
            "参数列表应包含 [\"-globalTimeShift\", \"+02:00\"]，实际: {:?}",
            args
        );
    }
}
