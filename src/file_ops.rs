//! 文件操作模块
//!
//! 支持基于元数据的文件重命名、图像提取等操作

use crate::ExifTool;
use crate::error::Result;
use crate::types::TagId;
use std::path::{Path, PathBuf};

/// 文件重命名模式
#[derive(Debug, Clone)]
pub enum RenamePattern {
    /// 基于日期时间
    DateTime { format: String },
    /// 基于特定标签
    Tag { tag: TagId, suffix: Option<String> },
    /// 自定义格式字符串
    Custom(String),
}

impl RenamePattern {
    /// 创建日期时间模式
    pub fn datetime(format: impl Into<String>) -> Self {
        Self::DateTime {
            format: format.into(),
        }
    }

    /// 创建标签模式
    pub fn tag(tag: TagId) -> Self {
        Self::Tag { tag, suffix: None }
    }

    /// 创建带后缀的标签模式
    pub fn tag_with_suffix(tag: TagId, suffix: impl Into<String>) -> Self {
        Self::Tag {
            tag,
            suffix: Some(suffix.into()),
        }
    }

    /// 创建自定义模式
    pub fn custom(format: impl Into<String>) -> Self {
        Self::Custom(format.into())
    }

    /// 转换为 ExifTool 文件名格式
    fn to_exiftool_format(&self) -> String {
        match self {
            Self::DateTime { format } => {
                format!("%{{DateTimeOriginal,{}}}", format)
            }
            Self::Tag { tag, suffix } => {
                if let Some(suf) = suffix {
                    format!("%{{{}}}{}", tag.name(), suf)
                } else {
                    format!("%{{{}}}", tag.name())
                }
            }
            Self::Custom(format) => format.clone(),
        }
    }
}

/// 文件组织选项
#[derive(Debug, Clone)]
pub struct OrganizeOptions {
    /// 目标目录
    pub target_dir: PathBuf,
    /// 子目录模式
    pub subdir_pattern: Option<RenamePattern>,
    /// 文件名模式
    pub filename_pattern: RenamePattern,
    /// 文件扩展名
    pub extension: Option<String>,
}

impl OrganizeOptions {
    /// 创建新的组织选项
    pub fn new<P: AsRef<Path>>(target_dir: P) -> Self {
        Self {
            target_dir: target_dir.as_ref().to_path_buf(),
            subdir_pattern: None,
            filename_pattern: RenamePattern::datetime("%Y%m%d_%H%M%S"),
            extension: None,
        }
    }

    /// 设置子目录模式
    pub fn subdir(mut self, pattern: RenamePattern) -> Self {
        self.subdir_pattern = Some(pattern);
        self
    }

    /// 设置文件名模式
    pub fn filename(mut self, pattern: RenamePattern) -> Self {
        self.filename_pattern = pattern;
        self
    }

    /// 设置文件扩展名
    pub fn extension(mut self, ext: impl Into<String>) -> Self {
        self.extension = Some(ext.into());
        self
    }
}

/// 文件操作 trait
pub trait FileOperations {
    /// 重命名单个文件
    fn rename_file<P: AsRef<Path>>(&self, path: P, pattern: &RenamePattern) -> Result<PathBuf>;

    /// 批量重命名文件
    fn rename_files<P: AsRef<Path>>(
        &self,
        paths: &[P],
        pattern: &RenamePattern,
    ) -> Result<Vec<PathBuf>>;

    /// 按日期组织文件到目录
    fn organize_by_date<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        path: P,
        target_dir: Q,
        date_format: &str,
    ) -> Result<PathBuf>;

    /// 根据元数据组织文件
    fn organize<P: AsRef<Path>>(&self, path: P, options: &OrganizeOptions) -> Result<PathBuf>;

    /// 生成元数据备份文件 (.mie 格式)
    fn create_metadata_backup<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        backup_path: Q,
    ) -> Result<()>;

    /// 从备份恢复元数据
    fn restore_from_backup<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        backup: P,
        target: Q,
    ) -> Result<()>;
}

impl FileOperations for ExifTool {
    fn rename_file<P: AsRef<Path>>(&self, path: P, pattern: &RenamePattern) -> Result<PathBuf> {
        let format = pattern.to_exiftool_format();

        self.write(path.as_ref())
            .arg(format!("-filename={}", format))
            .execute()?;

        // 构建新路径（简化版，实际需要查询 ExifTool 输出）
        let parent = path.as_ref().parent().unwrap_or(Path::new("."));
        let new_name = format!(
            "renamed_{}",
            path.as_ref()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );

        Ok(parent.join(new_name))
    }

    fn rename_files<P: AsRef<Path>>(
        &self,
        paths: &[P],
        pattern: &RenamePattern,
    ) -> Result<Vec<PathBuf>> {
        let mut results = Vec::with_capacity(paths.len());

        for path in paths {
            let new_path = self.rename_file(path, pattern)?;
            results.push(new_path);
        }

        Ok(results)
    }

    fn organize_by_date<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        path: P,
        target_dir: Q,
        date_format: &str,
    ) -> Result<PathBuf> {
        let options = OrganizeOptions::new(target_dir).subdir(RenamePattern::datetime(date_format));

        self.organize(path, &options)
    }

    fn organize<P: AsRef<Path>>(&self, path: P, options: &OrganizeOptions) -> Result<PathBuf> {
        let mut args = Vec::new();

        // 目录选项
        args.push("-d".to_string());
        args.push(options.target_dir.to_string_lossy().to_string());

        // 子目录模式
        if let Some(ref subdir) = options.subdir_pattern {
            args.push(format!(
                "-filename={}/{}",
                subdir.to_exiftool_format(),
                options.filename_pattern.to_exiftool_format()
            ));
        } else {
            args.push(format!(
                "-filename={}",
                options.filename_pattern.to_exiftool_format()
            ));
        }

        // 文件扩展名
        if let Some(ref ext) = options.extension {
            args.push(format!("-ext {}", ext));
        }

        args.push(path.as_ref().to_string_lossy().to_string());

        self.execute_raw(&args)?;

        // 返回目标路径（简化版）
        Ok(options.target_dir.clone())
    }

    fn create_metadata_backup<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        backup_path: Q,
    ) -> Result<()> {
        self.write(source)
            .arg(format!("-o {}", backup_path.as_ref().display()))
            .arg("-tagsFromFile @")
            .arg("-all:all")
            .execute()?;

        Ok(())
    }

    fn restore_from_backup<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        backup: P,
        target: Q,
    ) -> Result<()> {
        self.write(target)
            .copy_from(backup)
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_pattern() {
        let pattern = RenamePattern::datetime("%Y-%m-%d");
        assert_eq!(pattern.to_exiftool_format(), "%{DateTimeOriginal,%Y-%m-%d}");

        let pattern = RenamePattern::tag(TagId::MAKE);
        assert_eq!(pattern.to_exiftool_format(), "%{Make}");

        let pattern = RenamePattern::tag_with_suffix(TagId::MODEL, "_photo");
        assert_eq!(pattern.to_exiftool_format(), "%{Model}_photo");
    }

    #[test]
    fn test_organize_options() {
        let opts = OrganizeOptions::new("/output")
            .subdir(RenamePattern::datetime("%Y/%m"))
            .filename(RenamePattern::datetime("%d_%H%M%S"))
            .extension("jpg");

        assert_eq!(opts.target_dir, PathBuf::from("/output"));
        assert!(opts.subdir_pattern.is_some());
        assert!(opts.extension.is_some());
    }
}
