use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Route Identifier" field identifies a route of flight or traffic orientation,
    using the coding employed on aeronautical navigation charts and related publications.

    For Enroute Airways, Route Identifier codes should be derived from official government
    ARINC SPECIFICATION 424 - Page 60 5.0 NAVIGATION DATA - FIELD DEFINITIONS publications.

    For North American Routes for North Atlantic Traffic, "Common Portion" and other similar
    route system, route identifier code shall be those published in government sources. For
    the European Traffic Orientation System or other similar route systems such as North
    American Routes for North Atlantic Traffic, "Non-common Portion," Preferred Routes and
    Preferential Routes published without official and/or flight plan identifiers, but
    published as between specific airports or other navigation fixes, route identifiers define
    the initial fix and the terminus fix idents according to the naming rules in Chapter 7.

    For routings which do not include a unique initial or terminus fix, rules on creating unique
    Route Identifiers are also contained in Chapter 7. Those rules have been developed with use
    of the Geographical Reference Tables (TG). Refer to Chapter 3, Section 3.2.7.2 and Chapter 4,
    Section 4.1.26 for more detail.

    *This is a split field, `PreferredRouteIdentifier` contains the equivalent for preferred routes.*
    */
    pub EnrouteAirwayRouteIdentifier: 5;
}

trimmed_str_parseable! {
    /**
    The "Route Identifier" field identifies a route of flight or traffic orientation,
    using the coding employed on aeronautical navigation charts and related publications.

    For Preferred Routes, Route Identifiers may or may not be provided in government
    publications. Where they are available, they will be used.

    For North American Routes for North Atlantic Traffic, "Common Portion" and other similar
    route system, route identifier code shall be those published in government sources. For
    the European Traffic Orientation System or other similar route systems such as North
    American Routes for North Atlantic Traffic, "Non-common Portion," Preferred Routes and
    Preferential Routes published without official and/or flight plan identifiers, but
    published as between specific airports or other navigation fixes, route identifiers define
    the initial fix and the terminus fix idents according to the naming rules in Chapter 7.

    For routings which do not include a unique initial or terminus fix, rules on creating unique
    Route Identifiers are also contained in Chapter 7. Those rules have been developed with use
    of the Geographical Reference Tables (TG). Refer to Chapter 3, Section 3.2.7.2 and Chapter 4,
    Section 4.1.26 for more detail.

    *This is a split field, `EnrouteAirwayRouteIdentifier` contains the equivalent for enroute airways.*
    */
    pub PreferredRouteIdentifier: 10;
}
