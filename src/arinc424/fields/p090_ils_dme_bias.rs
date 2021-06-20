use crate::num_parseable;

num_parseable! {
    /**
    This field is used to specify the DME offset.

    The field contains a 2-digit bias term in nautical miles and tenths
    of a nautical mile with the decimal point suppressed. Field is blank
    for unbiased DME's.
    */
    pub ILSDMEBias: 2, div 10.0;
}
