use crate::autoparse_struct;
use crate::enum_parseable;

autoparse_struct! {
    /**
    The "Waypoint Type" field defines both the "type" and function of IFR
    waypoints and also define a waypoint as being VFR.
    */
    pub WaypointType {
        type1: WaypointType1,
        type2: WaypointType2,
        procedure: WaypointProcedure,
    }
}

enum_parseable! {
    pub enum WaypointType1 {
        ARCCenterFix = "A",

        /// **Combined Named Intersection and RNAV**
        NamedIntersectionRNAV = "C",

        UnnamedChartedIntersection = "I",
        MiddleMarker = "M",

        /// **(Terminal) NDB Navaid as Waypoint**
        NDB = "N",

        OuterMarker = "O",
        NamedIntersection = "R",
        UnchartedAirwayIntersection = "U",
        VFR = "V",
        RNAV = "W",

        None = " ",
    }
}

enum_parseable! {
    pub enum WaypointType2 {
        /// **Final Approach Fix**
        FAF = "A",

        /// **Initial and Final Approach Fix**
        IAFAndFAF = "B",

        /// **Final Approach Course Fix**
        FACF = "C",

        /// **Intermediate Approach Fix**
        IF = "D",

        /// **Off-Route intersection in the FAA National Reference System**
        OffRouteIntersectionNRS = "E",

        OffRouteIntersection = "F",

        /// **Initial Approach Fix**
        IAF = "I",

        /// **Final Approach Course Fix at Initial Approach Fix**
        IAFAndFACF = "K",

        /// **Final Approach Course Fix at Intermediate Approach Fix**
        IFAndFACF = "L",

        /// **Missed Approach Fix**
        MAF = "M",

        /// **Initial Approach Fix and Missed Approach Fix
        IAFAndMAF = "N",

        OceanicEntryExit = "O",

        /// **Pitch and Catch Point in the FAA High Altitude Redesign**
        PitchCatchPoint = "P",

        NamedIntersection = "R",

        /// **AACAA and SUA Waypoints in the FAA High Altitude Redesign**
        AACAAAndSUA = "S",

        /// **FIR/UIR or Controlled Airspace Intersection**
        ControlledIntersection = "U",

        /// **Enroute: Latitude/Longitude Intersection, Full Degree of Latitude**
        ///
        /// **Terminal: VFR Waypoint**
        CoordinateFullOrVFR = "V",

        /// **Enroute: Latitude/Longitude Intersection, Half Degree of Latitude**
        ///
        /// **Terminal: RNAV Waypoint**
        CoordinateHalfOrRNAV = "W",

        None = " ",
    }
}

enum_parseable! {
    pub enum WaypointProcedure {
        SID = "D",
        STAR = "E",
        Approach = "F",
        Multiple = "Z",
        None = " ",
    }
}
