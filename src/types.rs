//! 核心类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// 标签标识符 - 提供类型安全的标签访问
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(&'static str);

impl TagId {
    /// 创建新的标签标识符
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// 获取标签名称
    pub fn name(&self) -> &str {
        self.0
    }

    // === 常用 EXIF 标签 ===
    pub const MAKE: Self = Self("Make");
    pub const MODEL: Self = Self("Model");
    pub const DATE_TIME_ORIGINAL: Self = Self("DateTimeOriginal");
    pub const CREATE_DATE: Self = Self("CreateDate");
    pub const MODIFY_DATE: Self = Self("ModifyDate");
    pub const IMAGE_WIDTH: Self = Self("ImageWidth");
    pub const IMAGE_HEIGHT: Self = Self("ImageHeight");
    pub const ORIENTATION: Self = Self("Orientation");
    pub const X_RESOLUTION: Self = Self("XResolution");
    pub const Y_RESOLUTION: Self = Self("YResolution");
    pub const RESOLUTION_UNIT: Self = Self("ResolutionUnit");
    pub const SOFTWARE: Self = Self("Software");
    pub const COPYRIGHT: Self = Self("Copyright");
    pub const ARTIST: Self = Self("Artist");
    pub const IMAGE_DESCRIPTION: Self = Self("ImageDescription");

    // === 相机设置标签 ===
    pub const EXPOSURE_TIME: Self = Self("ExposureTime");
    pub const F_NUMBER: Self = Self("FNumber");
    pub const EXPOSURE_PROGRAM: Self = Self("ExposureProgram");
    pub const ISO: Self = Self("ISO");
    pub const SENSITIVITY_TYPE: Self = Self("SensitivityType");
    pub const RECOMMENDED_EXPOSURE_INDEX: Self = Self("RecommendedExposureIndex");
    pub const EXIF_VERSION: Self = Self("ExifVersion");
    pub const DATE_TIME_DIGITIZED: Self = Self("DateTimeDigitized");
    pub const COMPONENT_CONFIGURATION: Self = Self("ComponentConfiguration");
    pub const SHUTTER_SPEED_VALUE: Self = Self("ShutterSpeedValue");
    pub const APERTURE_VALUE: Self = Self("ApertureValue");
    pub const BRIGHTNESS_VALUE: Self = Self("BrightnessValue");
    pub const EXPOSURE_COMPENSATION: Self = Self("ExposureCompensation");
    pub const MAX_APERTURE_VALUE: Self = Self("MaxApertureValue");
    pub const SUBJECT_DISTANCE: Self = Self("SubjectDistance");
    pub const METERING_MODE: Self = Self("MeteringMode");
    pub const LIGHT_SOURCE: Self = Self("LightSource");
    pub const FLASH: Self = Self("Flash");
    pub const FOCAL_LENGTH: Self = Self("FocalLength");
    pub const FOCAL_LENGTH_IN_35MM_FORMAT: Self = Self("FocalLengthIn35mmFormat");
    pub const FLASH_ENERGY: Self = Self("FlashEnergy");
    pub const SPATIAL_FREQUENCY_RESPONSE: Self = Self("SpatialFrequencyResponse");
    pub const FOCAL_PLANE_X_RESOLUTION: Self = Self("FocalPlaneXResolution");
    pub const FOCAL_PLANE_Y_RESOLUTION: Self = Self("FocalPlaneYResolution");
    pub const FOCAL_PLANE_RESOLUTION_UNIT: Self = Self("FocalPlaneResolutionUnit");
    pub const SUBJECT_LOCATION: Self = Self("SubjectLocation");
    pub const EXPOSURE_INDEX: Self = Self("ExposureIndex");
    pub const SENSING_METHOD: Self = Self("SensingMethod");
    pub const FILE_SOURCE: Self = Self("FileSource");
    pub const SCENE_TYPE: Self = Self("SceneType");
    pub const CFA_PATTERN: Self = Self("CFAPattern");
    pub const CUSTOM_RENDERED: Self = Self("CustomRendered");
    pub const EXPOSURE_MODE: Self = Self("ExposureMode");
    pub const WHITE_BALANCE: Self = Self("WhiteBalance");
    pub const DIGITAL_ZOOM_RATIO: Self = Self("DigitalZoomRatio");
    pub const FOCAL_LENGTH_35EFL: Self = Self("FocalLength35efl");
    pub const SCENE_CAPTURE_TYPE: Self = Self("SceneCaptureType");
    pub const GAIN_CONTROL: Self = Self("GainControl");
    pub const CONTRAST: Self = Self("Contrast");
    pub const SATURATION: Self = Self("Saturation");
    pub const SHARPNESS: Self = Self("Sharpness");
    pub const DEVICE_SETTING_DESCRIPTION: Self = Self("DeviceSettingDescription");
    pub const SUBJECT_DISTANCE_RANGE: Self = Self("SubjectDistanceRange");

    // === GPS 标签 ===
    pub const GPS_LATITUDE_REF: Self = Self("GPSLatitudeRef");
    pub const GPS_LATITUDE: Self = Self("GPSLatitude");
    pub const GPS_LONGITUDE_REF: Self = Self("GPSLongitudeRef");
    pub const GPS_LONGITUDE: Self = Self("GPSLongitude");
    pub const GPS_ALTITUDE_REF: Self = Self("GPSAltitudeRef");
    pub const GPS_ALTITUDE: Self = Self("GPSAltitude");
    pub const GPS_TIMESTAMP: Self = Self("GPSTimeStamp");
    pub const GPS_SATELLITES: Self = Self("GPSSatellites");
    pub const GPS_STATUS: Self = Self("GPSStatus");
    pub const GPS_MEASURE_MODE: Self = Self("GPSMeasureMode");
    pub const GPS_DOP: Self = Self("GPSDOP");
    pub const GPS_SPEED_REF: Self = Self("GPSSpeedRef");
    pub const GPS_SPEED: Self = Self("GPSSpeed");
    pub const GPS_TRACK_REF: Self = Self("GPSTrackRef");
    pub const GPS_TRACK: Self = Self("GPSTrack");
    pub const GPS_IMG_DIRECTION_REF: Self = Self("GPSImgDirectionRef");
    pub const GPS_IMG_DIRECTION: Self = Self("GPSImgDirection");
    pub const GPS_MAP_DATUM: Self = Self("GPSMapDatum");
    pub const GPS_DEST_LATITUDE_REF: Self = Self("GPSDestLatitudeRef");
    pub const GPS_DEST_LATITUDE: Self = Self("GPSDestLatitude");
    pub const GPS_DEST_LONGITUDE_REF: Self = Self("GPSDestLongitudeRef");
    pub const GPS_DEST_LONGITUDE: Self = Self("GPSDestLongitude");
    pub const GPS_DEST_BEARING_REF: Self = Self("GPSDestBearingRef");
    pub const GPS_DEST_BEARING: Self = Self("GPSDestBearing");
    pub const GPS_DEST_DISTANCE_REF: Self = Self("GPSDestDistanceRef");
    pub const GPS_DEST_DISTANCE: Self = Self("GPSDestDistance");
    pub const GPS_PROCESSING_METHOD: Self = Self("GPSProcessingMethod");
    pub const GPS_AREA_INFORMATION: Self = Self("GPSAreaInformation");
    pub const GPS_DATE_STAMP: Self = Self("GPSDateStamp");
    pub const GPS_DIFFERENTIAL: Self = Self("GPSDifferential");
    pub const GPS_H_POSITIONING_ERROR: Self = Self("GPSHPositioningError");

    // === 文件信息标签 ===
    pub const FILE_NAME: Self = Self("FileName");
    pub const DIRECTORY: Self = Self("Directory");
    pub const FILE_SIZE: Self = Self("FileSize");
    pub const FILE_MODIFY_DATE: Self = Self("FileModifyDate");
    pub const FILE_ACCESS_DATE: Self = Self("FileAccessDate");
    pub const FILE_INODE_CHANGE_DATE: Self = Self("FileInodeChangeDate");
    pub const FILE_PERMISSIONS: Self = Self("FilePermissions");
    pub const FILE_TYPE: Self = Self("FileType");
    pub const FILE_TYPE_EXTENSION: Self = Self("FileTypeExtension");
    pub const MIME_TYPE: Self = Self("MIMEType");
    pub const EXIF_BYTE_ORDER: Self = Self("ExifByteOrder");
    pub const CURRENT_ICC_PROFILE: Self = Self("CurrentICCProfile");
    pub const PROFILE_DATE_TIME: Self = Self("ProfileDateTime");
    pub const PROFILE_FILE_SIGNATURE: Self = Self("ProfileFileSignature");
    pub const PRIMARY_PLATFORM: Self = Self("PrimaryPlatform");
    pub const CMM_TYPE: Self = Self("CMMType");
    pub const PROFILE_VERSION: Self = Self("ProfileVersion");
    pub const PROFILE_CLASS: Self = Self("ProfileClass");
    pub const COLOR_SPACE_DATA: Self = Self("ColorSpaceData");
    pub const PROFILE_CONNECTION_SPACE: Self = Self("ProfileConnectionSpace");
    pub const PROFILE_CONNECTION_SPACE_ILLUMINANT: Self = Self("ProfileConnectionSpaceIlluminant");
    pub const ICC_PROFILE_CREATOR: Self = Self("ICCProfileCreator");
    pub const ICC_PROFILE_DESCRIPTION: Self = Self("ICCProfileDescription");
    pub const ICC_VIEWING_CONDITIONS_DESCRIPTION: Self = Self("ICCViewingConditionsDescription");
    pub const ICC_DEVICE_MODEL: Self = Self("ICCDeviceModel");
    pub const ICC_DEVICE_MANUFACTURER: Self = Self("ICCDeviceManufacturer");

    // === IPTC 标签 ===
    pub const IPTC_OBJECT_NAME: Self = Self("ObjectName");
    pub const IPTC_EDIT_STATUS: Self = Self("EditStatus");
    pub const IPTC_EDITORIAL_UPDATE: Self = Self("EditorialUpdate");
    pub const IPTC_URGENCY: Self = Self("Urgency");
    pub const IPTC_SUBJECT_REFERENCE: Self = Self("SubjectReference");
    pub const IPTC_CATEGORY: Self = Self("Category");
    pub const IPTC_SUPPLEMENTAL_CATEGORY: Self = Self("SupplementalCategory");
    pub const IPTC_FIXTURE_IDENTIFIER: Self = Self("FixtureIdentifier");
    pub const IPTC_KEYWORDS: Self = Self("Keywords");
    pub const IPTC_CONTENT_LOCATION_CODE: Self = Self("ContentLocationCode");
    pub const IPTC_CONTENT_LOCATION_NAME: Self = Self("ContentLocationName");
    pub const IPTC_RELEASE_DATE: Self = Self("ReleaseDate");
    pub const IPTC_RELEASE_TIME: Self = Self("ReleaseTime");
    pub const IPTC_EXPIRATION_DATE: Self = Self("ExpirationDate");
    pub const IPTC_EXPIRATION_TIME: Self = Self("ExpirationTime");
    pub const IPTC_SPECIAL_INSTRUCTIONS: Self = Self("SpecialInstructions");
    pub const IPTC_ACTION_ADVISED: Self = Self("ActionAdvised");
    pub const IPTC_REFERENCE_SERVICE: Self = Self("ReferenceService");
    pub const IPTC_REFERENCE_DATE: Self = Self("ReferenceDate");
    pub const IPTC_REFERENCE_NUMBER: Self = Self("ReferenceNumber");
    pub const IPTC_DATE_CREATED: Self = Self("DateCreated");
    pub const IPTC_TIME_CREATED: Self = Self("TimeCreated");
    pub const IPTC_DIGITAL_CREATION_DATE: Self = Self("DigitalCreationDate");
    pub const IPTC_DIGITAL_CREATION_TIME: Self = Self("DigitalCreationTime");
    pub const IPTC_ORIGINATING_PROGRAM: Self = Self("OriginatingProgram");
    pub const IPTC_PROGRAM_VERSION: Self = Self("ProgramVersion");
    pub const IPTC_OBJECT_CYCLE: Self = Self("ObjectCycle");
    pub const IPTC_BY_LINE: Self = Self("By-line");
    pub const IPTC_BY_LINE_TITLE: Self = Self("By-lineTitle");
    pub const IPTC_CITY: Self = Self("City");
    pub const IPTC_SUB_LOCATION: Self = Self("Sub-location");
    pub const IPTC_PROVINCE_STATE: Self = Self("Province-State");
    pub const IPTC_COUNTRY_PRIMARY_LOCATION_CODE: Self = Self("Country-PrimaryLocationCode");
    pub const IPTC_COUNTRY_PRIMARY_LOCATION_NAME: Self = Self("Country-PrimaryLocationName");
    pub const IPTC_ORIGINAL_TRANSMISSION_REFERENCE: Self = Self("OriginalTransmissionReference");
    pub const IPTC_HEADLINE: Self = Self("Headline");
    pub const IPTC_CREDIT: Self = Self("Credit");
    pub const IPTC_SOURCE: Self = Self("Source");
    pub const IPTC_COPYRIGHT_NOTICE: Self = Self("CopyrightNotice");
    pub const IPTC_CONTACT: Self = Self("Contact");
    pub const IPTC_CAPTION_ABSTRACT: Self = Self("Caption-Abstract");
    pub const IPTC_WRITER_EDITOR: Self = Self("Writer-Editor");
    pub const IPTC_IMAGE_TYPE: Self = Self("ImageType");
    pub const IPTC_IMAGE_ORIENTATION: Self = Self("ImageOrientation");
    pub const IPTC_LANGUAGE_IDENTIFIER: Self = Self("LanguageIdentifier");

    // === XMP 标签 ( Dublin Core ) ===
    pub const XMP_DC_TITLE: Self = Self("Title");
    pub const XMP_DC_CREATOR: Self = Self("Creator");
    pub const XMP_DC_SUBJECT: Self = Self("Subject");
    pub const XMP_DC_DESCRIPTION: Self = Self("Description");
    pub const XMP_DC_PUBLISHER: Self = Self("Publisher");
    pub const XMP_DC_CONTRIBUTOR: Self = Self("Contributor");
    pub const XMP_DC_DATE: Self = Self("Date");
    pub const XMP_DC_TYPE: Self = Self("Type");
    pub const XMP_DC_FORMAT: Self = Self("Format");
    pub const XMP_DC_IDENTIFIER: Self = Self("Identifier");
    pub const XMP_DC_SOURCE: Self = Self("Source");
    pub const XMP_DC_LANGUAGE: Self = Self("Language");
    pub const XMP_DC_RELATION: Self = Self("Relation");
    pub const XMP_DC_COVERAGE: Self = Self("Coverage");
    pub const XMP_DC_RIGHTS: Self = Self("Rights");

    // === XMP 标签 ( XMP Rights ) ===
    pub const XMP_XMP_RIGHTS_MANAGED: Self = Self("RightsManaged");
    pub const XMP_XMP_RIGHTS_MARKED: Self = Self("RightsMarked");
    pub const XMP_XMP_RIGHTS_WEB_STATEMENT: Self = Self("WebStatement");
    pub const XMP_XMP_RIGHTS_USAGE_TERMS: Self = Self("UsageTerms");

    // === 图像尺寸标签 ===
    pub const IMAGE_SIZE: Self = Self("ImageSize");
    pub const MEGAPIXELS: Self = Self("Megapixels");
    pub const QUALITY: Self = Self("Quality");
    pub const BITS_PER_SAMPLE: Self = Self("BitsPerSample");
    pub const COLOR_COMPONENTS: Self = Self("ColorComponents");
    pub const Y_CB_CR_SUB_SAMPLING: Self = Self("YCbCrSubSampling");
    pub const Y_CB_CR_POSITIONING: Self = Self("YCbCrPositioning");

    // === 缩略图标签 ===
    pub const THUMBNAIL_IMAGE: Self = Self("ThumbnailImage");
    pub const THUMBNAIL_LENGTH: Self = Self("ThumbnailLength");
    pub const THUMBNAIL_OFFSET: Self = Self("ThumbnailOffset");
    pub const PREVIEW_IMAGE: Self = Self("PreviewImage");
    pub const PREVIEW_IMAGE_TYPE: Self = Self("PreviewImageType");
    pub const JPG_FROM_RAW: Self = Self("JpgFromRaw");
    pub const OTHER_IMAGE: Self = Self("OtherImage");

    // === 色彩空间标签 ===
    pub const COLOR_SPACE: Self = Self("ColorSpace");
    pub const GAMMA: Self = Self("Gamma");

    // === 复合标签 (Composite) ===
    pub const HYPERFOCAL_DISTANCE: Self = Self("HyperfocalDistance");
    pub const SCALE_FACTOR_35EFL: Self = Self("ScaleFactor35efl");
    pub const CIRCLE_OF_CONFUSION: Self = Self("CircleOfConfusion");
    pub const FIELD_OF_VIEW: Self = Self("FieldOfView");
    pub const LENS_ID: Self = Self("LensID");
    pub const LENS_INFO: Self = Self("LensInfo");
    pub const LENS_SPEC: Self = Self("LensSpec");
    pub const LENS_MAKE: Self = Self("LensMake");
    pub const LENS_MODEL: Self = Self("LensModel");
    pub const LENS_SERIAL_NUMBER: Self = Self("LensSerialNumber");

    // === Canon MakerNotes ===
    pub const CANON_MODEL_ID: Self = Self("CanonModelID");
    pub const CANON_EXPOSURE_MODE: Self = Self("CanonExposureMode");
    pub const CANON_FLASH_MODE: Self = Self("CanonFlashMode");
    pub const CANON_LENS_TYPE: Self = Self("CanonLensType");
    pub const CANON_LENS_MODEL: Self = Self("CanonLensModel");
    pub const CANON_IMAGE_SIZE: Self = Self("CanonImageSize");
    pub const CANON_IMAGE_QUALITY: Self = Self("CanonImageQuality");
    pub const CANON_SHARPNESS: Self = Self("CanonSharpness");
    pub const CANON_CONTRAST: Self = Self("CanonContrast");
    pub const CANON_SATURATION: Self = Self("CanonSaturation");
    pub const CANON_COLOR_TONE: Self = Self("CanonColorTone");
    pub const CANON_COLOR_SPACE: Self = Self("CanonColorSpace");
    pub const CANON_PICTURE_STYLE: Self = Self("CanonPictureStyle");
    pub const CANON_DRIVE_MODE: Self = Self("CanonDriveMode");
    pub const CANON_FOCUS_MODE: Self = Self("CanonFocusMode");
    pub const CANON_METERING_MODE: Self = Self("CanonMeteringMode");
    pub const CANON_AF_POINT: Self = Self("CanonAFPoint");
    pub const CANON_SELF_TIMER: Self = Self("CanonSelfTimer");
    pub const CANON_IMAGE_STABILIZATION: Self = Self("CanonImageStabilization");
    pub const CANON_WHITE_BALANCE: Self = Self("CanonWhiteBalance");

    // === Nikon MakerNotes ===
    pub const NIKON_MAKE: Self = Self("NikonMake");
    pub const NIKON_QUALITY: Self = Self("NikonQuality");
    pub const NIKON_COLOR_MODE: Self = Self("NikonColorMode");
    pub const NIKON_IMAGE_ADJUSTMENT: Self = Self("NikonImageAdjustment");
    pub const NIKON_CCD_SENSITIVITY: Self = Self("NikonCCDSensitivity");
    pub const NIKON_WHITE_BALANCE_FINE: Self = Self("NikonWhiteBalanceFine");
    pub const NIKON_ISO_SETTING: Self = Self("NikonISOSetting");
    pub const NIKON_IMAGE_OPTIMIZATION: Self = Self("NikonImageOptimization");
    pub const NIKON_SATURATION_ADJUST: Self = Self("NikonSaturationAdjust");
    pub const NIKON_SHARPNESS_ADJUST: Self = Self("NikonSharpnessAdjust");
    pub const NIKON_FOCUS_MODE: Self = Self("NikonFocusMode");
    pub const NIKON_FLASH_MODE: Self = Self("NikonFlashMode");
    pub const NIKON_SHOOTING_MODE: Self = Self("NikonShootingMode");
    pub const NIKON_AUTO_BRACKET_RELEASE: Self = Self("NikonAutoBracketRelease");
    pub const NIKON_LENS_TYPE: Self = Self("NikonLensType");
    pub const NIKON_LENS: Self = Self("NikonLens");

    // === Sony MakerNotes ===
    pub const SONY_MAKE: Self = Self("SonyMake");
    pub const SONY_IMAGE_SIZE: Self = Self("SonyImageSize");
    pub const SONY_QUALITY: Self = Self("SonyQuality");
    pub const SONY_FLASH_MODE: Self = Self("SonyFlashMode");
    pub const SONY_EXPOSURE_MODE: Self = Self("SonyExposureMode");
    pub const SONY_FOCUS_MODE: Self = Self("SonyFocusMode");
    pub const SONY_WHITE_BALANCE_MODE: Self = Self("SonyWhiteBalanceMode");
    pub const SONY_MACRO: Self = Self("SonyMacro");
    pub const SONY_SHARPNESS: Self = Self("SonySharpness");
    pub const SONY_SATURATION: Self = Self("SonySaturation");
    pub const SONY_CONTRAST: Self = Self("SonyContrast");
    pub const SONY_BRIGHTNESS: Self = Self("SonyBrightness");
    pub const SONY_LONG_EXPOSURE_NOISE_REDUCTION: Self = Self("SonyLongExposureNoiseReduction");
    pub const SONY_HIGH_ISO_NOISE_REDUCTION: Self = Self("SonyHighISONoiseReduction");
    pub const SONY_HDR: Self = Self("SonyHDR");
    pub const SONY_MULTI_FRAME_NR: Self = Self("SonyMultiFrameNR");

    // === Fuji MakerNotes ===
    pub const FUJI_QUALITY: Self = Self("FujiQuality");
    pub const FUJI_SATURATION: Self = Self("FujiSaturation");
    pub const FUJI_WHITE_BALANCE_FINE_TUNE: Self = Self("FujiWhiteBalanceFineTune");
    pub const FUJI_HIGH_IS0_NOISE_REDUCTION: Self = Self("FujiHighIS0NoiseReduction");
    pub const FUJI_FOCUS_MODE: Self = Self("FujiFocusMode");
    pub const FUJI_AF_MODE: Self = Self("FujiAFMode");
    pub const FUJI_FOCUS_PIXEL: Self = Self("FujiFocusPixel");
    pub const FUJI_IMAGE_SIZE: Self = Self("FujiImageSize");
    pub const FUJI_DUAL_IMAGE_STABILIZATION: Self = Self("FujiDualImageStabilization");
    pub const FUJI_FACE_DETECTION: Self = Self("FujiFaceDetection");
    pub const FUJI_NUM_FACE_ELEMENTS: Self = Self("FujiNumFaceElements");

    // === Panasonic MakerNotes ===
    pub const PANASONIC_IMAGE_QUALITY: Self = Self("PanasonicImageQuality");
    pub const PANASONIC_COLOR_MODE: Self = Self("PanasonicColorMode");
    pub const PANASONIC_IMAGE_STABILIZATION: Self = Self("PanasonicImageStabilization");
    pub const PANASONIC_MACRO_MODE: Self = Self("PanasonicMacroMode");
    pub const PANASONIC_FOCUS_MODE: Self = Self("PanasonicFocusMode");
    pub const PANASONIC_AF_AREA_MODE: Self = Self("PanasonicAFAreaMode");
    pub const PANASONIC_IMAGE_STABILIZATION2: Self = Self("PanasonicImageStabilization2");
    pub const PANASONIC_BABY_AGE: Self = Self("PanasonicBabyAge");
    pub const PANASONIC_BABY_NAME: Self = Self("PanasonicBabyName");

    // === Olympus MakerNotes ===
    pub const OLYMPUS_IMAGE_QUALITY: Self = Self("OlympusImageQuality");
    pub const OLYMPUS_MACRO_MODE: Self = Self("OlympusMacroMode");
    pub const OLYMPUS_DIGITAL_ZOOM: Self = Self("OlympusDigitalZoom");
    pub const OLYMPUS_VERSION: Self = Self("OlympusVersion");
    pub const OLYMPUS_IMAGE_PROCESSING: Self = Self("OlympusImageProcessing");
    pub const OLYMPUS_FOCUS_MODE: Self = Self("OlympusFocusMode");
    pub const OLYMPUS_AF_AREA: Self = Self("OlympusAFArea");
    pub const OLYMPUS_AF_POINT: Self = Self("OlympusAFPoint");
    pub const OLYMPUS_IMAGE_STABILIZATION: Self = Self("OlympusImageStabilization");
    pub const OLYMPUS_COLOR_SPACE: Self = Self("OlympusColorSpace");

    // === Pentax MakerNotes ===
    pub const PENTAX_MODEL_TYPE: Self = Self("PentaxModelType");
    pub const PENTAX_IMAGE_SIZE: Self = Self("PentaxImageSize");
    pub const PENTAX_QUALITY: Self = Self("PentaxQuality");
    pub const PENTAX_IMAGE_PROCESSING: Self = Self("PentaxImageProcessing");
    pub const PENTAX_FOCUS_MODE: Self = Self("PentaxFocusMode");
    pub const PENTAX_AF_POINT: Self = Self("PentaxAFPoint");
    pub const PENTAX_AUTO_BRACKETING: Self = Self("PentaxAutoBracketing");
    pub const PENTAX_WHITE_BALANCE: Self = Self("PentaxWhiteBalance");

    // === 更多 XMP 命名空间 ===
    pub const XMP_XMP_CREATE_DATE: Self = Self("xmp:CreateDate");
    pub const XMP_XMP_MODIFY_DATE: Self = Self("xmp:ModifyDate");
    pub const XMP_XMP_METADATA_DATE: Self = Self("xmp:MetadataDate");
    pub const XMP_XMP_CREATOR_TOOL: Self = Self("xmp:CreatorTool");
    pub const XMP_XMP_RATING: Self = Self("xmp:Rating");
    pub const XMP_XMP_LABEL: Self = Self("xmp:Label");
    pub const XMP_XMP_NICKNAME: Self = Self("xmp:Nickname");

    // === XMP IPTC Core ===
    pub const XMP_IPTC_CITY: Self = Self("Iptc4xmpCore:City");
    pub const XMP_IPTC_COUNTRY: Self = Self("Iptc4xmpCore:Country");
    pub const XMP_IPTC_COUNTRY_CODE: Self = Self("Iptc4xmpCore:CountryCode");
    pub const XMP_IPTC_STATE: Self = Self("Iptc4xmpCore:State");
    pub const XMP_IPTC_LOCATION: Self = Self("Iptc4xmpCore:Location");
    pub const XMP_IPTC_SUBJECT_CODE: Self = Self("Iptc4xmpCore:SubjectCode");
    pub const XMP_IPTC_INTELLECTUAL_GENRE: Self = Self("Iptc4xmpCore:IntellectualGenre");

    // === XMP IPTC Extension ===
    pub const XMP_IPTC_EXT_DIGITAL_SOURCE_TYPE: Self = Self("Iptc4xmpExt:DigitalSourceType");
    pub const XMP_IPTC_EXT_DIGITAL_GUIDE: Self = Self("Iptc4xmpExt:DigitalGuide");
    pub const XMP_IPTC_EXT_EVENT: Self = Self("Iptc4xmpExt:Event");
    pub const XMP_IPTC_EXT_ORGANISATION_IN_IMAGE: Self = Self("Iptc4xmpExt:OrganisationInImage");
    pub const XMP_IPTC_EXT_PERSON_IN_IMAGE: Self = Self("Iptc4xmpExt:PersonInImage");
    pub const XMP_IPTC_EXT_LOCATION_SHOWN: Self = Self("Iptc4xmpExt:LocationShown");

    // === XMP Photoshop ===
    pub const XMP_PHOTOSHOP_DATE_CREATED: Self = Self("photoshop:DateCreated");
    pub const XMP_PHOTOSHOP_CITY: Self = Self("photoshop:City");
    pub const XMP_PHOTOSHOP_STATE: Self = Self("photoshop:State");
    pub const XMP_PHOTOSHOP_COUNTRY: Self = Self("photoshop:Country");
    pub const XMP_PHOTOSHOP_CREDIT: Self = Self("photoshop:Credit");
    pub const XMP_PHOTOSHOP_SOURCE: Self = Self("photoshop:Source");
    pub const XMP_PHOTOSHOP_INSTRUCTIONS: Self = Self("photoshop:Instructions");
    pub const XMP_PHOTOSHOP_TRANSMISSION_REFERENCE: Self = Self("photoshop:TransmissionReference");
    pub const XMP_PHOTOSHOP_URGENCY: Self = Self("photoshop:Urgency");
    pub const XMP_PHOTOSHOP_CATEGORY: Self = Self("photoshop:Category");
    pub const XMP_PHOTOSHOP_SUPPLEMENTAL_CATEGORIES: Self =
        Self("photoshop:SupplementalCategories");
    pub const XMP_PHOTOSHOP_HEADLINE: Self = Self("photoshop:Headline");
    pub const XMP_PHOTOSHOP_CAPTION_WRITER: Self = Self("photoshop:CaptionWriter");

    // === XMP Camera Raw ===
    pub const XMP_CRS_VERSION: Self = Self("crs:Version");
    pub const XMP_CRS_WHITE_BALANCE: Self = Self("crs:WhiteBalance");
    pub const XMP_CRS_TEMPERATURE: Self = Self("crs:Temperature");
    pub const XMP_CRS_TINT: Self = Self("crs:Tint");
    pub const XMP_CRS_EXPOSURE: Self = Self("crs:Exposure");
    pub const XMP_CRS_SHADOWS: Self = Self("crs:Shadows");
    pub const XMP_CRS_BRIGHTNESS: Self = Self("crs:Brightness");
    pub const XMP_CRS_CONTRAST: Self = Self("crs:Contrast");
    pub const XMP_CRS_SATURATION: Self = Self("crs:Saturation");
    pub const XMP_CRS_SHARPNESS: Self = Self("crs:Sharpness");
    pub const XMP_CRS_LUMINANCE_SMOOTHING: Self = Self("crs:LuminanceSmoothing");
    pub const XMP_CRS_COLOR_NOISE_REDUCTION: Self = Self("crs:ColorNoiseReduction");
    pub const XMP_CRS_VIGNETTE_AMOUNT: Self = Self("crs:VignetteAmount");

    // === XMP AUX (Camera Raw Schema) ===
    pub const XMP_AUX_SERIAL_NUMBER: Self = Self("aux:SerialNumber");
    pub const XMP_AUX_LENS_INFO: Self = Self("aux:LensInfo");
    pub const XMP_AUX_LENS: Self = Self("aux:Lens");
    pub const XMP_AUX_LENS_ID: Self = Self("aux:LensID");
    pub const XMP_AUX_LENS_SERIAL_NUMBER: Self = Self("aux:LensSerialNumber");
    pub const XMP_AUX_IMAGE_NUMBER: Self = Self("aux:ImageNumber");
    pub const XMP_AUX_FLASH_COMPENSATION: Self = Self("aux:FlashCompensation");
    pub const XMP_AUX_FIRMWARE: Self = Self("aux:Firmware");

    // === XMP MM (Media Management) ===
    pub const XMP_MM_DOCUMENT_ID: Self = Self("xmpMM:DocumentID");
    pub const XMP_MM_INSTANCE_ID: Self = Self("xmpMM:InstanceID");
    pub const XMP_MM_ORIGINAL_DOCUMENT_ID: Self = Self("xmpMM:OriginalDocumentID");
    pub const XMP_MM_RENDITION_CLASS: Self = Self("xmpMM:RenditionClass");
    pub const XMP_MM_VERSION_ID: Self = Self("xmpMM:VersionID");
    pub const XMP_MM_VERSION_MODIFIER: Self = Self("xmpMM:VersionModifier");
    pub const XMP_MM_HISTORY: Self = Self("xmpMM:History");
    pub const XMP_MM_DERIVED_FROM: Self = Self("xmpMM:DerivedFrom");

    // === XMP TIFF ===
    pub const XMP_TIFF_MAKE: Self = Self("tiff:Make");
    pub const XMP_TIFF_MODEL: Self = Self("tiff:Model");
    pub const XMP_TIFF_IMAGE_WIDTH: Self = Self("tiff:ImageWidth");
    pub const XMP_TIFF_IMAGE_HEIGHT: Self = Self("tiff:ImageHeight");
    pub const XMP_TIFF_BITS_PER_SAMPLE: Self = Self("tiff:BitsPerSample");
    pub const XMP_TIFF_COMPRESSION: Self = Self("tiff:Compression");
    pub const XMP_TIFF_PHOTOMETRIC_INTERPRETATION: Self = Self("tiff:PhotometricInterpretation");
    pub const XMP_TIFF_ORIENTATION: Self = Self("tiff:Orientation");
    pub const XMP_TIFF_SAMPLES_PER_PIXEL: Self = Self("tiff:SamplesPerPixel");
    pub const XMP_TIFF_PLANAR_CONFIGURATION: Self = Self("tiff:PlanarConfiguration");
    pub const XMP_TIFF_YCBCR_SUB_SAMPLING: Self = Self("tiff:YCbCrSubSampling");
    pub const XMP_TIFF_YCBCR_POSITIONING: Self = Self("tiff:YCbCrPositioning");
    pub const XMP_TIFF_X_RESOLUTION: Self = Self("tiff:XResolution");
    pub const XMP_TIFF_Y_RESOLUTION: Self = Self("tiff:YResolution");
    pub const XMP_TIFF_RESOLUTION_UNIT: Self = Self("tiff:ResolutionUnit");

    // === XMP EXIF ===
    pub const XMP_EXIF_EXPOSURE_TIME: Self = Self("exif:ExposureTime");
    pub const XMP_EXIF_F_NUMBER: Self = Self("exif:FNumber");
    pub const XMP_EXIF_EXPOSURE_PROGRAM: Self = Self("exif:ExposureProgram");
    pub const XMP_EXIF_SPECTRAL_SENSITIVITY: Self = Self("exif:SpectralSensitivity");
    pub const XMP_EXIF_ISO_SPEED_RATINGS: Self = Self("exif:ISOSpeedRatings");
    pub const XMP_EXIF_DATE_TIME_ORIGINAL: Self = Self("exif:DateTimeOriginal");
    pub const XMP_EXIF_DATE_TIME_DIGITIZED: Self = Self("exif:DateTimeDigitized");
    pub const XMP_EXIF_COMPONENTS_CONFIGURATION: Self = Self("exif:ComponentsConfiguration");
    pub const XMP_EXIF_COMPRESSED_BITS_PER_PIXEL: Self = Self("exif:CompressedBitsPerPixel");
    pub const XMP_EXIF_SHUTTER_SPEED_VALUE: Self = Self("exif:ShutterSpeedValue");
    pub const XMP_EXIF_APERTURE_VALUE: Self = Self("exif:ApertureValue");
    pub const XMP_EXIF_BRIGHTNESS_VALUE: Self = Self("exif:BrightnessValue");
    pub const XMP_EXIF_EXPOSURE_BIAS_VALUE: Self = Self("exif:ExposureBiasValue");
    pub const XMP_EXIF_MAX_APERTURE_VALUE: Self = Self("exif:MaxApertureValue");
    pub const XMP_EXIF_SUBJECT_DISTANCE: Self = Self("exif:SubjectDistance");
    pub const XMP_EXIF_METERING_MODE: Self = Self("exif:MeteringMode");
    pub const XMP_EXIF_LIGHT_SOURCE: Self = Self("exif:LightSource");
    pub const XMP_EXIF_FLASH: Self = Self("exif:Flash");
    pub const XMP_EXIF_FOCAL_LENGTH: Self = Self("exif:FocalLength");
    pub const XMP_EXIF_FLASH_ENERGY: Self = Self("exif:FlashEnergy");
    pub const XMP_EXIF_SPATIAL_FREQUENCY_RESPONSE: Self = Self("exif:SpatialFrequencyResponse");
    pub const XMP_EXIF_FOCAL_PLANE_X_RESOLUTION: Self = Self("exif:FocalPlaneXResolution");
    pub const XMP_EXIF_FOCAL_PLANE_Y_RESOLUTION: Self = Self("exif:FocalPlaneYResolution");
    pub const XMP_EXIF_FOCAL_PLANE_RESOLUTION_UNIT: Self = Self("exif:FocalPlaneResolutionUnit");
    pub const XMP_EXIF_SUBJECT_LOCATION: Self = Self("exif:SubjectLocation");
    pub const XMP_EXIF_EXPOSURE_INDEX: Self = Self("exif:ExposureIndex");
    pub const XMP_EXIF_SENSING_METHOD: Self = Self("exif:SensingMethod");
    pub const XMP_EXIF_FILE_SOURCE: Self = Self("exif:FileSource");
    pub const XMP_EXIF_SCENE_TYPE: Self = Self("exif:SceneType");
    pub const XMP_EXIF_CFA_PATTERN: Self = Self("exif:CFAPattern");
    pub const XMP_EXIF_CUSTOM_RENDERED: Self = Self("exif:CustomRendered");
    pub const XMP_EXIF_EXPOSURE_MODE: Self = Self("exif:ExposureMode");
    pub const XMP_EXIF_WHITE_BALANCE: Self = Self("exif:WhiteBalance");
    pub const XMP_EXIF_DIGITAL_ZOOM_RATIO: Self = Self("exif:DigitalZoomRatio");
    pub const XMP_EXIF_FOCAL_LENGTH_IN_35MM_FILM: Self = Self("exif:FocalLengthIn35mmFilm");
    pub const XMP_EXIF_SCENE_CAPTURE_TYPE: Self = Self("exif:SceneCaptureType");
    pub const XMP_EXIF_GAIN_CONTROL: Self = Self("exif:GainControl");
    pub const XMP_EXIF_CONTRAST: Self = Self("exif:Contrast");
    pub const XMP_EXIF_SATURATION: Self = Self("exif:Saturation");
    pub const XMP_EXIF_SHARPNESS: Self = Self("exif:Sharpness");
    pub const XMP_EXIF_DEVICE_SETTING_DESCRIPTION: Self = Self("exif:DeviceSettingDescription");
    pub const XMP_EXIF_SUBJECT_DISTANCE_RANGE: Self = Self("exif:SubjectDistanceRange");
    pub const XMP_EXIF_IMAGE_UNIQUE_ID: Self = Self("exif:ImageUniqueID");

    // === 视频特定标签 ===
    pub const DURATION: Self = Self("Duration");
    pub const VIDEO_FRAME_RATE: Self = Self("VideoFrameRate");
    pub const VIDEO_FRAME_COUNT: Self = Self("VideoFrameCount");
    pub const VIDEO_BIT_RATE: Self = Self("VideoBitRate");
    pub const AUDIO_BIT_RATE: Self = Self("AudioBitRate");
    pub const VIDEO_COMPRESSION: Self = Self("VideoCompression");
    pub const AUDIO_COMPRESSION: Self = Self("AudioCompression");
    pub const TRACK_NUMBER: Self = Self("TrackNumber");
    pub const TRACK_TYPE: Self = Self("TrackType");
    pub const TRACK_CREATE_DATE: Self = Self("TrackCreateDate");
    pub const TRACK_MODIFY_DATE: Self = Self("TrackModifyDate");
    pub const MEDIA_CREATE_DATE: Self = Self("MediaCreateDate");
    pub const MEDIA_MODIFY_DATE: Self = Self("MediaModifyDate");
    pub const MEDIA_DATA_SIZE: Self = Self("MediaDataSize");
    pub const MEDIA_DATA_OFFSET: Self = Self("MediaDataOffset");
    pub const GENRE: Self = Self("Genre");
    // ARTIST 已在上面定义
    pub const ALBUM: Self = Self("Album");
    pub const YEAR: Self = Self("Year");
    pub const COMMENT: Self = Self("Comment");
    pub const LYRICS: Self = Self("Lyrics");
    pub const COMPOSER: Self = Self("Composer");
    pub const PUBLISHER: Self = Self("Publisher");

    // === 更多文件信息 ===
    pub const FILE_DESCRIPTION: Self = Self("FileDescription");
    pub const FILE_VERSION: Self = Self("FileVersion");
    pub const INTERNAL_VERSION_NUMBER: Self = Self("InternalVersionNumber");
    pub const COMPANY_NAME: Self = Self("CompanyName");
    pub const LEGAL_COPYRIGHT: Self = Self("LegalCopyright");
    pub const PRODUCT_NAME: Self = Self("ProductName");
    pub const PRODUCT_VERSION: Self = Self("ProductVersion");
    pub const MIME_ENCODING: Self = Self("MIMEEncoding");

    // === 扩展标签: 更多 Canon MakerNotes (50个) ===
    pub const CANON_SERIAL_NUMBER_EXT: Self = Self("SerialNumber");
    pub const CANON_FIRMWARE_VERSION_EXT: Self = Self("FirmwareVersion");
    pub const CANON_OWNER_NAME_EXT: Self = Self("OwnerName");
    pub const CANON_TIME_ZONE_EXT: Self = Self("TimeZone");
    pub const CANON_DAYLIGHT_SAVING_EXT: Self = Self("DaylightSaving");
    pub const CANON_AF_MICRO_ADJUSTMENT: Self = Self("AFMicroadjustment");
    pub const CANON_FLASH_EXPOSURE_COMP_EXT: Self = Self("FlashExposureComp");
    pub const CANON_BRACKET_MODE_EXT: Self = Self("BracketMode");
    pub const CANON_BRACKET_VALUE_EXT: Self = Self("BracketValue");
    pub const CANON_RAW_JPG_QUALITY_EXT: Self = Self("RawJpgQuality");
    pub const CANON_RAW_JPG_SIZE_EXT: Self = Self("RawJpgSize");
    pub const CANON_NOISE_REDUCTION_EXT: Self = Self("NoiseReduction");
    pub const CANON_WB_SHIFT_GM: Self = Self("WBShiftGM");
    pub const CANON_WB_SHIFT_AB: Self = Self("WBShiftAB");
    pub const CANON_COLOR_TEMPERATURE_EXT: Self = Self("ColorTemperature");
    pub const CANON_LENS_SERIAL_NUMBER_EXT: Self = Self("LensSerialNumber");
    pub const CANON_AF_POINTS_IN_FOCUS_EXT: Self = Self("AFPointsInFocus");
    pub const CANON_AF_POINTS_SELECTED_EXT: Self = Self("AFPointsSelected");
    pub const CANON_AF_POINTS_ACTIVE_EXT: Self = Self("AFPointsActive");
    pub const CANON_FOCUS_DISTANCE_UPPER_EXT: Self = Self("FocusDistanceUpper");
    pub const CANON_FOCUS_DISTANCE_LOWER_EXT: Self = Self("FocusDistanceLower");
    pub const CANON_FLASH_BITS_EXT: Self = Self("FlashBits");
    pub const CANON_FOCUS_CONTINUOUS_EXT: Self = Self("FocusContinuous");
    pub const CANON_AE_SETTING_EXT: Self = Self("AESetting");
    pub const CANON_DISPLAY_APERTURE_EXT: Self = Self("DisplayAperture");
    pub const CANON_ZOOM_SOURCE_WIDTH_EXT: Self = Self("ZoomSourceWidth");
    pub const CANON_ZOOM_TARGET_WIDTH_EXT: Self = Self("ZoomTargetWidth");
    pub const CANON_SPOT_METERING_MODE_EXT: Self = Self("SpotMeteringMode");
    pub const CANON_PHOTO_EFFECT_EXT: Self = Self("PhotoEffect");
    pub const CANON_MANUAL_FLASH_OUTPUT_EXT: Self = Self("ManualFlashOutput");
    pub const CANON_SRAW_QUALITY_EXT: Self = Self("SRAWQuality");
    pub const CANON_GAMMA_EXT: Self = Self("Gamma");
    pub const CANON_HIGH_SPEED_SYNC: Self = Self("HighSpeedSync");
    pub const CANON_AF_POINTS_INFO: Self = Self("AFPointsInfo");
    pub const CANON_MEASURE_ROLL: Self = Self("MeasureRoll");
    pub const CANON_MEASURE_PITCH: Self = Self("MeasurePitch");
    pub const CANON_MEASURE_YAW: Self = Self("MeasureYaw");
    pub const CANON_MEASURE_ACCEL_X: Self = Self("MeasureAccelX");
    pub const CANON_MEASURE_ACCEL_Y: Self = Self("MeasureAccelY");
    pub const CANON_MEASURE_ACCEL_Z: Self = Self("MeasureAccelZ");
    pub const CANON_DATE_STAMP_MODE: Self = Self("DateStampMode");
    pub const CANON_MY_COLORS_MODE_EXT: Self = Self("MyColorsMode");
    pub const CANON_FIRMWARE_REVISION: Self = Self("FirmwareRevision");
    pub const CANON_IMAGE_UNIQUE_ID_EXT: Self = Self("ImageUniqueID");
    pub const CANON_HDR_SETTING_EXT: Self = Self("HDRSetting");
    pub const CANON_MULTIPLE_EXPOSURE_EXT: Self = Self("MultipleExposure");
    pub const CANON_FILTER_EFFECT_EXT: Self = Self("FilterEffect");
    pub const CANON_TONING_EFFECT_EXT: Self = Self("ToningEffect");

    // === 扩展标签: 更多 Nikon MakerNotes (50个) ===
    pub const NIKON_WHITE_BALANCE_FINE_TUNE_EXT: Self = Self("WhiteBalanceFineTune");
    pub const NIKON_COLOR_SPACE_EXT: Self = Self("ColorSpace");
    pub const NIKON_VIGNETTE_CONTROL_EXT: Self = Self("VignetteControl");
    pub const NIKON_AUTO_DISTORTION_CONTROL_EXT: Self = Self("AutoDistortionControl");
    pub const NIKON_PICTURE_CONTROL_EXT: Self = Self("PictureControl");
    pub const NIKON_HIGH_ISO_NR_EXT: Self = Self("HighISONoiseReduction");
    pub const NIKON_LONG_EXPOSURE_NR_EXT: Self = Self("LongExposureNoiseReduction");
    pub const NIKON_ACTIVE_D_LIGHTING_EXT: Self = Self("ActiveDLighting");
    pub const NIKON_MULTIPLE_EXPOSURE_MODE_EXT: Self = Self("MultipleExposureMode");
    pub const NIKON_MULTI_EXPOSURE_SHOTS_EXT: Self = Self("MultiExposureShots");
    pub const NIKON_HDR_EXT: Self = Self("HDR");
    pub const NIKON_VR_MODE_EXT: Self = Self("VRMode");
    pub const NIKON_VR_INFO_EXT: Self = Self("VRInfo");
    pub const NIKON_FIRMWARE_VERSION_EXT: Self = Self("NikonFirmwareVersion");
    pub const NIKON_AF_POINTS_USED_EXT: Self = Self("AFPointsUsed");
    pub const NIKON_AF_POINTS_IN_FOCUS_EXT: Self = Self("AFPointsInFocus");
    pub const NIKON_AF_POINTS_SELECTED_EXT: Self = Self("AFPointsSelected");
    pub const NIKON_SCENE_MODE_EXT: Self = Self("SceneMode");
    pub const NIKON_LIGHTING_TYPE_EXT: Self = Self("LightingType");
    pub const NIKON_SHUTTER_COUNT_EXT: Self = Self("ShutterCount");
    pub const NIKON_ELECTRONIC_SHUTTER_COUNT_EXT: Self = Self("ElectronicShutterCount");
    pub const NIKON_NEF_BIT_DEPTH_EXT: Self = Self("NEFBitDepth");
    pub const NIKON_OPTIMIZATION_EXT: Self = Self("Optimization");
    pub const NIKON_SATURATION_EXT: Self = Self("NikonSaturation");
    pub const NIKON_AF_AREA_X_POSITION_EXT: Self = Self("AFAreaXPosition");
    pub const NIKON_AF_AREA_Y_POSITION_EXT: Self = Self("AFAreaYPosition");
    pub const NIKON_PHASE_DETECT_AF_EXT: Self = Self("PhaseDetectAF");
    pub const NIKON_PRIMARY_AF_POINT_EXT: Self = Self("PrimaryAFPoint");
    pub const NIKON_CONTRAST_DETECT_AF_EXT: Self = Self("ContrastDetectAF");
    pub const NIKON_AF_AREA_POINTS_EXT: Self = Self("AFAreaPoints");
    pub const NIKON_SERIAL_NUMBER_2_EXT: Self = Self("NikonSerialNumber2");
    pub const NIKON_SHUTTER_COUNT_2_EXT: Self = Self("NikonShutterCount2");
    pub const NIKON_FLASH_MODE_2_EXT: Self = Self("NikonFlashMode2");
    pub const NIKON_FLASH_CONTROL_MODE_EXT: Self = Self("FlashControlMode");
    pub const NIKON_FLASH_EXPOSURE_COMP_2_EXT: Self = Self("NikonFlashExposureComp2");
    pub const NIKON_FLASH_EXPOSURE_BRACKET_VALUE_EXT: Self = Self("FlashExposureBracketValue");
    pub const NIKON_EXTERNAL_FLASH_BOUNCE_EXT: Self = Self("ExternalFlashBounce");
    pub const NIKON_EXTERNAL_FLASH_ZOOM_EXT: Self = Self("ExternalFlashZoom");
    pub const NIKON_EXTERNAL_FLASH_MODE_EXT: Self = Self("ExternalFlashMode");
    pub const NIKON_EXTERNAL_FLASH_COMPENSATION_EXT: Self = Self("ExternalFlashCompensation");
    pub const NIKON_COMMANDER_CHANNEL_EXT: Self = Self("CommanderChannel");
    pub const NIKON_COMMANDER_GROUP_A_MODE_EXT: Self = Self("CommanderGroupAMode");
    pub const NIKON_COMMANDER_GROUP_A_COMPENSATION_EXT: Self = Self("CommanderGroupACompensation");

    // === 扩展标签: 更多 Sony MakerNotes (30个) ===
    pub const SONY_LENS_SPEC_EXT: Self = Self("SonyLensSpec");
    pub const SONY_COLOR_TEMPERATURE_EXT: Self = Self("SonyColorTemperature");
    pub const SONY_COLOR_COMPENSATION_FILTER_EXT: Self = Self("SonyColorCompensationFilter");
    pub const SONY_WHITE_BALANCE_FINE_TUNE_EXT: Self = Self("SonyWhiteBalanceFineTune");
    pub const SONY_IMAGE_STABILIZATION_STATE_EXT: Self = Self("SonyImageStabilizationState");
    pub const SONY_DYNAMIC_RANGE_OPTIMIZER_EXT: Self = Self("DynamicRangeOptimizer");
    pub const SONY_INTELLIGENT_AUTO_EXT: Self = Self("IntelligentAuto");
    pub const SONY_FLASH_LEVEL_EXT: Self = Self("FlashLevel");
    pub const SONY_RELEASE_MODE_EXT: Self = Self("ReleaseMode");
    pub const SONY_SEQUENCE_NUMBER_EXT: Self = Self("SequenceNumber");
    pub const SONY_FOCUS_STATUS_EXT: Self = Self("FocusStatus");
    pub const SONY_AF_AIDED_EXT: Self = Self("AFAided");
    pub const SONY_AF_AREA_MODE_EXT: Self = Self("AFAreaMode");
    pub const SONY_AF_POINT_SELECTED_EXT: Self = Self("AFPointSelected");
    pub const SONY_AF_STATUS_EXT: Self = Self("AFStatus");
    pub const SONY_LENS_MOUNT_EXT: Self = Self("SonyLensMount");
    pub const SONY_LENS_FORMAT_EXT: Self = Self("SonyLensFormat");
    pub const SONY_DISTORTION_CORRECTION_EXT: Self = Self("DistortionCorrection");
    pub const SONY_CHROMATIC_ABERRATION_CORRECTION_EXT: Self =
        Self("ChromaticAberrationCorrection");
    pub const SONY_VIGNETTING_CORRECTION_EXT: Self = Self("VignettingCorrection");
    pub const SONY_SHADING_COMPENSATION_EXT: Self = Self("ShadingCompensation");
    pub const SONY_HDR_SETTING_EXT: Self = Self("SonyHDRSetting");
    pub const SONY_HDR_ALIGNMENT_EXT: Self = Self("HDRAlignment");
    pub const SONY_PANORAMA_DIRECTION_EXT: Self = Self("PanoramaDirection");
    pub const SONY_PANORAMA_ANGLE_EXT: Self = Self("PanoramaAngle");
    pub const SONY_MULTI_FRAME_NR_EXT: Self = Self("MultiFrameNoiseReduction");
    pub const SONY_PICTURE_EFFECT_EXT: Self = Self("PictureEffect");
    pub const SONY_SOFT_SKIN_EFFECT_EXT: Self = Self("SoftSkinEffect");
    pub const SONY_AUTO_PORTRAIT_FRAMED_EXT: Self = Self("AutoPortraitFramed");
    pub const SONY_AF_ILLUMINATOR_EXT: Self = Self("AFIlluminator");

    // === 扩展标签: 视频元数据 (30个) ===
    pub const VIDEO_TRACK_CREATE_DATE_EXT: Self = Self("TrackCreateDate");
    pub const VIDEO_TRACK_MODIFY_DATE_EXT: Self = Self("TrackModifyDate");
    pub const VIDEO_MEDIA_CREATE_DATE_EXT: Self = Self("MediaCreateDate");
    pub const VIDEO_MEDIA_MODIFY_DATE_EXT: Self = Self("MediaModifyDate");
    pub const VIDEO_HANDLER_TYPE_EXT: Self = Self("HandlerType");
    pub const VIDEO_HANDLER_DESCRIPTION_EXT: Self = Self("HandlerDescription");
    pub const VIDEO_COMPRESSOR_ID_EXT: Self = Self("CompressorID");
    pub const VIDEO_BITS_PER_COMPONENT_EXT: Self = Self("BitsPerComponent");
    pub const VIDEO_COLOR_PROFILE_EXT: Self = Self("VideoColorProfile");
    pub const VIDEO_AUDIO_FORMAT_EXT: Self = Self("AudioFormat");
    pub const VIDEO_AUDIO_CHANNELS_EXT: Self = Self("AudioChannels");
    pub const VIDEO_AUDIO_BITS_PER_SAMPLE_EXT: Self = Self("AudioBitsPerSample");
    pub const VIDEO_AUDIO_SAMPLE_RATE_EXT: Self = Self("AudioSampleRate");
    pub const VIDEO_DURATION_EXT: Self = Self("Duration");
    pub const VIDEO_MOVIE_HEADER_VERSION_EXT: Self = Self("MovieHeaderVersion");
    pub const VIDEO_TIME_SCALE_EXT: Self = Self("TimeScale");
    pub const VIDEO_PREFERRED_RATE_EXT: Self = Self("PreferredRate");
    pub const VIDEO_PREFERRED_VOLUME_EXT: Self = Self("PreferredVolume");
    pub const VIDEO_PREVIEW_TIME_EXT: Self = Self("PreviewTime");
    pub const VIDEO_PREVIEW_DURATION_EXT: Self = Self("PreviewDuration");
    pub const VIDEO_POSTER_TIME_EXT: Self = Self("PosterTime");
    pub const VIDEO_SELECTION_TIME_EXT: Self = Self("SelectionTime");
    pub const VIDEO_SELECTION_DURATION_EXT: Self = Self("SelectionDuration");
    pub const VIDEO_CURRENT_TIME_EXT: Self = Self("CurrentTime");
    pub const VIDEO_NEXT_TRACK_ID_EXT: Self = Self("NextTrackID");
    pub const VIDEO_TRACK_ID_EXT: Self = Self("VideoTrackID");
    pub const VIDEO_TRACK_LAYER_EXT: Self = Self("VideoTrackLayer");
    pub const VIDEO_TRACK_VOLUME_EXT: Self = Self("VideoTrackVolume");
    pub const VIDEO_TRACK_DURATION_EXT: Self = Self("VideoTrackDuration");
    pub const VIDEO_WIDTH_EXT: Self = Self("VideoWidth");

    // === 扩展标签: RAW/DNG (20个) ===
    pub const DNG_VERSION_EXT: Self = Self("DNGVersion");
    pub const DNG_BACKWARD_VERSION_EXT: Self = Self("DNGBackwardVersion");
    pub const UNIQUE_CAMERA_MODEL_EXT: Self = Self("UniqueCameraModel");
    pub const LOCALIZED_CAMERA_MODEL_EXT: Self = Self("LocalizedCameraModel");
    pub const CFA_PLANE_COLOR_EXT: Self = Self("CFAPlaneColor");
    pub const CFA_LAYOUT_EXT: Self = Self("CFALayout");
    pub const LINEARIZATION_TABLE_EXT: Self = Self("LinearizationTable");
    pub const BLACK_LEVEL_EXT: Self = Self("BlackLevel");
    pub const WHITE_LEVEL_EXT: Self = Self("WhiteLevel");
    pub const DEFAULT_SCALE_EXT: Self = Self("DefaultScale");
    pub const BEST_QUALITY_SCALE_EXT: Self = Self("BestQualityScale");
    pub const DEFAULT_CROP_ORIGIN_EXT: Self = Self("DefaultCropOrigin");
    pub const DEFAULT_CROP_SIZE_EXT: Self = Self("DefaultCropSize");
    pub const CALIBRATION_ILLUMINANT_1_EXT: Self = Self("CalibrationIlluminant1");
    pub const CALIBRATION_ILLUMINANT_2_EXT: Self = Self("CalibrationIlluminant2");
    pub const COLOR_MATRIX_1_EXT: Self = Self("ColorMatrix1");
    pub const COLOR_MATRIX_2_EXT: Self = Self("ColorMatrix2");
    pub const CAMERA_CALIBRATION_1_EXT: Self = Self("CameraCalibration1");
    pub const CAMERA_CALIBRATION_2_EXT: Self = Self("CameraCalibration2");
    pub const ANALOG_BALANCE_EXT: Self = Self("AnalogBalance");
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for TagId {
    fn from(name: &'static str) -> Self {
        Self(name)
    }
}

impl AsRef<str> for TagId {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// 标签值类型 - 支持 ExifTool 返回的所有数据类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagValue {
    /// 字符串值
    String(String),

    /// 整数值
    Integer(i64),

    /// 浮点数值
    Float(f64),

    /// 布尔值
    Boolean(bool),

    /// 数组值
    Array(Vec<TagValue>),

    /// 二进制数据（Base64 编码）
    Binary(String),

    /// 空值
    Null,
}

impl TagValue {
    /// 尝试获取字符串值
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// 尝试获取整数值
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            Self::Float(f) => Some(*f as i64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// 尝试获取浮点数值
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Integer(i) => Some(*i as f64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// 尝试获取布尔值
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            Self::Integer(0) => Some(false),
            Self::Integer(_) => Some(true),
            Self::String(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" | "on" => Some(true),
                "false" | "no" | "0" | "off" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }

    /// 尝试获取数组
    pub fn as_array(&self) -> Option<&Vec<TagValue>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// 转换为字符串表示
    pub fn to_string_lossy(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Integer(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_lossy()).collect();
                format!("[{}]", items.join(", "))
            }
            Self::Binary(b) => format!("[binary: {} bytes]", b.len()),
            Self::Null => "null".to_string(),
        }
    }

    /// 检查是否为空
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl fmt::Display for TagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

/// 元数据结构
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// 顶层标签
    #[serde(flatten)]
    tags: HashMap<String, TagValue>,

    /// 分组标签（如 EXIF、IPTC、XMP 等）
    #[serde(skip)]
    groups: HashMap<String, Metadata>,
}

impl Metadata {
    /// 创建空的元数据
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取标签值
    pub fn get(&self, tag: &str) -> Option<&TagValue> {
        self.tags.get(tag)
    }

    /// 获取标签值（使用 TagId）
    pub fn get_tag(&self, tag: TagId) -> Option<&TagValue> {
        self.get(tag.name())
    }

    /// 设置标签值
    pub fn set(&mut self, tag: impl Into<String>, value: impl Into<TagValue>) {
        self.tags.insert(tag.into(), value.into());
    }

    /// 设置标签值（使用 TagId）
    pub fn set_tag(&mut self, tag: TagId, value: impl Into<TagValue>) {
        self.set(tag.name(), value);
    }

    /// 获取所有标签
    pub fn tags(&self) -> &HashMap<String, TagValue> {
        &self.tags
    }

    /// 获取所有标签（可变）
    pub fn tags_mut(&mut self) -> &mut HashMap<String, TagValue> {
        &mut self.tags
    }

    /// 获取分组
    pub fn group(&self, name: &str) -> Option<&Metadata> {
        self.groups.get(name)
    }

    /// 设置分组
    pub fn set_group(&mut self, name: impl Into<String>, metadata: Metadata) {
        self.groups.insert(name.into(), metadata);
    }

    /// 获取所有分组
    pub fn groups(&self) -> &HashMap<String, Metadata> {
        &self.groups
    }

    /// 检查是否包含标签
    pub fn contains(&self, tag: &str) -> bool {
        self.tags.contains_key(tag)
    }

    /// 检查是否包含标签（使用 TagId）
    pub fn contains_tag(&self, tag: TagId) -> bool {
        self.contains(tag.name())
    }

    /// 获取标签数量
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// 合并另一个元数据
    pub fn merge(&mut self, other: Metadata) {
        self.tags.extend(other.tags);
        self.groups.extend(other.groups);
    }

    /// 遍历所有标签
    pub fn iter(&self) -> impl Iterator<Item = (&String, &TagValue)> {
        self.tags.iter()
    }
}

impl IntoIterator for Metadata {
    type Item = (String, TagValue);
    type IntoIter = std::collections::hash_map::IntoIter<String, TagValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags.into_iter()
    }
}

impl<'a> IntoIterator for &'a Metadata {
    type Item = (&'a String, &'a TagValue);
    type IntoIter = std::collections::hash_map::Iter<'a, String, TagValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags.iter()
    }
}

