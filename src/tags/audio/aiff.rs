//! AIFF 音频格式标签

use crate::TagId;

pub const ANNOTATION: TagId = TagId("Annotation");
pub const AUTHOR: TagId = TagId("Author");
pub const COMMENT_TIME: TagId = TagId("CommentTime");
pub const COMPRESSION_TYPE: TagId = TagId("CompressionType");
pub const COMPRESSOR_NAME: TagId = TagId("CompressorName");
pub const FORMAT_VERSION: TagId = TagId("FormatVersion");
pub const FORMAT_VERSION_TIME: TagId = TagId("FormatVersionTime");
pub const NAME: TagId = TagId("Name");
pub const NUM_SAMPLE_FRAMES: TagId = TagId("NumSampleFrames");
pub const SAMPLE_RATE: TagId = TagId("SampleRate");
