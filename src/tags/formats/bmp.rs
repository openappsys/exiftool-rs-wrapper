//! BMP 图像格式标签

use crate::TagId;

pub const ALPHA_MASK: TagId = TagId("AlphaMask");
pub const B_M_P_VERSION: TagId = TagId("BMPVersion");
pub const BIT_DEPTH: TagId = TagId("BitDepth");
pub const BLUE_ENDPOINT: TagId = TagId("BlueEndpoint");
pub const BLUE_MASK: TagId = TagId("BlueMask");
pub const COMPRESSION: TagId = TagId("Compression");
pub const GAMMA_BLUE: TagId = TagId("GammaBlue");
pub const GAMMA_GREEN: TagId = TagId("GammaGreen");
pub const GAMMA_RED: TagId = TagId("GammaRed");
pub const GREEN_ENDPOINT: TagId = TagId("GreenEndpoint");
pub const GREEN_MASK: TagId = TagId("GreenMask");
pub const IMAGE_LENGTH: TagId = TagId("ImageLength");
pub const NUM_COLORS: TagId = TagId("NumColors");
pub const NUM_IMPORTANT_COLORS: TagId = TagId("NumImportantColors");
pub const PIXELS_PER_METER_X: TagId = TagId("PixelsPerMeterX");
pub const PIXELS_PER_METER_Y: TagId = TagId("PixelsPerMeterY");
pub const PLANES: TagId = TagId("Planes");
pub const PROFILE_DATA_OFFSET: TagId = TagId("ProfileDataOffset");
pub const PROFILE_SIZE: TagId = TagId("ProfileSize");
pub const RED_ENDPOINT: TagId = TagId("RedEndpoint");
pub const RED_MASK: TagId = TagId("RedMask");
pub const RENDERING_INTENT: TagId = TagId("RenderingIntent");
