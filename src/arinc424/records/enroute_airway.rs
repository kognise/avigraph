use crate::arinc424::fields::*;
use crate::autoparse_struct;

autoparse_struct! {
    /**
    The Enroute Airways file will contain the sequential listing of officially published airways
    and other established ATS Routes by geographical areas. The file also contains published
    airways specific to helicopter operations.

    The standard length for the Route Identifier is five characters. Some users envisage the need
    for a six-character field. This reserved column will permit this usage. Some data suppliers
    may use this position for the ATS Service suffix associated with some Route Identifiers.
    */
    pub EnrouteAirway {
        #(skip 7),
        route_identifier: EnrouteAirwayRouteIdentifier,
        #(skip 1),

        #(skip 6),
        sequence_number: QuadSequenceNumber,
        fix_identifier: FixIdentifier,
        fix_icao_code: ICAOCode,

        #(skip 2), // Section and subsection code, hopefully safe to ignore?
        #(continuation 1),
        waypoint_description_code: WaypointDescriptionCode,
        boundary_code: BoundaryCode,
        route_type: RouteTypeEnrouteAirway,
        level: Level,
        direction_restriction: DirectionalRestriction,
        cruise_table: CruiseTableIdentifier,
        eu: EUIndicator,

        recommended_navaid: RecommendedNavaid,
        navaid_icao_code: ICAOCode,
        rnp: RNP,
        #(skip 3),

        theta: Theta,
        rho: Rho,
        outbound_course: OutboundMagneticCourse,
        route_distance_from: RouteHoldingDistanceTime,
        inbound_course: InboundMagneticCourse,
        #(skip 1),

        // Why are there two? Who knows?
        minimum_altitude_1: AltitudeOrMinimumAltitude,
        minimum_altitude_2: AltitudeOrMinimumAltitude,

        maximum_altitude: MaximumAltitude,
        fixed_radius_transition_indicator: FixedRadiusTransitionIndicator,
        #(skip 22),

        record_number: FileRecordNumber,
        cycle: Cycle,
    }
}
