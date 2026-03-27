//! IPTC 标签
//! Feature: iptc

use crate::TagId;

impl TagId {
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
}
