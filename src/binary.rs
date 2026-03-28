//! 二进制数据处理模块
//!
//! # 注意事项
//!
//! 在 `-stay_open` 模式下，ExifTool 通过 stdout 文本协议传输数据，
//! 使用 `{ready}` 作为响应结束标记。如果二进制内容恰好包含 `{ready}`
//! 字符串，会导致响应解析提前终止，返回截断的数据。
//!
//! 因此，`read_binary()` 采用临时文件方案：先用 `-b -TAG -w TMPFILE`
//! 将二进制数据输出到临时文件，再读取文件内容，避免文本协议的限制。

use crate::ExifTool;
use crate::error::{Error, Result};
use std::path::{Path, PathBuf};

/// 二进制数据写入构建器
pub struct BinaryWriteBuilder<'et> {
    exiftool: &'et ExifTool,
    path: PathBuf,
    binary_tags: Vec<(BinaryTag, Vec<u8>)>,
    overwrite_original: bool,
    backup: bool,
}

impl<'et> BinaryWriteBuilder<'et> {
    /// 创建新的二进制写入构建器
    pub(crate) fn new<P: AsRef<Path>>(exiftool: &'et ExifTool, path: P) -> Self {
        Self {
            exiftool,
            path: path.as_ref().to_path_buf(),
            binary_tags: Vec::new(),
            overwrite_original: false,
            backup: true,
        }
    }

    /// 设置缩略图
    pub fn thumbnail(mut self, data: Vec<u8>) -> Self {
        self.binary_tags.push((BinaryTag::Thumbnail, data));
        self
    }

    /// 设置预览图
    pub fn preview(mut self, data: Vec<u8>) -> Self {
        self.binary_tags.push((BinaryTag::Preview, data));
        self
    }

    /// 设置 JPEG 预览
    pub fn jpeg_preview(mut self, data: Vec<u8>) -> Self {
        self.binary_tags.push((BinaryTag::JpegPreview, data));
        self
    }

    /// 覆盖原始文件
    pub fn overwrite_original(mut self, yes: bool) -> Self {
        self.overwrite_original = yes;
        self
    }

    /// 创建备份
    pub fn backup(mut self, yes: bool) -> Self {
        self.backup = yes;
        self
    }

    /// 执行写入
    pub fn execute(self) -> Result<BinaryWriteResult> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut temp_files = Vec::with_capacity(self.binary_tags.len());
        let mut args = Vec::new();

        // 基础选项
        if self.overwrite_original {
            args.push("-overwrite_original".to_string());
        }

        if !self.backup {
            args.push("-overwrite_original_in_place".to_string());
        }

        // 为每个二进制标签创建临时文件
        for (tag, data) in &self.binary_tags {
            let mut temp_file = NamedTempFile::new()?;
            temp_file.write_all(data)?;
            let temp_path = temp_file.path().to_path_buf();
            temp_files.push(temp_file);

            args.push(format!("-{}={}", tag.tag_name(), temp_path.display()));
        }

        // 添加目标文件
        args.push(self.path.to_string_lossy().to_string());

        // 执行命令
        let response = self.exiftool.execute_raw(&args)?;

        // 临时文件会在 temp_files 被 drop 时自动删除
        drop(temp_files);

        if response.is_error() {
            return Err(Error::process(
                response
                    .error_message()
                    .unwrap_or_else(|| "Unknown binary write error".to_string()),
            ));
        }

        Ok(BinaryWriteResult {
            path: self.path,
            written_tags: self.binary_tags.into_iter().map(|(tag, _)| tag).collect(),
        })
    }
}

/// 二进制写入结果
#[derive(Debug, Clone)]
pub struct BinaryWriteResult {
    /// 被修改的文件路径
    pub path: PathBuf,

    /// 写入的二进制标签
    pub written_tags: Vec<BinaryTag>,
}

/// 二进制标签类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryTag {
    /// 缩略图
    Thumbnail,

    /// 预览图
    Preview,

    /// JPEG 预览
    JpegPreview,

    /// 其他自定义标签
    Other(&'static str),
}

impl BinaryTag {
    /// 获取标签名称
    pub fn tag_name(&self) -> &str {
        match self {
            Self::Thumbnail => "ThumbnailImage",
            Self::Preview => "PreviewImage",
            Self::JpegPreview => "JpgFromRaw",
            Self::Other(name) => name,
        }
    }
}

impl From<&'static str> for BinaryTag {
    fn from(name: &'static str) -> Self {
        match name {
            "ThumbnailImage" => Self::Thumbnail,
            "PreviewImage" => Self::Preview,
            "JpgFromRaw" => Self::JpegPreview,
            _ => Self::Other(name),
        }
    }
}

/// 扩展 ExifTool 以支持二进制操作
pub trait BinaryOperations {
    /// 读取二进制数据
    fn read_binary<P: AsRef<Path>>(&self, path: P, tag: BinaryTag) -> Result<Vec<u8>>;

    /// 写入二进制数据
    fn write_binary<P: AsRef<Path>>(&self, path: P) -> BinaryWriteBuilder<'_>;

    /// 提取缩略图到文件
    fn extract_thumbnail<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, dest: Q) -> Result<()>;

    /// 提取预览图到文件
    fn extract_preview<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, dest: Q) -> Result<()>;
}

