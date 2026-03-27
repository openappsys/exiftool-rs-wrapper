//! 音频元数据标签（FLAC、ID3、RIFF、Vorbis、Opus、AIFF、AAC 等）

pub mod aac;
pub mod aiff;
pub mod ape;
pub mod audible;
pub mod flac;
pub mod id3;
pub mod opus;
pub mod riff;
pub mod vorbis;
pub mod wavpack;

pub use aac::*;
pub use aiff::*;
pub use ape::*;
pub use audible::*;
pub use flac::*;
pub use id3::*;
pub use opus::*;
pub use riff::*;
pub use vorbis::*;
pub use wavpack::*;
