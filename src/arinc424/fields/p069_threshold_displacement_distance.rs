use crate::num_parseable;

num_parseable! {
    /**
    The distance from the extremity of a runway to a threshold not located at that extremity
    of that runway.

    Threshold displacement distances derived from official government sources are entered
    into this field in feet.
    */
    pub ThresholdDisplacementDistance u16: 4;
}
