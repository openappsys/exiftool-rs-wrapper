//! 错误恢复和重试机制
//!
//! 提供错误恢复策略、重试机制、部分成功处理

use crate::error::Error;
use std::time::Duration;

/// 重试策略
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// 最大重试次数
    pub max_attempts: u32,
    /// 初始延迟
    pub initial_delay: Duration,
    /// 延迟倍数（指数退避）
    pub backoff_multiplier: f64,
    /// 最大延迟
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryPolicy {
    /// 创建新的重试策略
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            ..Default::default()
        }
    }

    /// 设置初始延迟
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// 设置退避倍数
    pub fn backoff(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// 计算第 n 次重试的延迟
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        let delay_ms = self.initial_delay.as_millis() as f64
            * self.backoff_multiplier.powi(attempt as i32 - 1);
        let delay_ms = delay_ms.min(self.max_delay.as_millis() as f64) as u64;

        Duration::from_millis(delay_ms)
    }

    /// 检查是否应该重试
    pub fn should_retry(&self, attempt: u32, error: &Error) -> bool {
        if attempt >= self.max_attempts {
            return false;
        }

        // 只对特定错误进行重试
        matches!(
            error,
            Error::Io(_) | Error::Timeout | Error::Process { .. } | Error::MutexPoisoned
        )
    }
}

/// 执行带重试的操作（需要启用 async 特性）
#[cfg(feature = "async")]
pub async fn with_retry<F, Fut, T>(policy: &RetryPolicy, operation: F) -> crate::error::Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = crate::error::Result<T>>,
{
    let mut attempt = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if !policy.should_retry(attempt, &error) {
                    return Err(error);
                }

                attempt += 1;
                let delay = policy.delay_for_attempt(attempt);

                if delay > Duration::ZERO {
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

/// 同步版本的重试
pub fn with_retry_sync<F, T>(policy: &RetryPolicy, operation: F) -> crate::error::Result<T>
where
    F: Fn() -> crate::error::Result<T>,
{
    let mut attempt = 0;

    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) => {
                if !policy.should_retry(attempt, &error) {
                    return Err(error);
                }

                attempt += 1;
                let delay = policy.delay_for_attempt(attempt);

                if delay > Duration::ZERO {
                    std::thread::sleep(delay);
                }
            }
        }
    }
}

/// 批量操作结果（支持部分成功）
#[derive(Debug)]
pub struct BatchResult<T, E> {
    /// 成功的结果
    pub successes: Vec<T>,
    /// 失败的错误
    pub failures: Vec<E>,
    /// 总数量
    pub total: usize,
}

impl<T, E> BatchResult<T, E> {
    /// 创建新的批量结果
    pub fn new() -> Self {
        Self {
            successes: Vec::new(),
            failures: Vec::new(),
            total: 0,
        }
    }

    /// 添加成功项
    pub fn add_success(&mut self, item: T) {
        self.successes.push(item);
        self.total += 1;
    }

    /// 添加失败项
    pub fn add_failure(&mut self, error: E) {
        self.failures.push(error);
        self.total += 1;
    }

    /// 检查是否全部成功
    pub fn is_complete(&self) -> bool {
        self.failures.is_empty()
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.successes.len() as f64 / self.total as f64) * 100.0
        }
    }

    /// 获取失败率
    pub fn failure_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.failures.len() as f64 / self.total as f64) * 100.0
        }
    }
}

impl<T, E> Default for BatchResult<T, E> {
    fn default() -> Self {
        Self::new()
    }
}

/// 可恢复的错误 trait
pub trait Recoverable {
    /// 检查是否可恢复
    fn is_recoverable(&self) -> bool;

    /// 获取恢复建议
    fn recovery_suggestion(&self) -> Option<String>;
}

impl Recoverable for Error {
    fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Error::Io(_) | Error::Timeout | Error::Process { .. } | Error::MutexPoisoned
        )
    }

    fn recovery_suggestion(&self) -> Option<String> {
        match self {
            Error::Io(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Some("请检查文件路径是否正确".to_string())
            }
            Error::Timeout => Some("请增加超时时间或检查网络连接".to_string()),
            Error::ExifToolNotFound => Some("请安装 ExifTool 并添加到 PATH".to_string()),
            Error::MutexPoisoned => Some("内部错误，请重新创建 ExifTool 实例".to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy() {
        let policy = RetryPolicy::default();

        assert_eq!(policy.delay_for_attempt(0), Duration::ZERO);
        assert_eq!(policy.delay_for_attempt(1), Duration::from_millis(100));
        assert_eq!(policy.delay_for_attempt(2), Duration::from_millis(200));
    }

    #[test]
    fn test_retry_policy_builder() {
        let policy = RetryPolicy::new(5)
            .initial_delay(Duration::from_secs(1))
            .backoff(3.0);

        assert_eq!(policy.max_attempts, 5);
        assert_eq!(policy.initial_delay, Duration::from_secs(1));
        assert_eq!(policy.backoff_multiplier, 3.0);
    }

    #[test]
    fn test_batch_result() {
        let mut result: BatchResult<i32, String> = BatchResult::new();

        result.add_success(1);
        result.add_success(2);
        result.add_failure("error".to_string());

        assert_eq!(result.total, 3);
        assert_eq!(result.successes.len(), 2);
        assert_eq!(result.failures.len(), 1);
        assert!(!result.is_complete());
        assert!((result.success_rate() - 66.66666666666667).abs() < 1e-10);
    }

    #[test]
    fn test_recoverable() {
        let timeout_err = Error::Timeout;
        assert!(timeout_err.is_recoverable());
        assert!(timeout_err.recovery_suggestion().is_some());

        let tag_err = Error::TagNotFound("test".to_string());
        assert!(!tag_err.is_recoverable());
    }

    #[test]
    fn test_retry_sync() {
        use std::cell::Cell;
        let policy = RetryPolicy::new(2);
        let attempts = Cell::new(0);

        // 测试成功的情况
        let result = with_retry_sync(&policy, || {
            attempts.set(attempts.get() + 1);
            Ok(42)
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts.get(), 1);
    }

    #[test]
    fn test_retry_sync_failure() {
        use std::cell::Cell;
        let policy = RetryPolicy::new(2);
        let attempts = Cell::new(0);

        // 测试失败的情况
        let result: Result<i32, _> = with_retry_sync(&policy, || {
            attempts.set(attempts.get() + 1);
            Err(Error::TagNotFound("test".to_string()))
        });

        assert!(result.is_err());
        // TagNotFound 不可恢复，所以只尝试一次
        assert_eq!(attempts.get(), 1);
    }
}
