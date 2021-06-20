use crate::num_parseable;

num_parseable! {
    /**
    Indicates that a specific turn radius from the inbound course to the outbound course is
    required by the airspace controlling agency.

    When a fix radius turn is required a 3 digit numeric value will be entered in this field
    representing the radius of the turn to 1 decimal place (tenths, decimal point suppressed)
    in nautical miles. A blank entry in this field indicates that no fixed radius transition
    is required.
    */
    pub FixedRadiusTransitionIndicator: 3, div 10.0;
}
