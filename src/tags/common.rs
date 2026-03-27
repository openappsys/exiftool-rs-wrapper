//! 基础标签 - 总是包含

use crate::TagId;

impl TagId {
    // 最常用的基础标签
    pub const MAKE: Self = Self("Make");
    pub const MODEL: Self = Self("Model");
    pub const DATE_TIME_ORIGINAL: Self = Self("DateTimeOriginal");
    pub const CREATE_DATE: Self = Self("CreateDate");
    pub const MODIFY_DATE: Self = Self("ModifyDate");
    pub const FILE_NAME: Self = Self("FileName");
    pub const FILE_SIZE: Self = Self("FileSize");
    pub const FILE_TYPE: Self = Self("FileType");
    pub const MIME_TYPE: Self = Self("MIMEType");
}
