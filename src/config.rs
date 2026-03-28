//! 配置和校验模块
//!
//! 支持配置文件加载、文件比较、十六进制转储、详细输出

use crate::ExifTool;
use crate::error::Result;
use crate::types::TagId;
use std::path::Path;

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
///
/// 对应 ExifTool 的 `-htmlDump[OFFSET]` 选项。
/// ExifTool 原生仅支持起始偏移量，不支持长度限制或每行字节数控制。
#[derive(Debug, Clone, Default)]
pub struct HexDumpOptions {
    /// 起始偏移量（对应 `-htmlDumpOFFSET`）
    pub start_offset: Option<usize>,
}

impl HexDumpOptions {
    /// 创建新的选项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置起始偏移
    pub fn start(mut self, offset: usize) -> Self {
        self.start_offset = Some(offset);
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
        let opts = HexDumpOptions::new().start(100);

        assert_eq!(opts.start_offset, Some(100));
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

    #[test]
    fn test_diff_two_different_files() {
        // 检查 ExifTool 是否可用，不可用则跳过
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时出现意外错误: {:?}", e),
        };

        // 创建两个内容相同的临时 JPEG 文件
        let tmp_dir = tempfile::tempdir().expect("创建临时目录失败");
        let file_a = tmp_dir.path().join("diff_a.jpg");
        let file_b = tmp_dir.path().join("diff_b.jpg");
        std::fs::write(&file_a, TINY_JPEG).expect("写入文件 A 失败");
        std::fs::write(&file_b, TINY_JPEG).expect("写入文件 B 失败");

        // 分别写入不同的元数据，使两个文件产生差异
        et.write(&file_a)
            .tag("Artist", "Alice")
            .tag("Copyright", "2026 Alice")
            .overwrite_original(true)
            .execute()
            .expect("写入文件 A 的元数据失败");

        et.write(&file_b)
            .tag("Artist", "Bob")
            .tag("Copyright", "2026 Bob")
            .overwrite_original(true)
            .execute()
            .expect("写入文件 B 的元数据失败");

        // 比较两个文件
        let diff = et.diff(&file_a, &file_b).expect("执行 diff 操作失败");

        // 验证两个文件不相同
        assert!(
            !diff.is_identical,
            "写入不同元数据后的两个文件的 diff 结果应为不相同"
        );
        // 验证差异列表中包含有差异的标签
        // Artist 和 Copyright 至少会产生差异
        assert!(
            !diff.different.is_empty(),
            "diff 结果的 different 列表不应为空，应至少包含 Artist/Copyright 的差异"
        );
    }
}
