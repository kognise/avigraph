use crate::num_parseable;

num_parseable! {
    /**
    The "Glide Slope Beam Width" field specifies the glide path beam width of the Glide Slope
    defined in the record.

    Glide Slope beam widths from official government sources are entered into this field in
    degrees, tenths of a degree and hundredths of a degree with the decimal point suppressed.
    */
    pub GlideSlopeBeamWidth: 3, div 100.0;
}
