use crate::num_parseable;

num_parseable! {
    /**
    "Theta" is defined as the magnetic bearing to the waypoint identified in the record's "FIX
    Ident" field from the Navaid in the "Recommended Navaid" field.

    Theta values are derived from official government sources when available. They are provided
    in degrees and tenths of a degree, with the decimal point suppressed. The content is
    controlled through requirements of the Path Terminator and coding rules contained in
    Attachment 5 of the specification.
    */
    pub Theta: 4, div 10.0;
}
