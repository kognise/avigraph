use crate::num_parseable;

num_parseable! {
    /**
    The "Leg Time" field specifies the length of the inbound leg of a holding
    pattern in units of time.

    Leg time, derived from official government sources, is entered into this
    field in minutes and tenths of a minute, with the decimal point suppressed.
    */
    pub LegTime: 2, div 10.0;
}
