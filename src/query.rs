//! 查询构建器 - Builder 模式实现

use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::process::Response;
use crate::types::{Metadata, TagId};
use crate::ExifTool;

/// 查询构建器
pub struct QueryBuilder<'et> {
    exiftool: &'et ExifTool,
    path: PathBuf,
    args: Vec<String>,
    include_unknown: bool,
    include_duplicates: bool,
    raw_values: bool,
    group_by_category: bool,
    specific_tags: Vec<String>,
    excluded_tags: Vec<String>,
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
            specific_tags: Vec::new(),
            excluded_tags: Vec::new(),
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
        self.args.push(format!("-charset {}", charset.into()));
        self
    }

    /// 使用特定语言
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.args.push(format!("-lang {}", lang.into()));
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
        self.args.push(format!("-c {}", format.into()));
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
        self.args.push(format!("-d {}", format.into()));
        self
    }

    /// 添加原始参数（高级用法）
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
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
        self.args.push(format!("-p {}", format.into()));
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
        self.args.push(format!("-sep {}", sep.into()));
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
        let arg = match value {
            Some(v) => format!("-api {}={}", opt.into(), v.into()),
            None => format!("-api {}", opt.into()),
        };
        self.args.push(arg);
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
        let arg = match value {
            Some(v) => format!("-userParam {}={}", param.into(), v.into()),
            None => format!("-userParam {}", param.into()),
        };
        self.args.push(arg);
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
        self.args.push(format!("-password {}", passwd.into()));
        self
    }

    /// 执行查询
    pub fn execute(self) -> Result<Metadata> {
        let args = self.build_args();

        // 发送命令并获取响应
        let response = self.exiftool.execute_raw(&args)?;

        // 解析响应
        self.parse_response(response)
    }

    /// 执行查询并返回 JSON
    pub fn execute_json(self) -> Result<serde_json::Value> {
        let args = self.build_args();
        let response = self.exiftool.execute_raw(&args)?;
        response.json()
    }

    /// 执行查询并反序列化为自定义类型
    pub fn execute_as<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let args = self.build_args();
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
        let args = self.build_args();
        let response = self.exiftool.execute_raw(&args)?;
        Ok(response.text().trim().to_string())
    }

    /// 构建参数列表
    fn build_args(&self) -> Vec<String> {
        let mut args = vec!["-json".to_string()];

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

        // 添加自定义参数
        args.extend(self.args.clone());

        // 特定标签
        for tag in &self.specific_tags {
            args.push(format!("-{}", tag));
        }

        // 排除标签
        for tag in &self.excluded_tags {
            args.push(format!("-{}=", tag));
        }

        // 文件路径
        args.push(self.path.to_string_lossy().to_string());

        args
    }

    /// 解析响应
    fn parse_response(&self, response: Response) -> Result<Metadata> {
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
        self.args.push(format!("-c {}", format.into()));
        self
    }

    /// 设置日期/时间格式
    pub fn date_format(mut self, format: impl Into<String>) -> Self {
        self.args.push(format!("-d {}", format.into()));
        self
    }

    /// 设置密码
    pub fn password(mut self, passwd: impl Into<String>) -> Self {
        self.args.push(format!("-password {}", passwd.into()));
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
        let arg = match value {
            Some(v) => format!("-api {}={}", opt.into(), v.into()),
            None => format!("-api {}", opt.into()),
        };
        self.args.push(arg);
        self
    }

    /// 设置用户参数
    pub fn user_param(
        mut self,
        param: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        let arg = match value {
            Some(v) => format!("-userParam {}={}", param.into(), v.into()),
            None => format!("-userParam {}", param.into()),
        };
        self.args.push(arg);
        self
    }

    /// 设置自定义打印格式
    pub fn print_format(mut self, format: impl Into<String>) -> Self {
        self.args.push(format!("-p {}", format.into()));
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
        self.args.push(format!("-sep {}", sep.into()));
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
    #[test]
    fn test_query_builder_args() {
        // 这个测试需要 ExifTool 实例，所以只做简单的构建测试
        // 实际测试需要在集成测试中进行
    }
}
