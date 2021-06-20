use crate::num_parseable;

num_parseable! {
    /**
    The "Frequency Protection Distance" field provides an indication of the
    distance to the next nearest Navaid on the same frequency.

    The distance to the next Navaid will be computer generated values. Values
    will be entered on Navaid with DME or TACAN equipped facilities only and
    will indicate the distance, in nautical miles, to the next nearest DME or
    TACAN equipped facility. Maximum relevant value will be 600 nautical miles.
    */
    pub FrequencyProtectionDistance u16: 3;
}
