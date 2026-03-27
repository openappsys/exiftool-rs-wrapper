//! PCX 图像格式标签

use crate::TagId;

pub const BYTES_PER_LINE: TagId = TagId("BytesPerLine");
pub const ENCODING: TagId = TagId("Encoding");
pub const LEFT_MARGIN: TagId = TagId("LeftMargin");
pub const MANUFACTURER: TagId = TagId("Manufacturer");
pub const SCREEN_HEIGHT: TagId = TagId("ScreenHeight");
pub const SCREEN_WIDTH: TagId = TagId("ScreenWidth");
pub const TOP_MARGIN: TagId = TagId("TopMargin");
