//! 高级写入功能模块
//!
//! 支持日期偏移、条件写入、批量写入等高级功能

use crate::ExifTool;
use crate::error::Result;
use crate::types::TagId;
use crate::write::WriteBuilder;
use std::path::Path;

/// 日期偏移方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateShiftDirection {
    /// 增加
    Add,
    /// 减少
    Subtract,
}

/// 时间单位
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    /// 年
    Years,
    /// 月
    Months,
    /// 日
    Days,
    /// 时
    Hours,
    /// 分
    Minutes,
    /// 秒
    Seconds,
}

impl TimeUnit {
    /// 获取单位缩写
    #[allow(dead_code)]
    fn abbreviation(&self) -> &'static str {
        match self {
            Self::Years => "y",
            Self::Months => "m",
            Self::Days => "d",
            Self::Hours => "H",
            Self::Minutes => "M",
            Self::Seconds => "S",
        }
    }
}

/// 日期时间偏移量
#[derive(Debug, Clone)]
pub struct DateTimeOffset {
    years: i32,
    months: i32,
    days: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
}

impl DateTimeOffset {
    /// 创建新的偏移量
    pub fn new() -> Self {
        Self {
            years: 0,
            months: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    /// 设置年
    pub fn years(mut self, value: i32) -> Self {
        self.years = value;
        self
    }

    /// 设置月
    pub fn months(mut self, value: i32) -> Self {
        self.months = value;
        self
    }

    /// 设置日
    pub fn days(mut self, value: i32) -> Self {
        self.days = value;
        self
    }

    /// 设置时
    pub fn hours(mut self, value: i32) -> Self {
        self.hours = value;
        self
    }

    /// 设置分
    pub fn minutes(mut self, value: i32) -> Self {
        self.minutes = value;
        self
    }

    /// 设置秒
    pub fn seconds(mut self, value: i32) -> Self {
        self.seconds = value;
        self
    }

    /// 格式化为 ExifTool 格式
    /// 格式: "+y:m:d H:M:S" 或 "-y:m:d H:M:S"
    pub fn format(&self) -> String {
        let sign = if self.is_negative() { "-" } else { "+" };
        format!(
            "{}{}:{:02}:{:02} {:02}:{:02}:{:02}",
            sign,
            self.years.abs(),
            self.months.abs(),
            self.days.abs(),
            self.hours.abs(),
            self.minutes.abs(),
            self.seconds.abs()
        )
    }

    fn is_negative(&self) -> bool {
        self.years < 0
            || self.months < 0
            || self.days < 0
            || self.hours < 0
            || self.minutes < 0
            || self.seconds < 0
    }
}

impl Default for DateTimeOffset {
    fn default() -> Self {
        Self::new()
    }
}

/// 高级写入操作 trait
pub trait AdvancedWriteOperations {
    /// 偏移日期时间标签
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// use exiftool_rs_wrapper::ExifTool;
    /// use exiftool_rs_wrapper::advanced::AdvancedWriteOperations;
    /// use exiftool_rs_wrapper::advanced::DateTimeOffset;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let exiftool = ExifTool::new()?;
    ///
    /// // 将所有日期时间增加 1 天 2 小时
    /// exiftool.shift_datetime("photo.jpg", DateTimeOffset::new().days(1).hours(2))?;
    /// # Ok(())
    /// # }
    /// ```
    fn shift_datetime<P: AsRef<Path>>(&self, path: P, offset: DateTimeOffset) -> Result<()>;

    /// 仅偏移特定日期时间标签
    fn shift_specific_datetime<P: AsRef<Path>>(
        &self,
        path: P,
        tag: TagId,
        offset: DateTimeOffset,
    ) -> Result<()>;

    /// 数值运算
    fn numeric_operation<P: AsRef<Path>>(
        &self,
        path: P,
        tag: TagId,
        operation: NumericOperation,
    ) -> Result<()>;

    /// 字符串追加
    fn append_string<P: AsRef<Path>>(&self, path: P, tag: TagId, suffix: &str) -> Result<()>;

    /// 条件写入
    fn write_if<P: AsRef<Path>, F>(&self, path: P, condition: &str, builder_fn: F) -> Result<()>
    where
        F: FnOnce(WriteBuilder<'_>) -> WriteBuilder<'_>;
}

/// 数值运算类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericOperation {
    /// 加法
    Add(i64),
    /// 减法
    Subtract(i64),
    /// 乘法
    Multiply(i64),
    /// 除法
    Divide(i64),
}

impl NumericOperation {
    fn operator(&self) -> &'static str {
        match self {
            Self::Add(_) => "+=",
            Self::Subtract(_) => "-=",
            Self::Multiply(_) => "*=",
            Self::Divide(_) => "/=",
        }
    }

    fn value(&self) -> i64 {
        match self {
            Self::Add(v) => *v,
            Self::Subtract(v) => *v,
            Self::Multiply(v) => *v,
            Self::Divide(v) => *v,
        }
    }
}

impl AdvancedWriteOperations for ExifTool {
    fn shift_datetime<P: AsRef<Path>>(&self, path: P, offset: DateTimeOffset) -> Result<()> {
        let offset_str = offset.format();

        self.write(path)
            .arg(format!("-AllDates{}", offset_str))
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }

    fn shift_specific_datetime<P: AsRef<Path>>(
        &self,
        path: P,
        tag: TagId,
        offset: DateTimeOffset,
    ) -> Result<()> {
        let offset_str = offset.format();

        self.write(path)
            .arg(format!("-{}{}", tag.name(), offset_str))
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }

    fn numeric_operation<P: AsRef<Path>>(
        &self,
        path: P,
        tag: TagId,
        operation: NumericOperation,
    ) -> Result<()> {
        let op_str = format!(
            "-{}{}{}",
            tag.name(),
            operation.operator(),
            operation.value()
        );

        self.write(path)
            .arg(op_str)
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }

    fn append_string<P: AsRef<Path>>(&self, path: P, tag: TagId, suffix: &str) -> Result<()> {
        self.write(path)
            .arg(format!("-{}+={}", tag.name(), suffix))
            .overwrite_original(true)
            .execute()?;

        Ok(())
    }

    fn write_if<P: AsRef<Path>, F>(&self, path: P, _condition: &str, builder_fn: F) -> Result<()>
    where
        F: FnOnce(WriteBuilder<'_>) -> WriteBuilder<'_>,
    {
        // 使用 builder_fn 构建 WriteBuilder，它已经可以通过 .condition() 设置条件
        let builder = self.write(path);
        let builder = builder_fn(builder);
        builder.execute().map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_offset_format() {
        let offset = DateTimeOffset::new().days(1).hours(2).minutes(30);

        assert_eq!(offset.format(), "+0:00:01 02:30:00");
    }

    #[test]
    fn test_datetime_offset_negative() {
        let offset = DateTimeOffset::new().days(-1).hours(-2);

        assert_eq!(offset.format(), "-0:00:01 02:00:00");
    }

    #[test]
    fn test_numeric_operation() {
        let op = NumericOperation::Add(5);
        assert_eq!(op.operator(), "+=");
        assert_eq!(op.value(), 5);

        let op = NumericOperation::Multiply(2);
        assert_eq!(op.operator(), "*=");
        assert_eq!(op.value(), 2);
    }
}
