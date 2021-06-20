use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    **Terminal/Alternate Airport**

    This field has two uses depending on the "VIA" field and File Code for "To Fix." For
    "VIA" field content of "ALT" this field will contain the Alternate Airport Ident for this
    Company Route. If the file code for "To Fix" contains "P," this field will contain the
    Airport Ident for REGN CODE (Section 5.41) of Terminal Waypoints (PC records) and Runway
    (PG records).
    */
    pub TerminalAlternateAirport: 4;
}
