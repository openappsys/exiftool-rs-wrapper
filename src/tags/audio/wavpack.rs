//! WAVPACK 音频格式标签

use crate::TagId;

pub const AUDIO_TYPE: TagId = TagId("AudioType");
pub const BYTES_PER_SAMPLE: TagId = TagId("BytesPerSample");
pub const COMPRESSION: TagId = TagId("Compression");
pub const DATA_FORMAT: TagId = TagId("DataFormat");
pub const SAMPLE_RATE: TagId = TagId("SampleRate");
