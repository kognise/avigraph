use crate::autoparse_struct;
use crate::enum_parseable;

autoparse_struct! {
    /**
    The waypoint usage field is employed to indicate the structure in which
    the waypoint is utilized.
    */
    pub WaypointUsage {
        usage1: WaypointUsage1,
        usage2: WaypointUsage2,
    }
}

enum_parseable! {
    pub enum WaypointUsage1 {
        RNAV = "R",
        None = " ",
    }
}

enum_parseable! {
    pub enum WaypointUsage2 {
        HighLow = "B",
        High = "H",
        Low = "L",
        TerminalOnly = " ",
    }
}
