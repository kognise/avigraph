use crate::num_parseable;

num_parseable! {
    /**
    The elevation of the landing threshold of the runway described in a runway record is
    defined in the "Landing Threshold Elevation" field.

    Runway landing threshold elevations derived from official government sources are entered
    into this field in feet, to a resolution of 1 foot. For elevations above MSL, the field
    contains the numeric characters of the elevation only. For below MSL elevations, the
    first character of the field is a minus (-) sign.
    */
    pub LandingThresholdElevation i32: 5;
}
