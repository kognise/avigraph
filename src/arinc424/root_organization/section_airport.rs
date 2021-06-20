use crate::enum_parseable;

enum_parseable! {
    pub enum AirportSubsection {
        Airport = "A",
        Gates = "B",
        TerminalWaypoints = "C",
        SIDs = "D",
        STARs = "E",
        ApproachProcedures = "F",
        Runways = "G",
        LocalizerGlideSlope = "I",
        MLS = "L",
        LocalizerMarkers = "M",
        TerminalNDBs = "N",
        Pathpoint = "P",
        FlightPlanningArrDep = "R",
        MSA = "S",
        GLSStation = "T",
        Communications = "V",
        None = " ",
    }
}
