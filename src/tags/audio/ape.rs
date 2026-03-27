//! APE 音频格式标签

use crate::TagId;

pub const A_P_E_VERSION: TagId = TagId("APEVersion");
pub const BLOCKS_PER_FRAME: TagId = TagId("BlocksPerFrame");
pub const FINAL_FRAME_BLOCKS: TagId = TagId("FinalFrameBlocks");
pub const SAMPLE_RATE: TagId = TagId("SampleRate");
pub const TOOL_NAME: TagId = TagId("ToolName");
pub const TOOL_VERSION: TagId = TagId("ToolVersion");
pub const TOTAL_FRAMES: TagId = TagId("TotalFrames");
