use crate::num_parseable;

num_parseable! {
    /**
    The "Cycle Date" field identifies the calendar period in which the
    record was added to the file or last revised. A change in any ARINC
    424 field, except Dynamic Magnetic Variation, Frequency Protection,
    Continuation Record Number and File Record Number, requires a cycle
    date change. The cycle date will not change if there is no change in
    the data.

    The first two digits of the field contain the last two digits of the
    year in which the addition or revision was made. The last two digits
    contain the numeric identity of the 28-day data update cycle during
    which the change occurred. Each calendar year contains 13 such cycles,
    however, on rare occasions 14 cycles will be encountered.
    */
    pub Cycle u16: 4;
}
