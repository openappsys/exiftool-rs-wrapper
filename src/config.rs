//! 配置和校验模块
//!
//! 支持配置文件加载、自定义标签定义、校验和计算

use crate::ExifTool;
use crate::error::Result;
use crate::types::TagId;
use std::path::{Path, PathBuf};

/// 校验和算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChecksumAlgorithm {
    /// MD5
    MD5,
    /// SHA1
    SHA1,
    /// SHA256
    SHA256,
    /// SHA512
    SHA512,
}

impl ChecksumAlgorithm {
    /// 获取算法名称
    pub fn name(&self) -> &'static str {
        match self {
            Self::MD5 => "MD5",
            Self::SHA1 => "SHA1",
            Self::SHA256 => "SHA256",
            Self::SHA512 => "SHA512",
        }
    }

    /// 获取 ExifTool 参数
    pub fn arg(&self) -> String {
        format!("-{}", self.name())
    }
}

/// 校验和结果
#[derive(Debug, Clone)]
pub struct ChecksumResult {
    /// 文件路径
    pub path: PathBuf,
    /// 校验和值
    pub checksum: String,
    /// 算法
    pub algorithm: ChecksumAlgorithm,
}

/// 文件比较结果
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// 是否相同
    pub is_identical: bool,
    /// 仅在源文件中存在的标签
    pub source_only: Vec<String>,
    /// 仅在目标文件中存在的标签
    pub target_only: Vec<String>,
    /// 值不同的标签
    pub different: Vec<(String, String, String)>, // (tag, source_value, target_value)
}

impl Default for DiffResult {
    fn default() -> Self {
        Self {
            is_identical: true,
            source_only: Vec::new(),
            target_only: Vec::new(),
            different: Vec::new(),
        }
    }
}

impl DiffResult {
    /// 创建新的比较结果
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加仅在源文件存在的标签
    pub fn add_source_only(&mut self, tag: impl Into<String>) {
        self.is_identical = false;
        self.source_only.push(tag.into());
    }

    /// 添加仅在目标文件存在的标签
    pub fn add_target_only(&mut self, tag: impl Into<String>) {
        self.is_identical = false;
        self.target_only.push(tag.into());
    }

    /// 添加不同的标签
    pub fn add_different(
        &mut self,
        tag: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
    ) {
        self.is_identical = false;
        self.different
            .push((tag.into(), source.into(), target.into()));
    }
}

/// 配置操作 trait
pub trait ConfigOperations {
    /// 加载配置文件（`-config`）
    fn with_config<P: AsRef<Path>>(&self, config_path: P) -> ExifTool;

    /// 计算校验和
    fn calculate_checksum<P: AsRef<Path>>(
        &self,
        path: P,
        algorithm: ChecksumAlgorithm,
    ) -> Result<ChecksumResult>;

    /// 计算多个校验和
    fn calculate_checksums<P: AsRef<Path>>(
        &self,
        path: P,
        algorithms: &[ChecksumAlgorithm],
    ) -> Result<Vec<ChecksumResult>>;

    /// 验证文件完整性
    fn verify_checksum<P: AsRef<Path>>(
        &self,
        path: P,
        expected: &str,
        algorithm: ChecksumAlgorithm,
    ) -> Result<bool>;

    /// 比较两个文件的元数据
    fn diff<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, target: Q) -> Result<DiffResult>;

    /// 比较两个文件的元数据（仅特定标签）
    fn diff_tags<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
        tags: &[TagId],
    ) -> Result<DiffResult>;
}

impl ConfigOperations for ExifTool {
    fn with_config<P: AsRef<Path>>(&self, config_path: P) -> ExifTool {
        ExifTool::with_config(self, config_path)
    }

    fn calculate_checksum<P: AsRef<Path>>(
        &self,
        path: P,
        algorithm: ChecksumAlgorithm,
    ) -> Result<ChecksumResult> {
        let args = vec![
            algorithm.arg(),
            "-b".to_string(),
            path.as_ref().to_string_lossy().to_string(),
        ];

        let response = self.execute_raw(&args)?;
        let checksum = response.text().trim().to_string();

        Ok(ChecksumResult {
            path: path.as_ref().to_path_buf(),
            checksum,
            algorithm,
        })
    }

    fn calculate_checksums<P: AsRef<Path>>(
        &self,
        path: P,
        algorithms: &[ChecksumAlgorithm],
    ) -> Result<Vec<ChecksumResult>> {
        let mut results = Vec::with_capacity(algorithms.len());

        for algo in algorithms {
            results.push(self.calculate_checksum(path.as_ref(), *algo)?);
        }

        Ok(results)
    }

    fn verify_checksum<P: AsRef<Path>>(
        &self,
        path: P,
        expected: &str,
        algorithm: ChecksumAlgorithm,
    ) -> Result<bool> {
        let result = self.calculate_checksum(path, algorithm)?;
        Ok(result.checksum.eq_ignore_ascii_case(expected))
    }

    fn diff<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, target: Q) -> Result<DiffResult> {
        let source_meta = self.query(&source).execute()?;
        let target_meta = self.query(&target).execute()?;

        compare_metadata(&source_meta, &target_meta)
    }

    fn diff_tags<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        target: Q,
        tags: &[TagId],
    ) -> Result<DiffResult> {
        let mut source_query = self.query(&source);
        let mut target_query = self.query(&target);

        for tag in tags {
            source_query = source_query.tag_id(*tag);
            target_query = target_query.tag_id(*tag);
        }

        let source_meta = source_query.execute()?;
        let target_meta = target_query.execute()?;

        compare_metadata(&source_meta, &target_meta)
    }
}

