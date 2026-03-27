//! FLAC 音频格式标签

use crate::TagId;

pub const APPLICATION_UNKNOWN: TagId = TagId("ApplicationUnknown");
pub const CHANNELS: TagId = TagId("Channels");
pub const CUE_SHEET: TagId = TagId("CueSheet");
pub const M_D5_SIGNATURE: TagId = TagId("MD5Signature");
pub const PADDING: TagId = TagId("Padding");
pub const PICTURE: TagId = TagId("Picture");
pub const PICTURE_DESCRIPTION: TagId = TagId("PictureDescription");
pub const PICTURE_M_I_M_E_TYPE: TagId = TagId("PictureMIMEType");
pub const PICTURE_TYPE: TagId = TagId("PictureType");
pub const SEEK_TABLE: TagId = TagId("SeekTable");
