//! 核心类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// 标签标识符 - 提供类型安全的标签访问
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(pub &'static str);

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
    pub const Make: Self = Self("Make");
    pub const Model: Self = Self("Model");
    pub const DateTimeOriginal: Self = Self("DateTimeOriginal");
    pub const CreateDate: Self = Self("CreateDate");
    pub const ModifyDate: Self = Self("ModifyDate");
    pub const ImageWidth: Self = Self("ImageWidth");
    pub const ImageHeight: Self = Self("ImageHeight");
    pub const Orientation: Self = Self("Orientation");
    pub const XResolution: Self = Self("XResolution");
    pub const YResolution: Self = Self("YResolution");
    pub const ResolutionUnit: Self = Self("ResolutionUnit");
    pub const Software: Self = Self("Software");
    pub const Copyright: Self = Self("Copyright");
    pub const Artist: Self = Self("Artist");
    pub const ImageDescription: Self = Self("ImageDescription");

    // === 相机设置标签 ===
    pub const ExposureTime: Self = Self("ExposureTime");
    pub const FNumber: Self = Self("FNumber");
    pub const ExposureProgram: Self = Self("ExposureProgram");
    pub const Iso: Self = Self("ISO");
    pub const SensitivityType: Self = Self("SensitivityType");
    pub const RecommendedExposureIndex: Self = Self("RecommendedExposureIndex");
    pub const ExifVersion: Self = Self("ExifVersion");
    pub const DateTimeDigitized: Self = Self("DateTimeDigitized");
    pub const ComponentConfiguration: Self = Self("ComponentConfiguration");
    pub const ShutterSpeedValue: Self = Self("ShutterSpeedValue");
    pub const ApertureValue: Self = Self("ApertureValue");
    pub const BrightnessValue: Self = Self("BrightnessValue");
    pub const ExposureCompensation: Self = Self("ExposureCompensation");
    pub const MaxApertureValue: Self = Self("MaxApertureValue");
    pub const SubjectDistance: Self = Self("SubjectDistance");
    pub const MeteringMode: Self = Self("MeteringMode");
    pub const LightSource: Self = Self("LightSource");
    pub const FLASH: Self = Self("Flash");
    pub const FocalLength: Self = Self("FocalLength");
    pub const FocalLengthIn35mmFormat: Self = Self("FocalLengthIn35mmFormat");
    pub const FlashEnergy: Self = Self("FlashEnergy");
    pub const SpatialFrequencyResponse: Self = Self("SpatialFrequencyResponse");
    pub const FocalPlaneXResolution: Self = Self("FocalPlaneXResolution");
    pub const FocalPlaneYResolution: Self = Self("FocalPlaneYResolution");
    pub const FocalPlaneResolutionUnit: Self = Self("FocalPlaneResolutionUnit");
    pub const SubjectLocation: Self = Self("SubjectLocation");
    pub const ExposureIndex: Self = Self("ExposureIndex");
    pub const SensingMethod: Self = Self("SensingMethod");
    pub const FileSource: Self = Self("FileSource");
    pub const SceneType: Self = Self("SceneType");
    pub const CfaPattern: Self = Self("CFAPattern");
    pub const CustomRendered: Self = Self("CustomRendered");
    pub const ExposureMode: Self = Self("ExposureMode");
    pub const WhiteBalance: Self = Self("WhiteBalance");
    pub const DigitalZoomRatio: Self = Self("DigitalZoomRatio");
    pub const FocalLength35efl: Self = Self("FocalLength35efl");
    pub const SceneCaptureType: Self = Self("SceneCaptureType");
    pub const GainControl: Self = Self("GainControl");
    pub const CONTRAST: Self = Self("Contrast");
    pub const SATURATION: Self = Self("Saturation");
    pub const SHARPNESS: Self = Self("Sharpness");
    pub const DeviceSettingDescription: Self = Self("DeviceSettingDescription");
    pub const SubjectDistanceRange: Self = Self("SubjectDistanceRange");

    // === GPS 标签 ===
    pub const GpsLatitudeRef: Self = Self("GPSLatitudeRef");
    pub const GpsLatitude: Self = Self("GPSLatitude");
    pub const GpsLongitudeRef: Self = Self("GPSLongitudeRef");
    pub const GpsLongitude: Self = Self("GPSLongitude");
    pub const GpsAltitudeRef: Self = Self("GPSAltitudeRef");
    pub const GpsAltitude: Self = Self("GPSAltitude");
    pub const GpsTimestamp: Self = Self("GPSTimeStamp");
    pub const GpsSatellites: Self = Self("GPSSatellites");
    pub const GpsStatus: Self = Self("GPSStatus");
    pub const GpsMeasureMode: Self = Self("GPSMeasureMode");
    pub const GpsDop: Self = Self("GPSDOP");
    pub const GpsSpeedRef: Self = Self("GPSSpeedRef");
    pub const GpsSpeed: Self = Self("GPSSpeed");
    pub const GpsTrackRef: Self = Self("GPSTrackRef");
    pub const GpsTrack: Self = Self("GPSTrack");
    pub const GpsImgDirectionRef: Self = Self("GPSImgDirectionRef");
    pub const GpsImgDirection: Self = Self("GPSImgDirection");
    pub const GpsMapDatum: Self = Self("GPSMapDatum");
    pub const GpsDestLatitudeRef: Self = Self("GPSDestLatitudeRef");
    pub const GpsDestLatitude: Self = Self("GPSDestLatitude");
    pub const GpsDestLongitudeRef: Self = Self("GPSDestLongitudeRef");
    pub const GpsDestLongitude: Self = Self("GPSDestLongitude");
    pub const GpsDestBearingRef: Self = Self("GPSDestBearingRef");
    pub const GpsDestBearing: Self = Self("GPSDestBearing");
    pub const GpsDestDistanceRef: Self = Self("GPSDestDistanceRef");
    pub const GpsDestDistance: Self = Self("GPSDestDistance");
    pub const GpsProcessingMethod: Self = Self("GPSProcessingMethod");
    pub const GpsAreaInformation: Self = Self("GPSAreaInformation");
    pub const GpsDateStamp: Self = Self("GPSDateStamp");
    pub const GpsDifferential: Self = Self("GPSDifferential");
    pub const GpsHPositioningError: Self = Self("GPSHPositioningError");

    // === 文件信息标签 ===
    pub const FileName: Self = Self("FileName");
    pub const DIRECTORY: Self = Self("Directory");
    pub const FileSize: Self = Self("FileSize");
    pub const FileModifyDate: Self = Self("FileModifyDate");
    pub const FileAccessDate: Self = Self("FileAccessDate");
    pub const FileInodeChangeDate: Self = Self("FileInodeChangeDate");
    pub const FilePermissions: Self = Self("FilePermissions");
    pub const FileType: Self = Self("FileType");
    pub const FileTypeExtension: Self = Self("FileTypeExtension");
    pub const MimeType: Self = Self("MIMEType");
    pub const ExifByteOrder: Self = Self("ExifByteOrder");
    pub const CurrentIccProfile: Self = Self("CurrentICCProfile");
    pub const ProfileDateTime: Self = Self("ProfileDateTime");
    pub const ProfileFileSignature: Self = Self("ProfileFileSignature");
    pub const PrimaryPlatform: Self = Self("PrimaryPlatform");
    pub const CmmType: Self = Self("CMMType");
    pub const ProfileVersion: Self = Self("ProfileVersion");
    pub const ProfileClass: Self = Self("ProfileClass");
    pub const ColorSpaceData: Self = Self("ColorSpaceData");
    pub const ProfileConnectionSpace: Self = Self("ProfileConnectionSpace");
    pub const ProfileConnectionSpaceIlluminant: Self = Self("ProfileConnectionSpaceIlluminant");
    pub const IccProfileCreator: Self = Self("ICCProfileCreator");
    pub const IccProfileDescription: Self = Self("ICCProfileDescription");
    pub const IccViewingConditionsDescription: Self = Self("ICCViewingConditionsDescription");
    pub const IccDeviceModel: Self = Self("ICCDeviceModel");
    pub const IccDeviceManufacturer: Self = Self("ICCDeviceManufacturer");

    // === IPTC 标签 ===
    pub const IptcObjectName: Self = Self("ObjectName");
    pub const IptcEditStatus: Self = Self("EditStatus");
    pub const IptcEditorialUpdate: Self = Self("EditorialUpdate");
    pub const IptcUrgency: Self = Self("Urgency");
    pub const IptcSubjectReference: Self = Self("SubjectReference");
    pub const IptcCategory: Self = Self("Category");
    pub const IptcSupplementalCategory: Self = Self("SupplementalCategory");
    pub const IptcFixtureIdentifier: Self = Self("FixtureIdentifier");
    pub const IptcKeywords: Self = Self("Keywords");
    pub const IptcContentLocationCode: Self = Self("ContentLocationCode");
    pub const IptcContentLocationName: Self = Self("ContentLocationName");
    pub const IptcReleaseDate: Self = Self("ReleaseDate");
    pub const IptcReleaseTime: Self = Self("ReleaseTime");
    pub const IptcExpirationDate: Self = Self("ExpirationDate");
    pub const IptcExpirationTime: Self = Self("ExpirationTime");
    pub const IptcSpecialInstructions: Self = Self("SpecialInstructions");
    pub const IptcActionAdvised: Self = Self("ActionAdvised");
    pub const IptcReferenceService: Self = Self("ReferenceService");
    pub const IptcReferenceDate: Self = Self("ReferenceDate");
    pub const IptcReferenceNumber: Self = Self("ReferenceNumber");
    pub const IptcDateCreated: Self = Self("DateCreated");
    pub const IptcTimeCreated: Self = Self("TimeCreated");
    pub const IptcDigitalCreationDate: Self = Self("DigitalCreationDate");
    pub const IptcDigitalCreationTime: Self = Self("DigitalCreationTime");
    pub const IptcOriginatingProgram: Self = Self("OriginatingProgram");
    pub const IptcProgramVersion: Self = Self("ProgramVersion");
    pub const IptcObjectCycle: Self = Self("ObjectCycle");
    pub const IptcByLine: Self = Self("By-line");
    pub const IptcByLineTitle: Self = Self("By-lineTitle");
    pub const IptcCity: Self = Self("City");
    pub const IptcSubLocation: Self = Self("Sub-location");
    pub const IptcProvinceState: Self = Self("Province-State");
    pub const IptcCountryPrimaryLocationCode: Self = Self("Country-PrimaryLocationCode");
    pub const IptcCountryPrimaryLocationName: Self = Self("Country-PrimaryLocationName");
    pub const IptcOriginalTransmissionReference: Self = Self("OriginalTransmissionReference");
    pub const IptcHeadline: Self = Self("Headline");
    pub const IptcCredit: Self = Self("Credit");
    pub const IptcSource: Self = Self("Source");
    pub const IptcCopyrightNotice: Self = Self("CopyrightNotice");
    pub const IptcContact: Self = Self("Contact");
    pub const IptcCaptionAbstract: Self = Self("Caption-Abstract");
    pub const IptcWriterEditor: Self = Self("Writer-Editor");
    pub const IptcImageType: Self = Self("ImageType");
    pub const IptcImageOrientation: Self = Self("ImageOrientation");
    pub const IptcLanguageIdentifier: Self = Self("LanguageIdentifier");

    // === XMP 标签 ( Dublin Core ) ===
    pub const XmpDcTitle: Self = Self("Title");
    pub const XmpDcCreator: Self = Self("Creator");
    pub const XmpDcSubject: Self = Self("Subject");
    pub const XmpDcDescription: Self = Self("Description");
    pub const XmpDcPublisher: Self = Self("Publisher");
    pub const XmpDcContributor: Self = Self("Contributor");
    pub const XmpDcDate: Self = Self("Date");
    pub const XmpDcType: Self = Self("Type");
    pub const XmpDcFormat: Self = Self("Format");
    pub const XmpDcIdentifier: Self = Self("Identifier");
    pub const XmpDcSource: Self = Self("Source");
    pub const XmpDcLanguage: Self = Self("Language");
    pub const XmpDcRelation: Self = Self("Relation");
    pub const XmpDcCoverage: Self = Self("Coverage");
    pub const XmpDcRights: Self = Self("Rights");

    // === XMP 标签 ( XMP Rights ) ===
    pub const XmpXmpRightsManaged: Self = Self("RightsManaged");
    pub const XmpXmpRightsMarked: Self = Self("RightsMarked");
    pub const XmpXmpRightsWebStatement: Self = Self("WebStatement");
    pub const XmpXmpRightsUsageTerms: Self = Self("UsageTerms");

    // === 图像尺寸标签 ===
    pub const ImageSize: Self = Self("ImageSize");
    pub const MEGAPIXELS: Self = Self("Megapixels");
    pub const QUALITY: Self = Self("Quality");
    pub const BitsPerSample: Self = Self("BitsPerSample");
    pub const ColorComponents: Self = Self("ColorComponents");
    pub const YCbCrSubSampling: Self = Self("YCbCrSubSampling");
    pub const YCbCrPositioning: Self = Self("YCbCrPositioning");

    // === 缩略图标签 ===
    pub const ThumbnailImage: Self = Self("ThumbnailImage");
    pub const ThumbnailLength: Self = Self("ThumbnailLength");
    pub const ThumbnailOffset: Self = Self("ThumbnailOffset");
    pub const PreviewImage: Self = Self("PreviewImage");
    pub const PreviewImageType: Self = Self("PreviewImageType");
    pub const JpgFromRaw: Self = Self("JpgFromRaw");
    pub const OtherImage: Self = Self("OtherImage");

    // === 色彩空间标签 ===
    pub const ColorSpace: Self = Self("ColorSpace");
    pub const GAMMA: Self = Self("Gamma");

    // === 复合标签 (Composite) ===
    pub const HyperfocalDistance: Self = Self("HyperfocalDistance");
    pub const ScaleFactor35efl: Self = Self("ScaleFactor35efl");
    pub const CircleOfConfusion: Self = Self("CircleOfConfusion");
    pub const FieldOfView: Self = Self("FieldOfView");
    pub const LensId: Self = Self("LensID");
    pub const LensInfo: Self = Self("LensInfo");
    pub const LensSpec: Self = Self("LensSpec");
    pub const LensMake: Self = Self("LensMake");
    pub const LensModel: Self = Self("LensModel");
    pub const LensSerialNumber: Self = Self("LensSerialNumber");

    // === Canon MakerNotes ===
    pub const CanonModelId: Self = Self("CanonModelID");
    pub const CanonExposureMode: Self = Self("CanonExposureMode");
    pub const CanonFlashMode: Self = Self("CanonFlashMode");
    pub const CanonLensType: Self = Self("CanonLensType");
    pub const CanonLensModel: Self = Self("CanonLensModel");
    pub const CanonImageSize: Self = Self("CanonImageSize");
    pub const CanonImageQuality: Self = Self("CanonImageQuality");
    pub const CanonSharpness: Self = Self("CanonSharpness");
    pub const CanonContrast: Self = Self("CanonContrast");
    pub const CanonSaturation: Self = Self("CanonSaturation");
    pub const CanonColorTone: Self = Self("CanonColorTone");
    pub const CanonColorSpace: Self = Self("CanonColorSpace");
    pub const CanonPictureStyle: Self = Self("CanonPictureStyle");
    pub const CanonDriveMode: Self = Self("CanonDriveMode");
    pub const CanonFocusMode: Self = Self("CanonFocusMode");
    pub const CanonMeteringMode: Self = Self("CanonMeteringMode");
    pub const CanonAfPoint: Self = Self("CanonAFPoint");
    pub const CanonSelfTimer: Self = Self("CanonSelfTimer");
    pub const CanonImageStabilization: Self = Self("CanonImageStabilization");
    pub const CanonWhiteBalance: Self = Self("CanonWhiteBalance");

    // === Nikon MakerNotes ===
    pub const NikonMake: Self = Self("NikonMake");
    pub const NikonQuality: Self = Self("NikonQuality");
    pub const NikonColorMode: Self = Self("NikonColorMode");
    pub const NikonImageAdjustment: Self = Self("NikonImageAdjustment");
    pub const NikonCcdSensitivity: Self = Self("NikonCCDSensitivity");
    pub const NikonWhiteBalanceFine: Self = Self("NikonWhiteBalanceFine");
    pub const NikonIsoSetting: Self = Self("NikonISOSetting");
    pub const NikonImageOptimization: Self = Self("NikonImageOptimization");
    pub const NikonSaturationAdjust: Self = Self("NikonSaturationAdjust");
    pub const NikonSharpnessAdjust: Self = Self("NikonSharpnessAdjust");
    pub const NikonFocusMode: Self = Self("NikonFocusMode");
    pub const NikonFlashMode: Self = Self("NikonFlashMode");
    pub const NikonShootingMode: Self = Self("NikonShootingMode");
    pub const NikonAutoBracketRelease: Self = Self("NikonAutoBracketRelease");
    pub const NikonLensType: Self = Self("NikonLensType");
    pub const NikonLens: Self = Self("NikonLens");

    // === Sony MakerNotes ===
    pub const SonyMake: Self = Self("SonyMake");
    pub const SonyImageSize: Self = Self("SonyImageSize");
    pub const SonyQuality: Self = Self("SonyQuality");
    pub const SonyFlashMode: Self = Self("SonyFlashMode");
    pub const SonyExposureMode: Self = Self("SonyExposureMode");
    pub const SonyFocusMode: Self = Self("SonyFocusMode");
    pub const SonyWhiteBalanceMode: Self = Self("SonyWhiteBalanceMode");
    pub const SonyMacro: Self = Self("SonyMacro");
    pub const SonySharpness: Self = Self("SonySharpness");
    pub const SonySaturation: Self = Self("SonySaturation");
    pub const SonyContrast: Self = Self("SonyContrast");
    pub const SonyBrightness: Self = Self("SonyBrightness");
    pub const SonyLongExposureNoiseReduction: Self = Self("SonyLongExposureNoiseReduction");
    pub const SonyHighIsoNoiseReduction: Self = Self("SonyHighISONoiseReduction");
    pub const SonyHdr: Self = Self("SonyHDR");
    pub const SonyMultiFrameNr: Self = Self("SonyMultiFrameNR");

    // === Fuji MakerNotes ===
    pub const FujiQuality: Self = Self("FujiQuality");
    pub const FujiSaturation: Self = Self("FujiSaturation");
    pub const FujiWhiteBalanceFineTune: Self = Self("FujiWhiteBalanceFineTune");
    pub const FujiHighIs0NoiseReduction: Self = Self("FujiHighIS0NoiseReduction");
    pub const FujiFocusMode: Self = Self("FujiFocusMode");
    pub const FujiAfMode: Self = Self("FujiAFMode");
    pub const FujiFocusPixel: Self = Self("FujiFocusPixel");
    pub const FujiImageSize: Self = Self("FujiImageSize");
    pub const FujiDualImageStabilization: Self = Self("FujiDualImageStabilization");
    pub const FujiFaceDetection: Self = Self("FujiFaceDetection");
    pub const FujiNumFaceElements: Self = Self("FujiNumFaceElements");

    // === Panasonic MakerNotes ===
    pub const PanasonicImageQuality: Self = Self("PanasonicImageQuality");
    pub const PanasonicColorMode: Self = Self("PanasonicColorMode");
    pub const PanasonicImageStabilization: Self = Self("PanasonicImageStabilization");
    pub const PanasonicMacroMode: Self = Self("PanasonicMacroMode");
    pub const PanasonicFocusMode: Self = Self("PanasonicFocusMode");
    pub const PanasonicAfAreaMode: Self = Self("PanasonicAFAreaMode");
    pub const PanasonicImageStabilization2: Self = Self("PanasonicImageStabilization2");
    pub const PanasonicBabyAge: Self = Self("PanasonicBabyAge");
    pub const PanasonicBabyName: Self = Self("PanasonicBabyName");

    // === Olympus MakerNotes ===
    pub const OlympusImageQuality: Self = Self("OlympusImageQuality");
    pub const OlympusMacroMode: Self = Self("OlympusMacroMode");
    pub const OlympusDigitalZoom: Self = Self("OlympusDigitalZoom");
    pub const OlympusVersion: Self = Self("OlympusVersion");
    pub const OlympusImageProcessing: Self = Self("OlympusImageProcessing");
    pub const OlympusFocusMode: Self = Self("OlympusFocusMode");
    pub const OlympusAfArea: Self = Self("OlympusAFArea");
    pub const OlympusAfPoint: Self = Self("OlympusAFPoint");
    pub const OlympusImageStabilization: Self = Self("OlympusImageStabilization");
    pub const OlympusColorSpace: Self = Self("OlympusColorSpace");

    // === Pentax MakerNotes ===
    pub const PentaxModelType: Self = Self("PentaxModelType");
    pub const PentaxImageSize: Self = Self("PentaxImageSize");
    pub const PentaxQuality: Self = Self("PentaxQuality");
    pub const PentaxImageProcessing: Self = Self("PentaxImageProcessing");
    pub const PentaxFocusMode: Self = Self("PentaxFocusMode");
    pub const PentaxAfPoint: Self = Self("PentaxAFPoint");
    pub const PentaxAutoBracketing: Self = Self("PentaxAutoBracketing");
    pub const PentaxWhiteBalance: Self = Self("PentaxWhiteBalance");

    // === 更多 XMP 命名空间 ===
    pub const XmpXmpCreateDate: Self = Self("xmp:CreateDate");
    pub const XmpXmpModifyDate: Self = Self("xmp:ModifyDate");
    pub const XmpXmpMetadataDate: Self = Self("xmp:MetadataDate");
    pub const XmpXmpCreatorTool: Self = Self("xmp:CreatorTool");
    pub const XmpXmpRating: Self = Self("xmp:Rating");
    pub const XmpXmpLabel: Self = Self("xmp:Label");
    pub const XmpXmpNickname: Self = Self("xmp:Nickname");

    // === XMP IPTC Core ===
    pub const XmpIptcCity: Self = Self("Iptc4xmpCore:City");
    pub const XmpIptcCountry: Self = Self("Iptc4xmpCore:Country");
    pub const XmpIptcCountryCode: Self = Self("Iptc4xmpCore:CountryCode");
    pub const XmpIptcState: Self = Self("Iptc4xmpCore:State");
    pub const XmpIptcLocation: Self = Self("Iptc4xmpCore:Location");
    pub const XmpIptcSubjectCode: Self = Self("Iptc4xmpCore:SubjectCode");
    pub const XmpIptcIntellectualGenre: Self = Self("Iptc4xmpCore:IntellectualGenre");

    // === XMP IPTC Extension ===
    pub const XmpIptcExtDigitalSourceType: Self = Self("Iptc4xmpExt:DigitalSourceType");
    pub const XmpIptcExtDigitalGuide: Self = Self("Iptc4xmpExt:DigitalGuide");
    pub const XmpIptcExtEvent: Self = Self("Iptc4xmpExt:Event");
    pub const XmpIptcExtOrganisationInImage: Self = Self("Iptc4xmpExt:OrganisationInImage");
    pub const XmpIptcExtPersonInImage: Self = Self("Iptc4xmpExt:PersonInImage");
    pub const XmpIptcExtLocationShown: Self = Self("Iptc4xmpExt:LocationShown");

    // === XMP Photoshop ===
    pub const XmpPhotoshopDateCreated: Self = Self("photoshop:DateCreated");
    pub const XmpPhotoshopCity: Self = Self("photoshop:City");
    pub const XmpPhotoshopState: Self = Self("photoshop:State");
    pub const XmpPhotoshopCountry: Self = Self("photoshop:Country");
    pub const XmpPhotoshopCredit: Self = Self("photoshop:Credit");
    pub const XmpPhotoshopSource: Self = Self("photoshop:Source");
    pub const XmpPhotoshopInstructions: Self = Self("photoshop:Instructions");
    pub const XmpPhotoshopTransmissionReference: Self = Self("photoshop:TransmissionReference");
    pub const XmpPhotoshopUrgency: Self = Self("photoshop:Urgency");
    pub const XmpPhotoshopCategory: Self = Self("photoshop:Category");
    pub const XMP_PHOTOSHOP_SUPPLEMENTAL_CATEGORIES: Self =
        Self("photoshop:SupplementalCategories");
    pub const XmpPhotoshopHeadline: Self = Self("photoshop:Headline");
    pub const XmpPhotoshopCaptionWriter: Self = Self("photoshop:CaptionWriter");

    // === XMP Camera Raw ===
    pub const XmpCrsVersion: Self = Self("crs:Version");
    pub const XmpCrsWhiteBalance: Self = Self("crs:WhiteBalance");
    pub const XmpCrsTemperature: Self = Self("crs:Temperature");
    pub const XmpCrsTint: Self = Self("crs:Tint");
    pub const XmpCrsExposure: Self = Self("crs:Exposure");
    pub const XmpCrsShadows: Self = Self("crs:Shadows");
    pub const XmpCrsBrightness: Self = Self("crs:Brightness");
    pub const XmpCrsContrast: Self = Self("crs:Contrast");
    pub const XmpCrsSaturation: Self = Self("crs:Saturation");
    pub const XmpCrsSharpness: Self = Self("crs:Sharpness");
    pub const XmpCrsLuminanceSmoothing: Self = Self("crs:LuminanceSmoothing");
    pub const XmpCrsColorNoiseReduction: Self = Self("crs:ColorNoiseReduction");
    pub const XmpCrsVignetteAmount: Self = Self("crs:VignetteAmount");

    // === XMP AUX (Camera Raw Schema) ===
    pub const XmpAuxSerialNumber: Self = Self("aux:SerialNumber");
    pub const XmpAuxLensInfo: Self = Self("aux:LensInfo");
    pub const XmpAuxLens: Self = Self("aux:Lens");
    pub const XmpAuxLensId: Self = Self("aux:LensID");
    pub const XmpAuxLensSerialNumber: Self = Self("aux:LensSerialNumber");
    pub const XmpAuxImageNumber: Self = Self("aux:ImageNumber");
    pub const XmpAuxFlashCompensation: Self = Self("aux:FlashCompensation");
    pub const XmpAuxFirmware: Self = Self("aux:Firmware");

    // === XMP MM (Media Management) ===
    pub const XmpMmDocumentId: Self = Self("xmpMM:DocumentID");
    pub const XmpMmInstanceId: Self = Self("xmpMM:InstanceID");
    pub const XmpMmOriginalDocumentId: Self = Self("xmpMM:OriginalDocumentID");
    pub const XmpMmRenditionClass: Self = Self("xmpMM:RenditionClass");
    pub const XmpMmVersionId: Self = Self("xmpMM:VersionID");
    pub const XmpMmVersionModifier: Self = Self("xmpMM:VersionModifier");
    pub const XmpMmHistory: Self = Self("xmpMM:History");
    pub const XmpMmDerivedFrom: Self = Self("xmpMM:DerivedFrom");

    // === XMP TIFF ===
    pub const XmpTiffMake: Self = Self("tiff:Make");
    pub const XmpTiffModel: Self = Self("tiff:Model");
    pub const XmpTiffImageWidth: Self = Self("tiff:ImageWidth");
    pub const XmpTiffImageHeight: Self = Self("tiff:ImageHeight");
    pub const XmpTiffBitsPerSample: Self = Self("tiff:BitsPerSample");
    pub const XmpTiffCompression: Self = Self("tiff:Compression");
    pub const XmpTiffPhotometricInterpretation: Self = Self("tiff:PhotometricInterpretation");
    pub const XmpTiffOrientation: Self = Self("tiff:Orientation");
    pub const XmpTiffSamplesPerPixel: Self = Self("tiff:SamplesPerPixel");
    pub const XmpTiffPlanarConfiguration: Self = Self("tiff:PlanarConfiguration");
    pub const XmpTiffYcbcrSubSampling: Self = Self("tiff:YCbCrSubSampling");
    pub const XmpTiffYcbcrPositioning: Self = Self("tiff:YCbCrPositioning");
    pub const XmpTiffXResolution: Self = Self("tiff:XResolution");
    pub const XmpTiffYResolution: Self = Self("tiff:YResolution");
    pub const XmpTiffResolutionUnit: Self = Self("tiff:ResolutionUnit");

    // === XMP EXIF ===
    pub const XmpExifExposureTime: Self = Self("exif:ExposureTime");
    pub const XmpExifFNumber: Self = Self("exif:FNumber");
    pub const XmpExifExposureProgram: Self = Self("exif:ExposureProgram");
    pub const XmpExifSpectralSensitivity: Self = Self("exif:SpectralSensitivity");
    pub const XmpExifIsoSpeedRatings: Self = Self("exif:ISOSpeedRatings");
    pub const XmpExifDateTimeOriginal: Self = Self("exif:DateTimeOriginal");
    pub const XmpExifDateTimeDigitized: Self = Self("exif:DateTimeDigitized");
    pub const XmpExifComponentsConfiguration: Self = Self("exif:ComponentsConfiguration");
    pub const XmpExifCompressedBitsPerPixel: Self = Self("exif:CompressedBitsPerPixel");
    pub const XmpExifShutterSpeedValue: Self = Self("exif:ShutterSpeedValue");
    pub const XmpExifApertureValue: Self = Self("exif:ApertureValue");
    pub const XmpExifBrightnessValue: Self = Self("exif:BrightnessValue");
    pub const XmpExifExposureBiasValue: Self = Self("exif:ExposureBiasValue");
    pub const XmpExifMaxApertureValue: Self = Self("exif:MaxApertureValue");
    pub const XmpExifSubjectDistance: Self = Self("exif:SubjectDistance");
    pub const XmpExifMeteringMode: Self = Self("exif:MeteringMode");
    pub const XmpExifLightSource: Self = Self("exif:LightSource");
    pub const XmpExifFlash: Self = Self("exif:Flash");
    pub const XmpExifFocalLength: Self = Self("exif:FocalLength");
    pub const XmpExifFlashEnergy: Self = Self("exif:FlashEnergy");
    pub const XmpExifSpatialFrequencyResponse: Self = Self("exif:SpatialFrequencyResponse");
    pub const XmpExifFocalPlaneXResolution: Self = Self("exif:FocalPlaneXResolution");
    pub const XmpExifFocalPlaneYResolution: Self = Self("exif:FocalPlaneYResolution");
    pub const XmpExifFocalPlaneResolutionUnit: Self = Self("exif:FocalPlaneResolutionUnit");
    pub const XmpExifSubjectLocation: Self = Self("exif:SubjectLocation");
    pub const XmpExifExposureIndex: Self = Self("exif:ExposureIndex");
    pub const XmpExifSensingMethod: Self = Self("exif:SensingMethod");
    pub const XmpExifFileSource: Self = Self("exif:FileSource");
    pub const XmpExifSceneType: Self = Self("exif:SceneType");
    pub const XmpExifCfaPattern: Self = Self("exif:CFAPattern");
    pub const XmpExifCustomRendered: Self = Self("exif:CustomRendered");
    pub const XmpExifExposureMode: Self = Self("exif:ExposureMode");
    pub const XmpExifWhiteBalance: Self = Self("exif:WhiteBalance");
    pub const XmpExifDigitalZoomRatio: Self = Self("exif:DigitalZoomRatio");
    pub const XmpExifFocalLengthIn35mmFilm: Self = Self("exif:FocalLengthIn35mmFilm");
    pub const XmpExifSceneCaptureType: Self = Self("exif:SceneCaptureType");
    pub const XmpExifGainControl: Self = Self("exif:GainControl");
    pub const XmpExifContrast: Self = Self("exif:Contrast");
    pub const XmpExifSaturation: Self = Self("exif:Saturation");
    pub const XmpExifSharpness: Self = Self("exif:Sharpness");
    pub const XmpExifDeviceSettingDescription: Self = Self("exif:DeviceSettingDescription");
    pub const XmpExifSubjectDistanceRange: Self = Self("exif:SubjectDistanceRange");
    pub const XmpExifImageUniqueId: Self = Self("exif:ImageUniqueID");

    // === 视频特定标签 ===
    pub const DURATION: Self = Self("Duration");
    pub const VideoFrameRate: Self = Self("VideoFrameRate");
    pub const VideoFrameCount: Self = Self("VideoFrameCount");
    pub const VideoBitRate: Self = Self("VideoBitRate");
    pub const AudioBitRate: Self = Self("AudioBitRate");
    pub const VideoCompression: Self = Self("VideoCompression");
    pub const AudioCompression: Self = Self("AudioCompression");
    pub const TrackNumber: Self = Self("TrackNumber");
    pub const TrackType: Self = Self("TrackType");
    pub const TrackCreateDate: Self = Self("TrackCreateDate");
    pub const TrackModifyDate: Self = Self("TrackModifyDate");
    pub const MediaCreateDate: Self = Self("MediaCreateDate");
    pub const MediaModifyDate: Self = Self("MediaModifyDate");
    pub const MediaDataSize: Self = Self("MediaDataSize");
    pub const MediaDataOffset: Self = Self("MediaDataOffset");
    pub const GENRE: Self = Self("Genre");
    // ARTIST 已在上面定义
    pub const ALBUM: Self = Self("Album");
    pub const YEAR: Self = Self("Year");
    pub const COMMENT: Self = Self("Comment");
    pub const LYRICS: Self = Self("Lyrics");
    pub const COMPOSER: Self = Self("Composer");
    pub const PUBLISHER: Self = Self("Publisher");

    // === 更多文件信息 ===
    pub const FileDescription: Self = Self("FileDescription");
    pub const FileVersion: Self = Self("FileVersion");
    pub const InternalVersionNumber: Self = Self("InternalVersionNumber");
    pub const CompanyName: Self = Self("CompanyName");
    pub const LegalCopyright: Self = Self("LegalCopyright");
    pub const ProductName: Self = Self("ProductName");
    pub const ProductVersion: Self = Self("ProductVersion");
    pub const MimeEncoding: Self = Self("MIMEEncoding");

    // === 扩展标签: 更多 Canon MakerNotes (50个) ===
    pub const CanonSerialNumberExt: Self = Self("SerialNumber");
    pub const CanonFirmwareVersionExt: Self = Self("FirmwareVersion");
    pub const CanonOwnerNameExt: Self = Self("OwnerName");
    pub const CanonTimeZoneExt: Self = Self("TimeZone");
    pub const CanonDaylightSavingExt: Self = Self("DaylightSaving");
    pub const CanonAfMicroAdjustment: Self = Self("AFMicroadjustment");
    pub const CanonFlashExposureCompExt: Self = Self("FlashExposureComp");
    pub const CanonBracketModeExt: Self = Self("BracketMode");
    pub const CanonBracketValueExt: Self = Self("BracketValue");
    pub const CanonRawJpgQualityExt: Self = Self("RawJpgQuality");
    pub const CanonRawJpgSizeExt: Self = Self("RawJpgSize");
    pub const CanonNoiseReductionExt: Self = Self("NoiseReduction");
    pub const CanonWbShiftGm: Self = Self("WBShiftGM");
    pub const CanonWbShiftAb: Self = Self("WBShiftAB");
    pub const CanonColorTemperatureExt: Self = Self("ColorTemperature");
    pub const CanonLensSerialNumberExt: Self = Self("LensSerialNumber");
    pub const CanonAfPointsInFocusExt: Self = Self("AFPointsInFocus");
    pub const CanonAfPointsSelectedExt: Self = Self("AFPointsSelected");
    pub const CanonAfPointsActiveExt: Self = Self("AFPointsActive");
    pub const CanonFocusDistanceUpperExt: Self = Self("FocusDistanceUpper");
    pub const CanonFocusDistanceLowerExt: Self = Self("FocusDistanceLower");
    pub const CanonFlashBitsExt: Self = Self("FlashBits");
    pub const CanonFocusContinuousExt: Self = Self("FocusContinuous");
    pub const CanonAeSettingExt: Self = Self("AESetting");
    pub const CanonDisplayApertureExt: Self = Self("DisplayAperture");
    pub const CanonZoomSourceWidthExt: Self = Self("ZoomSourceWidth");
    pub const CanonZoomTargetWidthExt: Self = Self("ZoomTargetWidth");
    pub const CanonSpotMeteringModeExt: Self = Self("SpotMeteringMode");
    pub const CanonPhotoEffectExt: Self = Self("PhotoEffect");
    pub const CanonManualFlashOutputExt: Self = Self("ManualFlashOutput");
    pub const CanonSrawQualityExt: Self = Self("SRAWQuality");
    pub const CanonGammaExt: Self = Self("Gamma");
    pub const CanonHighSpeedSync: Self = Self("HighSpeedSync");
    pub const CanonAfPointsInfo: Self = Self("AFPointsInfo");
    pub const CanonMeasureRoll: Self = Self("MeasureRoll");
    pub const CanonMeasurePitch: Self = Self("MeasurePitch");
    pub const CanonMeasureYaw: Self = Self("MeasureYaw");
    pub const CanonMeasureAccelX: Self = Self("MeasureAccelX");
    pub const CanonMeasureAccelY: Self = Self("MeasureAccelY");
    pub const CanonMeasureAccelZ: Self = Self("MeasureAccelZ");
    pub const CanonDateStampMode: Self = Self("DateStampMode");
    pub const CanonMyColorsModeExt: Self = Self("MyColorsMode");
    pub const CanonFirmwareRevision: Self = Self("FirmwareRevision");
    pub const CanonImageUniqueIdExt: Self = Self("ImageUniqueID");
    pub const CanonHdrSettingExt: Self = Self("HDRSetting");
    pub const CanonMultipleExposureExt: Self = Self("MultipleExposure");
    pub const CanonFilterEffectExt: Self = Self("FilterEffect");
    pub const CanonToningEffectExt: Self = Self("ToningEffect");

    // === 扩展标签: 更多 Nikon MakerNotes (50个) ===
    pub const NikonWhiteBalanceFineTuneExt: Self = Self("WhiteBalanceFineTune");
    pub const NikonColorSpaceExt: Self = Self("ColorSpace");
    pub const NikonVignetteControlExt: Self = Self("VignetteControl");
    pub const NikonAutoDistortionControlExt: Self = Self("AutoDistortionControl");
    pub const NikonPictureControlExt: Self = Self("PictureControl");
    pub const NikonHighIsoNrExt: Self = Self("HighISONoiseReduction");
    pub const NikonLongExposureNrExt: Self = Self("LongExposureNoiseReduction");
    pub const NikonActiveDLightingExt: Self = Self("ActiveDLighting");
    pub const NikonMultipleExposureModeExt: Self = Self("MultipleExposureMode");
    pub const NikonMultiExposureShotsExt: Self = Self("MultiExposureShots");
    pub const NikonHdrExt: Self = Self("HDR");
    pub const NikonVrModeExt: Self = Self("VRMode");
    pub const NikonVrInfoExt: Self = Self("VRInfo");
    pub const NikonFirmwareVersionExt: Self = Self("NikonFirmwareVersion");
    pub const NikonAfPointsUsedExt: Self = Self("AFPointsUsed");
    pub const NikonAfPointsInFocusExt: Self = Self("AFPointsInFocus");
    pub const NikonAfPointsSelectedExt: Self = Self("AFPointsSelected");
    pub const NikonSceneModeExt: Self = Self("SceneMode");
    pub const NikonLightingTypeExt: Self = Self("LightingType");
    pub const NikonShutterCountExt: Self = Self("ShutterCount");
    pub const NikonElectronicShutterCountExt: Self = Self("ElectronicShutterCount");
    pub const NikonNefBitDepthExt: Self = Self("NEFBitDepth");
    pub const NikonOptimizationExt: Self = Self("Optimization");
    pub const NikonSaturationExt: Self = Self("NikonSaturation");
    pub const NikonAfAreaXPositionExt: Self = Self("AFAreaXPosition");
    pub const NikonAfAreaYPositionExt: Self = Self("AFAreaYPosition");
    pub const NikonPhaseDetectAfExt: Self = Self("PhaseDetectAF");
    pub const NikonPrimaryAfPointExt: Self = Self("PrimaryAFPoint");
    pub const NikonContrastDetectAfExt: Self = Self("ContrastDetectAF");
    pub const NikonAfAreaPointsExt: Self = Self("AFAreaPoints");
    pub const NikonSerialNumber2Ext: Self = Self("NikonSerialNumber2");
    pub const NikonShutterCount2Ext: Self = Self("NikonShutterCount2");
    pub const NikonFlashMode2Ext: Self = Self("NikonFlashMode2");
    pub const NikonFlashControlModeExt: Self = Self("FlashControlMode");
    pub const NikonFlashExposureComp2Ext: Self = Self("NikonFlashExposureComp2");
    pub const NikonFlashExposureBracketValueExt: Self = Self("FlashExposureBracketValue");
    pub const NikonExternalFlashBounceExt: Self = Self("ExternalFlashBounce");
    pub const NikonExternalFlashZoomExt: Self = Self("ExternalFlashZoom");
    pub const NikonExternalFlashModeExt: Self = Self("ExternalFlashMode");
    pub const NikonExternalFlashCompensationExt: Self = Self("ExternalFlashCompensation");
    pub const NikonCommanderChannelExt: Self = Self("CommanderChannel");
    pub const NikonCommanderGroupAModeExt: Self = Self("CommanderGroupAMode");
    pub const NikonCommanderGroupACompensationExt: Self = Self("CommanderGroupACompensation");

    // === 扩展标签: 更多 Sony MakerNotes (30个) ===
    pub const SonyLensSpecExt: Self = Self("SonyLensSpec");
    pub const SonyColorTemperatureExt: Self = Self("SonyColorTemperature");
    pub const SonyColorCompensationFilterExt: Self = Self("SonyColorCompensationFilter");
    pub const SonyWhiteBalanceFineTuneExt: Self = Self("SonyWhiteBalanceFineTune");
    pub const SonyImageStabilizationStateExt: Self = Self("SonyImageStabilizationState");
    pub const SonyDynamicRangeOptimizerExt: Self = Self("DynamicRangeOptimizer");
    pub const SonyIntelligentAutoExt: Self = Self("IntelligentAuto");
    pub const SonyFlashLevelExt: Self = Self("FlashLevel");
    pub const SonyReleaseModeExt: Self = Self("ReleaseMode");
    pub const SonySequenceNumberExt: Self = Self("SequenceNumber");
    pub const SonyFocusStatusExt: Self = Self("FocusStatus");
    pub const SonyAfAidedExt: Self = Self("AFAided");
    pub const SonyAfAreaModeExt: Self = Self("AFAreaMode");
    pub const SonyAfPointSelectedExt: Self = Self("AFPointSelected");
    pub const SonyAfStatusExt: Self = Self("AFStatus");
    pub const SonyLensMountExt: Self = Self("SonyLensMount");
    pub const SonyLensFormatExt: Self = Self("SonyLensFormat");
    pub const SonyDistortionCorrectionExt: Self = Self("DistortionCorrection");
    pub const SONY_CHROMATIC_ABERRATION_CORRECTION_EXT: Self =
        Self("ChromaticAberrationCorrection");
    pub const SonyVignettingCorrectionExt: Self = Self("VignettingCorrection");
    pub const SonyShadingCompensationExt: Self = Self("ShadingCompensation");
    pub const SonyHdrSettingExt: Self = Self("SonyHDRSetting");
    pub const SonyHdrAlignmentExt: Self = Self("HDRAlignment");
    pub const SonyPanoramaDirectionExt: Self = Self("PanoramaDirection");
    pub const SonyPanoramaAngleExt: Self = Self("PanoramaAngle");
    pub const SonyMultiFrameNrExt: Self = Self("MultiFrameNoiseReduction");
    pub const SonyPictureEffectExt: Self = Self("PictureEffect");
    pub const SonySoftSkinEffectExt: Self = Self("SoftSkinEffect");
    pub const SonyAutoPortraitFramedExt: Self = Self("AutoPortraitFramed");
    pub const SonyAfIlluminatorExt: Self = Self("AFIlluminator");

    // === 扩展标签: 视频元数据 (30个) ===
    pub const VideoTrackCreateDateExt: Self = Self("TrackCreateDate");
    pub const VideoTrackModifyDateExt: Self = Self("TrackModifyDate");
    pub const VideoMediaCreateDateExt: Self = Self("MediaCreateDate");
    pub const VideoMediaModifyDateExt: Self = Self("MediaModifyDate");
    pub const VideoHandlerTypeExt: Self = Self("HandlerType");
    pub const VideoHandlerDescriptionExt: Self = Self("HandlerDescription");
    pub const VideoCompressorIdExt: Self = Self("CompressorID");
    pub const VideoBitsPerComponentExt: Self = Self("BitsPerComponent");
    pub const VideoColorProfileExt: Self = Self("VideoColorProfile");
    pub const VideoAudioFormatExt: Self = Self("AudioFormat");
    pub const VideoAudioChannelsExt: Self = Self("AudioChannels");
    pub const VideoAudioBitsPerSampleExt: Self = Self("AudioBitsPerSample");
    pub const VideoAudioSampleRateExt: Self = Self("AudioSampleRate");
    pub const VideoDurationExt: Self = Self("Duration");
    pub const VideoMovieHeaderVersionExt: Self = Self("MovieHeaderVersion");
    pub const VideoTimeScaleExt: Self = Self("TimeScale");
    pub const VideoPreferredRateExt: Self = Self("PreferredRate");
    pub const VideoPreferredVolumeExt: Self = Self("PreferredVolume");
    pub const VideoPreviewTimeExt: Self = Self("PreviewTime");
    pub const VideoPreviewDurationExt: Self = Self("PreviewDuration");
    pub const VideoPosterTimeExt: Self = Self("PosterTime");
    pub const VideoSelectionTimeExt: Self = Self("SelectionTime");
    pub const VideoSelectionDurationExt: Self = Self("SelectionDuration");
    pub const VideoCurrentTimeExt: Self = Self("CurrentTime");
    pub const VideoNextTrackIdExt: Self = Self("NextTrackID");
    pub const VideoTrackIdExt: Self = Self("VideoTrackID");
    pub const VideoTrackLayerExt: Self = Self("VideoTrackLayer");
    pub const VideoTrackVolumeExt: Self = Self("VideoTrackVolume");
    pub const VideoTrackDurationExt: Self = Self("VideoTrackDuration");
    pub const VideoWidthExt: Self = Self("VideoWidth");

    // === 扩展标签: RAW/DNG (20个) ===
    pub const DngVersionExt: Self = Self("DNGVersion");
    pub const DngBackwardVersionExt: Self = Self("DNGBackwardVersion");
    pub const UniqueCameraModelExt: Self = Self("UniqueCameraModel");
    pub const LocalizedCameraModelExt: Self = Self("LocalizedCameraModel");
    pub const CfaPlaneColorExt: Self = Self("CFAPlaneColor");
    pub const CfaLayoutExt: Self = Self("CFALayout");
    pub const LinearizationTableExt: Self = Self("LinearizationTable");
    pub const BlackLevelExt: Self = Self("BlackLevel");
    pub const WhiteLevelExt: Self = Self("WhiteLevel");
    pub const DefaultScaleExt: Self = Self("DefaultScale");
    pub const BestQualityScaleExt: Self = Self("BestQualityScale");
    pub const DefaultCropOriginExt: Self = Self("DefaultCropOrigin");
    pub const DefaultCropSizeExt: Self = Self("DefaultCropSize");
    pub const CalibrationIlluminant1Ext: Self = Self("CalibrationIlluminant1");
    pub const CalibrationIlluminant2Ext: Self = Self("CalibrationIlluminant2");
    pub const ColorMatrix1Ext: Self = Self("ColorMatrix1");
    pub const ColorMatrix2Ext: Self = Self("ColorMatrix2");
    pub const CameraCalibration1Ext: Self = Self("CameraCalibration1");
    pub const CameraCalibration2Ext: Self = Self("CameraCalibration2");
    pub const AnalogBalanceExt: Self = Self("AnalogBalance");

    // === Other Tags (16609 tags) - feature = "other" ===
    // Note: Full other.rs integration in progress
    // All 16609 other tags available in src/tags/other.rs
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
///
/// ExifTool 的 `-json -G` 输出中，分组信息已通过键名中的 `Group:Tag` 格式体现，
/// 因此不再单独维护 groups 字段。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// 顶层标签
    #[serde(flatten)]
    tags: HashMap<String, TagValue>,
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
        assert_eq!(TagId::Make.name(), "Make");
        assert_eq!(TagId::Model.name(), "Model");
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
