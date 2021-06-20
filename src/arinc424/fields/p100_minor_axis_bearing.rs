use crate::num_parseable;

num_parseable! {
    /**
    The "Minor Axis Bearing" field indicates the true bearing of the minor axis of marker
    beacons.

    This field will contain the true bearing in degrees and tenths of a degree, with the
    decimal point suppressed.
    */
    pub MinorAxisBearing: 4, div 10.0;
}
