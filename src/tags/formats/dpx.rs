//! DPX 图像格式标签

use crate::TagId;

pub const BIT_DEPTH: TagId = TagId("BitDepth");
pub const BYTE_ORDER: TagId = TagId("ByteOrder");
pub const COLORIMETRIC_SPECIFICATION: TagId = TagId("ColorimetricSpecification");
pub const COMPONENTS_CONFIGURATION: TagId = TagId("ComponentsConfiguration");
pub const D_P_X_FILE_SIZE: TagId = TagId("DPXFileSize");
pub const DATA_SIGN: TagId = TagId("DataSign");
pub const DITTO_KEY: TagId = TagId("DittoKey");
pub const ENCRYPTION_KEY: TagId = TagId("EncryptionKey");
pub const FRAME_I_D: TagId = TagId("FrameID");
pub const HEADER_VERSION: TagId = TagId("HeaderVersion");
pub const IMAGE2_DESCRIPTION: TagId = TagId("Image2Description");
pub const IMAGE3_DESCRIPTION: TagId = TagId("Image3Description");
pub const IMAGE4_DESCRIPTION: TagId = TagId("Image4Description");
pub const IMAGE5_DESCRIPTION: TagId = TagId("Image5Description");
pub const IMAGE6_DESCRIPTION: TagId = TagId("Image6Description");
pub const IMAGE7_DESCRIPTION: TagId = TagId("Image7Description");
pub const IMAGE8_DESCRIPTION: TagId = TagId("Image8Description");
pub const IMAGE_ELEMENTS: TagId = TagId("ImageElements");
pub const IMAGE_FILE_NAME: TagId = TagId("ImageFileName");
pub const INPUT_DEVICE_NAME: TagId = TagId("InputDeviceName");
pub const INPUT_DEVICE_SERIAL_NUMBER: TagId = TagId("InputDeviceSerialNumber");
pub const ORIGINAL_FRAME_RATE: TagId = TagId("OriginalFrameRate");
pub const PROJECT: TagId = TagId("Project");
pub const RESERVED5: TagId = TagId("Reserved5");
pub const SHUTTER_ANGLE: TagId = TagId("ShutterAngle");
pub const SLATE_INFORMATION: TagId = TagId("SlateInformation");
pub const SOURCE_CREATE_DATE: TagId = TagId("SourceCreateDate");
pub const SOURCE_FILE_NAME: TagId = TagId("SourceFileName");
pub const TIME_CODE: TagId = TagId("TimeCode");
pub const TRANSFER_CHARACTERISTIC: TagId = TagId("TransferCharacteristic");
pub const USER_I_D: TagId = TagId("UserID");
pub const X_OFFSET: TagId = TagId("XOffset");
pub const Y_OFFSET: TagId = TagId("YOffset");
