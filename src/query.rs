//! 查询构建器 - Builder 模式实现

use std::path::{Path, PathBuf};

use crate::ExifTool;
use crate::error::{Error, Result};
use crate::process::Response;
use crate::types::{Metadata, TagId};

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

    /// 添加原始参数（高级用法）
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
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
