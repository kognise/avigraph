use crate::num_parseable;

num_parseable! {
    /**
    The "Glide Slope Angle" field defines the glide slope angle of an ILS facility/GLS
    approach. The "Minimum Elevation Angle" field defines the lowest elevation angle
    authorized for the MLS procedure.

    Glide Slope and Elevation angles from official government sources are entered into the
    fields in degrees, tenths of a degree and hundredths of a degree with the decimal point
    suppressed.
    */
    pub GlideSlopeAngle: 3, div 100.0;
}
