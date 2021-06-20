use crate::num_parseable;

num_parseable! {
    /**
    The "Magnetic Bearing" for ILS localizer, MLS Azimuth, MLS Back Azimuth and Runway
    records is given in the primary record. This field allows the true bearing to be entered
    independently of the magnetic variation.

    True Bearings are entered into the field in degrees, tenths of a degree and hundredths of
    a degree, with the decimal point suppressed.
    */
    pub TrueBearing: 5, div 100.0;
}
