use crate::autoparse_struct;
use crate::enum_parseable;

enum_parseable! {
    /**
    The "Route Type" field defines the type of Enroute Airway, Preferred Route, Airport and
    Heliport SID/STAR/Approach Routes of which the record is an element. For Airport and
    Heliport Approach Routes, "Route Type" includes a "primary route type," and up to two
    "route type qualifiers."
    */
    pub enum RouteTypeEnrouteAirway {
        /// **Airline Airway (Tailored Data)**
        Airline = "A",

        Control = "C",
        DirectRoute = "D",
        Helicopter = "H",

        /// **Officially Designated Airways, except RNAV, Helicopter Airways**
        NotRNAVOrHelicopter = "O",

        RNAV = "R",
        UndesignatedATSRoute = "S",
        None = " ",
    }
}

enum_parseable! {
    /**
    The "Route Type" field defines the type of Enroute Airway, Preferred Route, Airport and
    Heliport SID/STAR/Approach Routes of which the record is an element. For Airport and
    Heliport Approach Routes, "Route Type" includes a "primary route type," and up to two
    "route type qualifiers."
    */
    pub enum RouteTypePreferredRoute {
        /// **North American Route for North Atlantic Traffic - Common Portion**
        NARNATCommon = "C",

        Preferential = "D",

        /// **Pacific Oceanic Transition Route**
        PACOTS = "J",

        TACANAustralia = "M",

        /// **North American Route for North Atlantic Traffic - Non-common Portion**
        NARNATNonCommon = "N",

        /// **Preferred/Preferential Overflight Route**
        PreferredOverflightRoute = "0",

        Preferred = "P",

        /// **Tower Orientation System Route**
        TOS = "S",

        /// **Tower Enroute Control Route**
        TEC = "T",

        None = " ",
    }
}

enum_parseable! {
    /**
    The "Route Type" field defines the type of Enroute Airway, Preferred Route, Airport and
    Heliport SID/STAR/Approach Routes of which the record is an element. For Airport and
    Heliport Approach Routes, "Route Type" includes a "primary route type," and up to two
    "route type qualifiers."
    */
    pub enum RouteTypeAirportHeliportSID {
        EngineOutSID = "0",
        SIDRunwayTransition = "1",
        SIDOrSIDCommmonRoute = "2",
        SIDEnrouteTransition = "3",
        RNAVSIDRunwayTransition = "4",
        RNAVSIDOrSIDCommmonRoute = "5",
        RNAVSIDEnrouteTransition = "6",
        FMSSIDRunwayTransition = "F",
        FMSSIDOrSIDCommmonRoute = "M",
        FMSSIDEnrouteTransition = "S",
        VectorSIDRunwayTransition = "T",
        VectorSIDEnrouteTransition = "V",
        None = " ",
    }
}

enum_parseable! {
    /**
    The "Route Type" field defines the type of Enroute Airway, Preferred Route, Airport and
    Heliport SID/STAR/Approach Routes of which the record is an element. For Airport and
    Heliport Approach Routes, "Route Type" includes a "primary route type," and up to two
    "route type qualifiers."
    */
    pub enum RouteTypeAirportHeliportSTAR {
        STAREnrouteTransition = "1",
        STAROrSTARCommonRoute = "2",
        STARRunwayTransition = "3",
        RNAVSTAREnrouteTransition = "4",
        RNAVSTAROrSTARCommonRoute = "5",
        RNAVSTARRunwayTransition = "6",
        ProfileDescentEnrouteTransition = "7",
        ProfileDescentCommonRoute = "8",
        ProfileDescent = "9",
        FMSSTAREnrouteTransition = "F",
        FMSSTAROrSTARCommonRoute = "M",
        FMSSTARRunwayTransition = "S",
        None = " ",
    }
}

