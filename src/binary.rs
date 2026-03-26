//! 二进制数据处理模块

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
        let args = vec![
            "-b".to_string(),
            format!("-{}", tag.tag_name()),
            path.as_ref().to_string_lossy().to_string(),
        ];

        let response = self.execute_raw(&args)?;

        // 二进制数据在响应中直接返回
        let data = response.text().trim().as_bytes().to_vec();

        // 如果响应看起来是 Base64 编码的，尝试解码
        if let Ok(decoded) = base64_decode(response.text().trim()) {
            Ok(decoded)
        } else {
            Ok(data)
        }
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

/// Base64 解码（简单实现）
fn base64_decode(input: &str) -> Result<Vec<u8>> {
    use base64::{Engine, engine::general_purpose::STANDARD};

    STANDARD
        .decode(input)
        .map_err(|e| Error::parse(format!("Base64 decode error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tag() {
        assert_eq!(BinaryTag::Thumbnail.tag_name(), "ThumbnailImage");
        assert_eq!(BinaryTag::Preview.tag_name(), "PreviewImage");

        let tag: BinaryTag = "ThumbnailImage".into();
        assert_eq!(tag, BinaryTag::Thumbnail);
    }

    #[test]
    fn test_base64_decode() {
        // 测试简单 Base64 解码
        let encoded = "SGVsbG8gV29ybGQh";
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, b"Hello World!");
    }
}
