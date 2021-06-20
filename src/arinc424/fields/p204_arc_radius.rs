use crate::num_parseable;

num_parseable! {
    /**
    The "ARC Radius" field is used to define the radius of a precision turn. In Terminal
    Procedures, this is the "Constant Radius To A Fix" Path and Termination, for "RF" Leg.
    In Holding Patterns, this is the turning radius, inbound to outbound leg, for RNP
    Holding. The ARC Radius field is also used to specify the turn radius of RNP holding
    patterns included in SID, STAR, and Approach Records as HA, HF, and HM legs.

    The content of the field will be derived from official source publications. It will be
    expressed in nautical miles, tenths, hundredths and thousandths of a nautical mile, with
    the decimal point suppressed. A conversion to feet of the resolution in nautical miles
    is equal to an accuracy of 6 feet.
    */
    pub ArcRadius: 6, div 1000.0;
}
