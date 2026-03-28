#![allow(non_upper_case_globals)]

//! 标签定义 - formats/tiff

use crate::TagId;

pub const DepthMapTiff: TagId = TagId("DepthMapTiff");
pub const GeoTiffAsciiParams: TagId = TagId("GeoTiffAsciiParams");
pub const GeoTiffDirectory: TagId = TagId("GeoTiffDirectory");
pub const GeoTiffDoubleParams: TagId = TagId("GeoTiffDoubleParams");
pub const GeoTiffVersion: TagId = TagId("GeoTiffVersion");
pub const PreviewTIFF: TagId = TagId("PreviewTIFF");
pub const SingleShotDepthMapTiff: TagId = TagId("SingleShotDepthMapTiff");
pub const TIFF_EPStandardID: TagId = TagId("TIFF-EPStandardID");
pub const TIFFHandling: TagId = TagId("TIFFHandling");
pub const TIFFPreview: TagId = TagId("TIFFPreview");
pub const TIFFSummary: TagId = TagId("TIFFSummary");
pub const TIFF_FXExtensions: TagId = TagId("TIFF_FXExtensions");
pub const ThumbnailTIFF: TagId = TagId("ThumbnailTIFF");
pub const TiffMeteringImage: TagId = TagId("TiffMeteringImage");
pub const TiffMeteringImageHeight: TagId = TagId("TiffMeteringImageHeight");
pub const TiffMeteringImageWidth: TagId = TagId("TiffMeteringImageWidth");