// 类型转换实现
impl From<String> for TagValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for TagValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<i64> for TagValue {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<i32> for TagValue {
    fn from(i: i32) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<f64> for TagValue {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<f32> for TagValue {
    fn from(f: f32) -> Self {
        Self::Float(f as f64)
    }
}

impl From<bool> for TagValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<Vec<TagValue>> for TagValue {
    fn from(arr: Vec<TagValue>) -> Self {
        Self::Array(arr)
    }
}

impl<T: Into<TagValue>> From<Option<T>> for TagValue {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Self::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_id() {
        assert_eq!(TagId::MAKE.name(), "Make");
        assert_eq!(TagId::MODEL.name(), "Model");
    }

    #[test]
    fn test_tag_value_conversions() {
        let str_val: TagValue = "test".into();
        assert_eq!(str_val.as_string(), Some(&"test".to_string()));

        let int_val: TagValue = 42i64.into();
        assert_eq!(int_val.as_integer(), Some(42));

        let float_val: TagValue = std::f64::consts::PI.into();
        assert_eq!(float_val.as_float(), Some(std::f64::consts::PI));

        let bool_val: TagValue = true.into();
        assert_eq!(bool_val.as_bool(), Some(true));
    }

    #[test]
    fn test_metadata() {
        let mut meta = Metadata::new();
        meta.set("Make", "Canon");
        meta.set("Model", "EOS 5D");

        assert_eq!(meta.len(), 2);
        assert!(meta.contains("Make"));
        assert_eq!(
            meta.get("Make"),
            Some(&TagValue::String("Canon".to_string()))
        );
    }

    #[test]
    fn test_metadata_iteration() {
        let mut meta = Metadata::new();
        meta.set("A", 1);
        meta.set("B", 2);

        let mut count = 0;
        for (key, _value) in &meta {
            count += 1;
            assert!(key == "A" || key == "B");
        }
        assert_eq!(count, 2);
    }
}
