use crate::num_parseable;

num_parseable! {
    /**
    "RHO" is defined as the geodesic distance in nautical miles to the waypoint identified in the
    record's "Fix Ident" field from the NAVAID in the "Recommended NAVAID" field.

    Rho values derived from official government sources will be used when available. They are
    entered into the field in nautical miles and tenths of a nautical mile, with the decimal point
    suppressed. The content is controlled through requirements of the Path Terminator and coding
    rules contained in Attachment 5 of this specification.
    */
    pub Rho: 4, div 10.0;
}
