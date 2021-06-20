use crate::num_parseable;

num_parseable! {
    /**
    The "Longest Runway" field permits airport to be classified on the basis of the longest
    operational hard-surface runway.

    The longest runway will be derived from official government sources and entered in the
    field in hundreds of feet. This value will represent the longest hard-surfaced
    operational runway available without restriction at the airport. The value reflects
    overall pavement length declared suitable and available for the ground operations of
    aircraft. Where no hard-surfaced runway is available or those available do not meet
    criteria, the value will represent the longest operational runway at the airport.
    */
    pub LongestRunway u16: 3, mul 100;
}
