use crate::num_parseable;

num_parseable! {
    /**
    **Glide Slope/Elevation Position**

    The "Glide Slope/Elevation Position" field defines the location of the antenna with
    respect to the approach end of the runway.

    The field contains four numeric characters indicating the distance in feet (to a
    resolution of one foot) from a line drawn at right angles to the runway at the antenna
    position to the threshold of the runway.
    */
    pub GlideSlopePosition u16: 4;
}
