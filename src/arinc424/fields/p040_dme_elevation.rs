use crate::num_parseable;

num_parseable! {
    /**
    The "DME Elevation" field defines the elevation of the DME component of the Navaid
    described in the record.

    DME elevations specified in official government publications are entered into this
    field in feet with respect to MSL. When the elevation is below MSL, the first column
    of the field contains a minus (-) sign.
    */
    pub DMEElevation i32: 5;
}