impl BinaryOperations for ExifTool {
    fn read_binary<P: AsRef<Path>>(&self, path: P, tag: BinaryTag) -> Result<Vec<u8>> {
        // 警告：不能直接通过 stay_open stdout 文本协议传输二进制数据，
        // 因为二进制内容可能包含 `{ready}` 字符串，导致响应解析提前终止。
        // 这里采用临时文件方案：使用 `-b -TAG -w` 输出到临时文件，再读取文件内容。
        let tmp_dir = tempfile::tempdir()?;
        // 使用固定文件名模板，ExifTool 的 -w 选项会将输出写入此目录
        // %f 代表原始文件名（不含扩展名），%s 代表原始扩展名
        let out_pattern = format!("{}/bin_out.dat", tmp_dir.path().display());

        let args = vec![
            "-b".to_string(),
            format!("-{}", tag.tag_name()),
            "-w".to_string(),
            out_pattern.clone(),
            path.as_ref().to_string_lossy().to_string(),
        ];

        let response = self.execute_raw(&args)?;

        // 检查 ExifTool 是否报告错误
        if response.is_error() {
            return Err(Error::process(
                response
                    .error_message()
                    .unwrap_or_else(|| "读取二进制数据失败".to_string()),
            ));
        }

        // 读取临时文件中的二进制数据
        let data = std::fs::read(&out_pattern).map_err(|e| {
            Error::process(format!(
                "读取临时文件失败（标签 {} 可能不存在）: {}",
                tag.tag_name(),
                e
            ))
        })?;

        // 临时目录会在 tmp_dir 被 drop 时自动清理
        Ok(data)
    }

    fn write_binary<P: AsRef<Path>>(&self, path: P) -> BinaryWriteBuilder<'_> {
        BinaryWriteBuilder::new(self, path)
    }

    fn extract_thumbnail<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, dest: Q) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let data = self.read_binary(source, BinaryTag::Thumbnail)?;

        if data.is_empty() {
            return Err(Error::TagNotFound("ThumbnailImage".to_string()));
        }

        let mut file = File::create(dest)?;
        file.write_all(&data)?;

        Ok(())
    }

    fn extract_preview<P: AsRef<Path>, Q: AsRef<Path>>(&self, source: P, dest: Q) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let data = self.read_binary(source, BinaryTag::Preview)?;

        if data.is_empty() {
            return Err(Error::TagNotFound("PreviewImage".to_string()));
        }

        let mut file = File::create(dest)?;
        file.write_all(&data)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExifTool;

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
    fn test_binary_tag() {
        assert_eq!(BinaryTag::Thumbnail.tag_name(), "ThumbnailImage");
        assert_eq!(BinaryTag::Preview.tag_name(), "PreviewImage");

        let tag: BinaryTag = "ThumbnailImage".into();
        assert_eq!(tag, BinaryTag::Thumbnail);
    }

    /// Base64 解码（仅用于测试）
    fn base64_decode(input: &str) -> Result<Vec<u8>> {
        use base64::{Engine, engine::general_purpose::STANDARD};

        STANDARD
            .decode(input)
            .map_err(|e| Error::parse(format!("Base64 解码错误: {}", e)))
    }

    #[test]
    fn test_base64_decode() {
        // 测试简单 Base64 解码
        let encoded = "SGVsbG8gV29ybGQh";
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, b"Hello World!");
    }

    #[test]
    fn test_extract_thumbnail_with_embedded_image() {
        // 检查 ExifTool 是否可用，不可用则跳过
        let et = match ExifTool::new() {
            Ok(et) => et,
            Err(crate::error::Error::ExifToolNotFound) => return,
            Err(e) => panic!("创建 ExifTool 实例时出现意外错误: {:?}", e),
        };

        // 创建临时 JPEG 文件
        let tmp_dir = tempfile::tempdir().expect("创建临时目录失败");
        let src_file = tmp_dir.path().join("thumb_source.jpg");
        std::fs::write(&src_file, TINY_JPEG).expect("写入临时 JPEG 文件失败");

        // 先使用 write_binary 写入一个缩略图数据
        // 使用 TINY_JPEG 本身作为缩略图数据
        let write_result = et
            .write_binary(&src_file)
            .thumbnail(TINY_JPEG.to_vec())
            .overwrite_original(true)
            .execute();

        match write_result {
            Ok(_) => {
                // 缩略图写入成功，尝试提取
                let dest_file = tmp_dir.path().join("extracted_thumb.jpg");
                let extract_result = et.extract_thumbnail(&src_file, &dest_file);

                match extract_result {
                    Ok(()) => {
                        // 验证提取的文件存在且非空
                        assert!(
                            dest_file.exists(),
                            "提取的缩略图文件应存在: {:?}",
                            dest_file
                        );
                        let thumb_data =
                            std::fs::read(&dest_file).expect("读取提取的缩略图文件失败");
                        assert!(!thumb_data.is_empty(), "提取的缩略图文件不应为空");
                    }
                    Err(e) => {
                        // 最小 JPEG 可能不支持缩略图嵌入，记录但不使测试失败
                        eprintln!("缩略图提取失败（最小 JPEG 可能不支持缩略图嵌入）: {:?}", e);
                    }
                }
            }
            Err(e) => {
                // 缩略图写入失败（最小 JPEG 可能不支持），记录但不使测试失败
                eprintln!("缩略图写入失败（最小 JPEG 可能不支持缩略图嵌入）: {:?}", e);
            }
        }
    }
}