/// 比较两个元数据结构
fn compare_metadata(
    source: &crate::types::Metadata,
    target: &crate::types::Metadata,
) -> Result<DiffResult> {
    let mut result = DiffResult::new();

    // 收集所有标签
    let mut all_tags: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (tag, _) in source.iter() {
        all_tags.insert(tag.clone());
    }
    for (tag, _) in target.iter() {
        all_tags.insert(tag.clone());
    }

    // 比较每个标签
    for tag in all_tags {
        match (source.get(&tag), target.get(&tag)) {
            (Some(s), Some(t)) => {
                if s != t {
                    result.add_different(&tag, s.to_string_lossy(), t.to_string_lossy());
                }
            }
            (Some(_), None) => result.add_source_only(&tag),
            (None, Some(_)) => result.add_target_only(&tag),
            (None, None) => {} // 不可能发生
        }
    }

    Ok(result)
}

/// 十六进制转储选项
#[derive(Debug, Clone, Default)]
pub struct HexDumpOptions {
    /// 起始偏移
    pub start_offset: Option<usize>,
    /// 长度限制
    pub length: Option<usize>,
    /// 每行字节数
    pub bytes_per_line: usize,
}

impl HexDumpOptions {
    /// 创建新的选项
    pub fn new() -> Self {
        Self {
            start_offset: None,
            length: None,
            bytes_per_line: 16,
        }
    }

    /// 设置起始偏移
    pub fn start(mut self, offset: usize) -> Self {
        self.start_offset = Some(offset);
        self
    }

    /// 设置长度
    pub fn length(mut self, len: usize) -> Self {
        self.length = Some(len);
        self
    }

    /// 设置每行字节数
    pub fn bytes_per_line(mut self, n: usize) -> Self {
        self.bytes_per_line = n;
        self
    }
}

/// 十六进制转储 trait
pub trait HexDumpOperations {
    /// 获取文件的十六进制转储
    fn hex_dump<P: AsRef<Path>>(&self, path: P, options: &HexDumpOptions) -> Result<String>;

    /// 获取特定标签的十六进制值
    fn hex_dump_tag<P: AsRef<Path>>(&self, path: P, tag: TagId) -> Result<String>;
}

impl HexDumpOperations for ExifTool {
    fn hex_dump<P: AsRef<Path>>(&self, path: P, options: &HexDumpOptions) -> Result<String> {
        let mut args = Vec::new();

        if let Some(offset) = options.start_offset {
            args.push(format!("-htmlDump{}", offset));
        } else {
            args.push("-htmlDump".to_string());
        }

        if options.length.is_some() {
            return Err(crate::error::Error::invalid_arg(
                "hex_dump 的 length 选项当前不受 ExifTool 原生命令直接支持",
            ));
        }

        args.push(path.as_ref().to_string_lossy().to_string());

        let response = self.execute_raw(&args)?;
        Ok(response.text())
    }

    fn hex_dump_tag<P: AsRef<Path>>(&self, path: P, tag: TagId) -> Result<String> {
        let args = vec![
            "-H".to_string(),
            format!("-{}", tag.name()),
            path.as_ref().to_string_lossy().to_string(),
        ];

        let response = self.execute_raw(&args)?;
        Ok(response.text())
    }
}

/// 详细输出选项
#[derive(Debug, Clone)]
pub struct VerboseOptions {
    /// 详细级别 (0-5)
    pub level: u8,
    /// HTML 格式输出
    pub html_format: bool,
}

impl VerboseOptions {
    /// 创建新的详细输出选项
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(5),
            html_format: false,
        }
    }

    /// 使用 HTML 格式
    pub fn html(mut self) -> Self {
        self.html_format = true;
        self
    }

    /// 获取 ExifTool 参数
    pub fn args(&self) -> Vec<String> {
        let mut args = vec![format!("-v{}", self.level)];

        if self.html_format {
            args.push("-htmlDump".to_string());
        }

        args
    }
}

/// 详细输出 trait
pub trait VerboseOperations {
    /// 获取详细输出
    fn verbose_dump<P: AsRef<Path>>(&self, path: P, options: &VerboseOptions) -> Result<String>;
}

impl VerboseOperations for ExifTool {
    fn verbose_dump<P: AsRef<Path>>(&self, path: P, options: &VerboseOptions) -> Result<String> {
        let mut args = options.args();
        args.push(path.as_ref().to_string_lossy().to_string());

        let response = self.execute_raw(&args)?;
        Ok(response.text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_algorithm() {
        assert_eq!(ChecksumAlgorithm::MD5.name(), "MD5");
        assert_eq!(ChecksumAlgorithm::SHA256.arg(), "-SHA256");
    }

    #[test]
    fn test_diff_result() {
        let mut diff = DiffResult::new();
        assert!(diff.is_identical);

        diff.add_source_only("Make");
        assert!(!diff.is_identical);
        assert_eq!(diff.source_only.len(), 1);

        diff.add_different("Model", "Canon", "Nikon");
        assert_eq!(diff.different.len(), 1);
    }

    #[test]
    fn test_hex_dump_options() {
        let opts = HexDumpOptions::new()
            .start(100)
            .length(256)
            .bytes_per_line(32);

        assert_eq!(opts.start_offset, Some(100));
        assert_eq!(opts.length, Some(256));
        assert_eq!(opts.bytes_per_line, 32);
    }

    #[test]
    fn test_verbose_options() {
        let opts = VerboseOptions::new(3);
        let args = opts.args();
        assert!(args.contains(&"-v3".to_string()));

        let opts = VerboseOptions::new(2).html();
        let args = opts.args();
        assert!(args.contains(&"-v2".to_string()));
        assert!(args.contains(&"-htmlDump".to_string()));
    }
}
