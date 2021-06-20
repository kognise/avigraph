use crate::arinc424::fields::*;
use crate::autoparse_struct;

autoparse_struct! {
    /**
    The Enroute Holding Patterns contained in this file are holding patterns
    recommended by the official government authority for inclusion on enroute
    aeronautical charts. The Terminal Holding Patterns included in this file
    are holding patterns recommended for aeronautical charts for the
    geographical area of an airport or heliport. The type, Enroute or Terminal,
    will be determined by the Subsection of the fix upon which the holding is
    predicated.
    */
    pub HoldingPattern {
        region_code: RegionCode,
        region_icao_code: ICAOCode,
        #(skip 15),

        duplicate_identifier: DuplicateIdentifier,
        fix_identifier: FixIdentifier,
        fix_icao_code: ICAOCode,
        #(skip 2), // Section/subsection codes, shouldn't be important
        #(continuation 1),

        inbound_holding_course: InboundHoldingCourse,
        turn_direction: Turn,
        leg_length: LegLength,
        leg_time: LegTime,

        minimum_altitude: AltitudeOrMinimumAltitude,
        maximum_altitude: MaximumAltitude,
        holding_speed: HoldingSpeed,
        rnp: RNP,
        arc_radius: ArcRadius,

        #(skip 27),
        name: Name25,

        record_number: FileRecordNumber,
        cycle: Cycle,
    }
}
