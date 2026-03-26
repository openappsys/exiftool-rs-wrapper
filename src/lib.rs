//! ExifTool Rust Wrapper
//!
//! 一个用于 ExifTool 的 Rust 封装库，提供类型安全的 API 来读取和写入文件元数据。
//!
//! # 示例
//!
//! ```rust,no_run
//! use exiftool_rs_wrapper::{ExifTool, ExifError};
//!
//! fn main() -> Result<(), ExifError> {
//!     let mut exiftool = ExifTool::new()?;
//!     let metadata = exiftool.read_metadata("photo.jpg")?;
//!     println!("{:?}", metadata);
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};
use thiserror::Error;

/// 错误类型
#[derive(Error, Debug)]
pub enum ExifError {
    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// JSON 解析错误
    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),

    /// ExifTool 执行错误
    #[error("ExifTool 执行失败: {0}")]
    Execution(String),

    /// ExifTool 未找到
    #[error("未找到 ExifTool，请确保已安装并添加到 PATH")]
    NotFound,

    /// 无效的路径
    #[error("无效的路径: {0}")]
    InvalidPath(String),
}

/// ExifTool 元数据结构
pub type Metadata = HashMap<String, serde_json::Value>;

/// ExifTool 包装器
pub struct ExifTool {
    // 可以在这里添加配置选项
}

impl ExifTool {
    /// 创建新的 ExifTool 实例
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// let exiftool = ExifTool::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, ExifError> {
        // 检查 exiftool 是否可用
        match Command::new("exiftool").arg("-ver").output() {
            Ok(output) if output.status.success() => Ok(Self {}),
            Ok(_) => Err(ExifError::Execution("ExifTool 版本检查失败".to_string())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(ExifError::NotFound),
            Err(e) => Err(ExifError::Io(e)),
        }
    }

    /// 读取文件的元数据
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    ///
    /// # 返回值
    ///
    /// 返回包含元数据的 HashMap
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// let mut exiftool = ExifTool::new().unwrap();
    /// let metadata = exiftool.read_metadata("photo.jpg").unwrap();
    /// ```
    pub fn read_metadata<P: AsRef<Path>>(&mut self, path: P) -> Result<Metadata, ExifError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ExifError::InvalidPath(path.to_string_lossy().to_string()));
        }

        let output = Command::new("exiftool")
            .arg("-json")
            .arg("-g") // 按组组织输出
            .arg("-a") // 允许重复标签
            .arg("-u") // 包括未知标签
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ExifError::Execution(stderr.to_string()));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let metadata: Vec<Metadata> = serde_json::from_str(&json_str)?;

        metadata
            .into_iter()
            .next()
            .ok_or_else(|| ExifError::Execution("无元数据返回".to_string()))
    }

    /// 读取特定标签的元数据
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    /// * `tag` - 标签名
    ///
    /// # 返回值
    ///
    /// 返回标签的值，如果不存在则返回 None
    pub fn read_tag<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        path: P,
        tag: S,
    ) -> Result<Option<serde_json::Value>, ExifError> {
        let metadata = self.read_metadata(path)?;
        Ok(metadata.get(tag.as_ref()).cloned())
    }

    /// 写入元数据标签
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    /// * `tag` - 标签名
    /// * `value` - 标签值
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use exiftool_rs_wrapper::ExifTool;
    ///
    /// let mut exiftool = ExifTool::new().unwrap();
    /// exiftool.write_tag("photo.jpg", "Copyright", "© 2024").unwrap();
    /// ```
    pub fn write_tag<P: AsRef<Path>, S: AsRef<str>, V: AsRef<str>>(
        &mut self,
        path: P,
        tag: S,
        value: V,
    ) -> Result<(), ExifError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ExifError::InvalidPath(path.to_string_lossy().to_string()));
        }

        let tag_spec = format!("-{}={}", tag.as_ref(), value.as_ref());

        let output = Command::new("exiftool")
            .arg("-overwrite_original")
            .arg(&tag_spec)
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ExifError::Execution(stderr.to_string()));
        }

        Ok(())
    }

    /// 删除元数据标签
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    /// * `tag` - 要删除的标签名
    pub fn delete_tag<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        path: P,
        tag: S,
    ) -> Result<(), ExifError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ExifError::InvalidPath(path.to_string_lossy().to_string()));
        }

        let tag_spec = format!("-{}=", tag.as_ref());

        let output = Command::new("exiftool")
            .arg("-overwrite_original")
            .arg(&tag_spec)
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ExifError::Execution(stderr.to_string()));
        }

        Ok(())
    }

    /// 复制元数据从一个文件到另一个文件
    ///
    /// # 参数
    ///
    /// * `source` - 源文件路径
    /// * `target` - 目标文件路径
    pub fn copy_metadata<P: AsRef<Path>, Q: AsRef<Path>>(
        &mut self,
        source: P,
        target: Q,
    ) -> Result<(), ExifError> {
        let source = source.as_ref();
        let target = target.as_ref();

        if !source.exists() {
            return Err(ExifError::InvalidPath(source.to_string_lossy().to_string()));
        }

        if !target.exists() {
            return Err(ExifError::InvalidPath(target.to_string_lossy().to_string()));
        }

        let output = Command::new("exiftool")
            .arg("-overwrite_original")
            .arg("-TagsFromFile")
            .arg(source)
            .arg(target)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ExifError::Execution(stderr.to_string()));
        }

        Ok(())
    }

    /// 获取所有支持的标签列表
    ///
    /// # 返回值
    ///
    /// 返回所有支持的标签名称列表
    pub fn list_tags(&mut self) -> Result<Vec<String>, ExifError> {
        let output = Command::new("exiftool")
            .arg("-list")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ExifError::Execution(stderr.to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let tags: Vec<String> = stdout
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && !line.starts_with('-'))
            .collect();

        Ok(tags)
    }

    /// 获取 ExifTool 版本信息
    ///
    /// # 返回值
    ///
    /// 返回版本号字符串
    pub fn version(&self) -> Result<String, ExifError> {
        let output = Command::new("exiftool")
            .arg("-ver")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ExifError::Execution(stderr.to_string()));
        }

        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.trim().to_string())
    }
}

impl Default for ExifTool {
    fn default() -> Self {
        Self::new().expect("无法创建默认 ExifTool 实例")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[allow(dead_code)]
    fn create_test_image(dir: &TempDir, name: &str) -> std::path::PathBuf {
        // 创建一个简单的测试 JPEG 文件
        let path = dir.path().join(name);
        let mut file = File::create(&path).unwrap();
        // 写入最小的 JPEG 文件头
        file.write_all(&[
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xD9,
        ])
        .unwrap();
        path
    }

    #[test]
    fn test_exiftool_new() {
        // 此测试需要系统上安装 exiftool
        match ExifTool::new() {
            Ok(_) => println!("ExifTool 可用"),
            Err(ExifError::NotFound) => {
                println!("跳过测试: 未找到 ExifTool");
            }
            Err(e) => panic!("意外错误: {:?}", e),
        }
    }

    #[test]
    fn test_version() {
        match ExifTool::new() {
            Ok(exiftool) => {
                let version = exiftool.version().unwrap();
                assert!(!version.is_empty());
                println!("ExifTool 版本: {}", version);
            }
            Err(ExifError::NotFound) => {
                println!("跳过测试: 未找到 ExifTool");
            }
            Err(e) => panic!("意外错误: {:?}", e),
        }
    }
}
