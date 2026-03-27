//! GPS 标签
//! Feature: gps

use crate::TagId;

impl TagId {
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
}
