use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The Preferred Route "To Fix" field is used to terminate the route referenced in the
    SID/STAR/APCH/AWY field (Section 5.78), or terminate a "Direct" segment or start an
    "Initial" segment when no SID/STAR/APCH/AWY is referenced.

    For Preferred Route records, the field will contain Enroute Waypoint, Terminal Waypoint,
    VHF NAVAID, NDB NAVAID or Terminal NDB NAVID, Airport Identifier.

    *This is a split field, `ToFixCompanyRoute` contains the equivalent for company routes.*
    */
    pub ToFixPreferredRoute: 5;
}

trimmed_str_parseable! {
    /**
    The Company Route "To Fix" field is used to terminate the route referenced in the
    SID/STAR/APCH/AWY field (Section 5.78), or terminate a "Direct" segment or start an
    "Initial" segment when no SID/STAR/APCH/AWY is referenced.

    For Company Route records the field will contain Enroute Waypoint, Terminal Waypoint,
    VHF NAVAID, NDB NAVAID, Terminal NDB NAVAID, Airport or Runway Identifier. The customer
    will define where a particular route segment is to terminate.

    *This is a split field, `ToFixPreferredRoute` contains the equivalent for preferred
    routes.*
    */
    pub ToFixCompanyRoute: 6;
}
