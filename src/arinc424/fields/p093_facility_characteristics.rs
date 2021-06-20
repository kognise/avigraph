use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;
use crate::autoparse_struct;
use crate::enum_parseable;

/**
The "Facility Characteristics" field identifies the characteristics of the NAVAID
facility.

*This is a very complex field due to the many variants.*
*/
#[derive(Debug)]
pub enum FacilityCharacteristics {
    NDB(FacilityCharacteristicsNDB),
    MLS(FacilityCharacteristicsMLS),
    NotNDBMLS(FacilityCharacteristicsNotNDBMLS),
    None,
}

impl Parseable for FacilityCharacteristics {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.peek(5)?;

        if data.trim().is_empty() {
            Some(Self::None)
        } else if data.get(2..3)? == " " {
            Some(Self::NDB(FacilityCharacteristicsNDB::parse(reader)?))
        } else if matches!(data.get(3..4)?, " " | "H") {
            Some(Self::MLS(FacilityCharacteristicsMLS::parse(reader)?))
        } else {
            Some(Self::NotNDBMLS(FacilityCharacteristicsNotNDBMLS::parse(
                reader,
            )?))
        }
    }
}

autoparse_struct! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub FacilityCharacteristicsNDB {
        #(skip 1),
        voice: FacilityCharacteristicsVoice,
        emission_type: FacilityCharacteristicsEmissionType,
        frequency: FacilityCharacteristicsEmissionFrequency,
        repetition_rate: FacilityCharacteristicsRepetitionRate,
    }
}

autoparse_struct! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub FacilityCharacteristicsMLS {
        timing: FacilityCharacteristicsTiming,
        voice: FacilityCharacteristicsVoice,
        #(skip 1),
        scan_rate: FacilityCharacteristicsScanRate,
        location: FacilityCharacteristicsLocation,
    }
}

autoparse_struct! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub FacilityCharacteristicsNotNDBMLS {
        timing: FacilityCharacteristicsTiming,
        voice: FacilityCharacteristicsVoice,
        #(skip 1),
        usability: FacilityCharacteristicsUsability,
        location: FacilityCharacteristicsLocation,
    }
}

enum_parseable! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub enum FacilityCharacteristicsTiming {
        Synchronous = "S",
        Asynchronous = "A",
        Unknown = "U",
    }
}

enum_parseable! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub enum FacilityCharacteristicsVoice {
        VoiceIdent = "Y",
        NoVoiceIdent = "N",
        Undefined = "U",
    }
}

enum_parseable! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub enum FacilityCharacteristicsEmissionType {
        /// Unmodulated Carrier
        A0 = "0",

        /// Carrier keyed
        A1 = "1",

        // Tone keyed modulation
        A2 = "2",
    }
}

enum_parseable! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub enum FacilityCharacteristicsEmissionFrequency {
        Frequency400H = "4",
        Frequency1020H = "1",
    }
}

/**
*This is only exported for typing convenience. Please only use `FacilityCharacteristics`
for parsing.*
*/
#[derive(Debug)]
pub enum FacilityCharacteristicsRepetitionRate {
    Known(u8),
    Unknown,
}

impl Parseable for FacilityCharacteristicsRepetitionRate {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(1)?;

        if data == " " {
            Some(Self::Unknown)
        } else {
            Some(Self::Known(data.parse().ok()?))
        }
    }
}

enum_parseable! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub enum FacilityCharacteristicsUsability {
        Usable = "Y",
        Unusable = "N",
        Restricted = "R",
        Undefined = "U",
    }
}

enum_parseable! {
    /**
    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub enum FacilityCharacteristicsLocation {
        CollocatedWithLocalizer = "L",
        CollocatedWithGlideSlope = "G",
        NotCollocatedWithLocalizerOrGlideSlope = " ",
        CollocatedWithAzimuth = "A",
        CollocatedWithElevation = "E",
        NotCollocatedWithAzimuthOrElevation = "N",
    }
}

enum_parseable! {
    /**
    **MLS Approach Azimuth Scan Rate**

    *This is only exported for typing convenience. Please only use `FacilityCharacteristics`
    for parsing.*
    */
    pub enum FacilityCharacteristicsScanRate {
        HighRateGuidance = "H",
        Undefined = " ",
    }
}
