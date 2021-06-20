use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Waypoint Name/Description" field sets the unabbreviated name of a
    named waypoint or a definition of an unnamed waypoint.

    The name of a named waypoint is spelled out in full. Definitions for
    unnamed waypoints are described in Chapter 7.
    */
    pub WaypointNameDescription: 25;
}
