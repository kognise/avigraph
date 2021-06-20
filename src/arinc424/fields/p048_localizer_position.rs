use crate::num_parseable;

num_parseable! {
    /**
    The "Localizer/Azimuth Position" field defines the location of the facility antenna
    relative to one end of the runway.

    The field contains the official government source distance, in feet, from the antenna to
    the runway end. The resolution is one foot.
    */
    pub LocalizerPosition u16: 4;
}
