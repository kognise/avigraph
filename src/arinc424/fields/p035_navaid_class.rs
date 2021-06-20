use crate::autoparse_struct;
use crate::enum_parseable;

autoparse_struct! {
    /**
    The "Navaid Class" field provides information in coded format on the type of
    navaid, the useable range or assigned output power of the navaid, information
    carried on the navaid signal and collocation of navaids in both an electronic
    and aeronautical sense. The field is made up of five columns of codes that define
    this information.

    The information for the five columns is derived from official government sources.

    *This is a split field, `NDBNavaidClass` contains the equivalent for NDBs.*
    */
    pub VORNavaidClass {
        type1: VORType1,
        type2: VORType2,
        power: VORPower,
        additional_info: AdditionalInfo,
        collocation: VORCollocation,
    }
}

autoparse_struct! {
    /**
    The "Navaid Class" field provides information in coded format on the type of
    navaid, the useable range or assigned output power of the navaid, information
    carried on the navaid signal and collocation of navaids in both an electronic
    and aeronautical sense. The field is made up of five columns of codes that define
    this information.

    The information for the five columns is derived from official government sources.

    *This is a split field, `VORNavaidClass` contains the equivalent for VORs.*
    */
    pub NDBNavaidClass {
        type1: NDBType1,
        type2: NDBType2,
        power: NDBPower,
        additional_info: AdditionalInfo,
        collocation: NDBCollocation,
    }
}

enum_parseable! {
    pub enum VORType1 {
        VOR = "V",
        None = " ",
    }
}

enum_parseable! {
    pub enum NDBType1 {
        NDB = "H",
        SABH = "S",
        MarineBeacon = "M",
    }
}

enum_parseable! {
    pub enum VORType2 {
        DME = "D",
        TACAN = "T",
        MilitaryTACAN = "M",
        ILSDME = "I",
        MLSDMEN = "N",
        MLSDMEP = "P",
        None = " ",
    }
}

enum_parseable! {
    pub enum NDBType2 {
        InnerMarker = "I",
        MiddleMarker = "M",
        OuterMarker = "O",
        BackMarker = "C",
        None = " ",
    }
}

enum_parseable! {
    pub enum VORPower {
        Terminal = "T",
        LowAltitude = "L",
        HighAltitude = "H",

        /**
        VHF Navaid Records, Range/Power. The character "U" is entered into column
        30 on VHF Navaid Records when the official government source has not defined
        the use of the facility or has not restricted such use by range or altitude.
        */
        Undefined = "U",

        /**
        The character "C" is entered into column 30 when that record contains a
        TACAN Navaid that is frequency paired with an ILS Localizer with the same
        identifier at the same location. The character is only used in combination
        with the character "I" = ILSDME in column 29. If the "C" appears in column
        30, the Range is understood to be "T" = Terminal use. Note that in some output
        files, this TACAN Navaid may be listed twice, once as a TACAN and once as
        an ILSTACAN, depending on the individual government source publication.
        */
        ILSTACAN = "C",
    }
}

enum_parseable! {
    pub enum NDBPower {
        WattsGT200 = "H",
        Watts50To1999 = " ",
        Watts20ToLT50 = "M",
        WattsLT25 = "L",
    }
}

enum_parseable! {
    pub enum AdditionalInfo {
        BiasedILS = "D",
        AutomaticTranscribedWX = "A",
        ScheduledWX = "B",
        NoVoice = "W",
        Voice = " ",
    }
}

enum_parseable! {
    pub enum VORCollocation {
        Collocated = " ",

        /**
        For VHF Navaid records, the character "N" in column 32 is entered if either the
        latitude and/or the longitude of the VOR and the Collocated DME or TACAN of a
        frequency paired VORDME or VORTAC differ by 1/10 arc minutes or more. Column 32
        is "blank" on VHF Navaids where the difference in latitude or longitude is less
        than the 1/10-arc minutes. Column 32 of the VHF Navaid will also carry the "N" or
        "blank" meaning listed above for frequency paired ILSDMEs and ILSTACANs. Note that
        in this later case, the character is carried on the ILSDME or ILSTACAN record as
        the Localizer record is not part of the VHF Navaid Section.
        */
        NotCollocated = "N",
    }
}

enum_parseable! {
    pub enum NDBCollocation {
        /**
        While not a "collocation" indication, the character "B" is entered in the Collocation
        Columns (32 of NDB Navaids and 79 of Airport/Heliport Localizer Marker/Locator Navaids)
        to indicate the type of signal emitted by the Navaid requires the use of a Beat
        Frequency Oscillator (BFO) to make the morse identifier transmission audible. Should both
        a collocation and a BFO condition exist for one and the same Navaid Record, preference is
        given to the collocation characters.
        */
        BFOOperation = "B",

        /**
        For Airport/Heliport Localizer Marker/Locator records, the character "A" in column 79 is
        entered if the latitude or longitude of a Marker and its aeronautically associated Locator
        differ by less than 1/10-arc minutes. Column 79 is left "blank" when the latitude and longitude
        of the Marker and Locator are exactly the same.
        */
        Collocated = "A" ;| " ",

        /**
        For Airport/Heliport Localizer Marker/Locator records, the character "N" in column 79 is
        entered if either the latitude or longitude of a Marker and it aeronautically associated
        Locator differ by 1/10-arc minutes or more.
        */
        NotCollocated = "N",
    }
}
