use crate::num_parseable;

num_parseable! {
    /**
    This field is used to supply the distance in nautical miles from the "To Airport/Fix" to
    the "ALT ARPT".

    Values for this field will be supplied by the customer and must be equal to or greater
    than the great circle distance from the destination airport/fix to the alternate airport.
    */
    pub AlternateDistance u16: 4;
}
