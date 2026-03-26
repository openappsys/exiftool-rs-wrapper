//! ExifTool 错误类型定义

use std::io;
use thiserror::Error;

/// 所有 ExifTool 操作的错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// 进程错误
    #[error("ExifTool process error: {message}")]
    Process {
        message: String,
        exit_code: Option<i32>,
    },

    /// 解析错误
    #[error("Parse error: {0}")]
    Parse(String),

    /// 标签未找到
    #[error("Tag not found: {0}")]
    TagNotFound(String),

    /// 无效参数
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// ExifTool 未找到
    #[error("ExifTool not found in PATH. Please install ExifTool: https://exiftool.org/")]
    ExifToolNotFound,

    /// 互斥锁被污染
    #[error("Internal mutex poisoned")]
    MutexPoisoned,

    /// 序列化错误
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// 超时
    #[error("Operation timed out")]
    Timeout,
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// 创建进程错误
    pub fn process(message: impl Into<String>) -> Self {
        Self::Process {
            message: message.into(),
            exit_code: None,
        }
    }

    /// 创建带退出码的进程错误
    pub fn process_with_code(message: impl Into<String>, code: i32) -> Self {
        Self::Process {
            message: message.into(),
            exit_code: Some(code),
        }
    }

    /// 创建解析错误
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }

    /// 创建无效参数错误
    pub fn invalid_arg(message: impl Into<String>) -> Self {
        Self::InvalidArgument(message.into())
    }
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, crate::process::ExifToolInner>>>
    for Error
{
    fn from(
        _: std::sync::PoisonError<std::sync::MutexGuard<'_, crate::process::ExifToolInner>>,
    ) -> Self {
        Error::MutexPoisoned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::process("test error");
        assert!(matches!(err, Error::Process { .. }));
        assert!(err.to_string().contains("test error"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }
}
