use crate::arinc424::fields::*;
use crate::autoparse_struct;

autoparse_struct! {
    /**
    The Enroute Waypoint file (EA) contains all enroute onairway and off-airway waypoints
    within a desired geographical area. The Airport Terminal Waypoint file (PC) contains
    all terminal waypoints and VFR waypoints within the geographical area of each airport.
    Airport Terminal Waypoints utilized by two or more airports will be stored in the
    Enroute Waypoint Subsection (EA) to eliminate duplication. Terminal Waypoints used
    jointly by an airport and a heliport are also stored in the Enroute Waypoint file. The
    Enroute Waypoint File will contain waypoints established for Helicopter Airways. For
    Heliport Terminal Waypoints (HC) see Section 4.2.2.
    */
    pub Waypoint {
        region_code: RegionCode,
        region_icao_code: ICAOCode,
        #(skip 1), // Subsection code, handled by main parser

        waypoint_identifier: FixIdentifier,
        #(skip 1),
        waypoint_icao_code: ICAOCode,
        #(continuation 1, skip 4),
        waypoint_type: WaypointType,
        waypoint_usage: WaypointUsage,
        #(skip 1),
        waypoint_latitude: Latitude,
        waypoint_longitude: Longitude,
        #(skip 23),

        dynamic_magnetic_variation: MagneticVariation,
        #(skip 5),
        datum: DatumCode,
        #(skip 8),
        name_format: NameFormatIndicator,
        name_description: WaypointNameDescription,

        record_number: FileRecordNumber,
        cycle: Cycle,
    }
}
