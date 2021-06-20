use crate::num_parseable;

num_parseable! {
    /**
    The "Leg Length" field specifies the distance between the point at which the
    aircraft rolls out on the inbound leg of the holding pattern and the fix at
    which the holding pattern is defined.

    Leg length derived from official government sources is entered into the field
    in nautical miles and tenths of a nautical mile, with the decimal point
    suppressed.
    */
    pub LegLength: 3, div 10.0;
}