enum_parseable! {
    /**
    The "Route Type" field defines the type of Enroute Airway, Preferred Route, Airport and
    Heliport SID/STAR/Approach Routes of which the record is an element. For Airport and
    Heliport Approach Routes, "Route Type" includes a "primary route type," and up to two
    "route type qualifiers."

    *The content of this Route Type is split in the record layout. The Primary Route Type is in
    column 20 and the Qualifiers 1 and 2 in columns 119 and 120. Both the Primary Route Type and
    the Qualifier 1 and Qualifier 2 are used to sort the file content, see
    `RouteTypeAirportHeliportApproachQualifiers` to parse these.*
    */
    pub enum RouteTypeAirportHeliportApproach {
        ApproachTransition = "A",
        LocalizerOrBackcourse = "B",
        VORDME = "D",
        FMS = "F",
        IGS = "G",
        ILS = "I",
        GLS = "J",
        LOC = "L",
        MLS = "M",
        NDB = "N",
        GPS = "P",
        NDBDME = "Q",

        /**
        Route Type "R" is used for all types of RNAV procedure coding except GLS. The type of
        RNAV procedure is further defined through the content of Qualifier 1. Conventional Area
        Navigation Approach using RHO-RHO or RHO-THETA equipment is Route Type "R," Qualifier
        1 "T" or "V." RNAV (GPS) Approach procedures should be coded as Route Type "R,"
        Qualifier 1 "J," "L," "R," "U" or "W," as appropriate.
        */
        RNAV = "R",

        /// **VOR Approach using VORDME/VORTAC**
        VORUsingVORDMh = "S",

        TACAN = "T",
        SDF = "U",
        VOR = "V",
        MLSTypeA = "W",
        LDA = "X",
        MLSTypeBOrC = "Y",
        Missed = "Z",
        None = " ",
    }
}

autoparse_struct! {
    /**
    *This is the qualifiers part of the split record, the main type is in
    `RouteTypeAirportHeliportApproach`.*
    */
    pub RouteTypeAirportHeliportApproachQualifiers {
        qualifier1: RouteTypeAirportHeliportApproachQualifier1,
        qualifier2: RouteTypeAirportHeliportApproachQualifier2,
    }
}

enum_parseable! {
    /**
    The Qualifiers 1 values of "D" and "N" can be used in conjunction with any Route Type except
    for Route Type "R," "V," "D," "S," "N" and "Q." The values in Qualifier 1 should be set
    consistently for all portions of the coded procedure. Qualifier 2 is additionally provided
    on the approach and missed approach segments. There is no defined qualifier 2 content for
    approach transitions.
    */
    pub enum RouteTypeAirportHeliportApproachQualifier1 {
        DMERequired = "D",

        /// **GPS required, DME/DME to RNP xx.x not authorized**
        GPSRequiredDMEDMENotAuthorized = "J",

        /**
        Although there seems to be universal acceptance of GLS as the procedure title for GBAS
        procedures, the Qualifier 1 Code of "L" that could be applied to an RNAV or RNAV (GPS)
        procedure published with appropriate minimums has been retained as a precaution.
        */
        GBAS = "L",

        GPSRequired = "P",

        /// **GPS or DME/DME to RNP xx.x required**
        GPSOrDMEDMERequired = "R",

        DMEDMERequired = "T",

        /// **RNAV, Sensor Not Specified**
        RNAVSensorNS = "U",

        VORDMERNAV = "V",

        /// Path Point Data Records are required to support these Approach Procedure Types.
        SBAS = "W",

        None = " ",
    }
}

enum_parseable! {
    /**
    Qualifier 2 values "A" and "B" can be used only in conjunction with Route Type "Z" (missed.)
    */
    pub enum RouteTypeAirportHeliportApproachQualifier2 {
        PrimaryMissedApproach = "A",
        SecondaryMissedApproach = "B",
        EngineOutMissedApproach = "E",
        CircleToLandMinimums = "C",
        StraightInMinimums = "S",
        None = " ",
    }
}
