use crate::enum_parseable;

enum_parseable! {
    pub enum HeliportSubsection {
        Heliport = "A",
        TerminalWaypoints = "C",
        SIDs = "D",
        STARs = "E",
        ApproachProcedures = "F",
        MSA = "S",
        Communications = "V",
        None = " ",
    }
}
