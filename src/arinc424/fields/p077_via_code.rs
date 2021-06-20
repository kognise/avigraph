use crate::enum_parseable;

enum_parseable! {
    /**
    The "VIA Code" field is used to define the type of route used in the
    SID/STAR/Approach/Airways field (Section 5.78) on Company Route records and defines the
    type of route used in the AWY Identifier on Preferred Route records. On the Preferred
    Route records, some codes define the use, or restriction to use, of a fix or routing.
    */
    pub enum: 3 => ViaCode {
        AlternateAirport = "ALT",
        ApproachRoute = "APP",
        DesignatedAirway = "AWY",
        DirectToFix = "DIR",
        InitialFix = "INT",
        RouteViaFix = "RVF",
        RouteViaFixNotPermitted = "RNF",
        SID = "SID",
        SIDEnrouteTransition = "SDE",
        SIDRunwayTransition = "SDY",

        /// **Standard Terminal Arrival and Profile Descent**
        STAR = "STR",

        /// **Standard Terminal Arrival and Profile Descent - Enroute Transition**
        STAREnrouteTransition = "STE",

        /// **Standard Terminal Arrival and Profile Descent - Runway Transition**
        STARRunwayTransition = "STY",

        None = "   ",
    }
}
