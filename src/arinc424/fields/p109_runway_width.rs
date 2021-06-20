use crate::num_parseable;

num_parseable! {
    /**
    The width of the runway identified in the "Runway Identifier" field is specified in the
    "Runway Width" field.

    Runway widths derived from Official Government Sources are entered into the field in
    feet, with a resolution of one foot. For runways of variable width, the minimum width
    encountered over the runway length will be entered.
    */
    pub RunwayWidth u16: 3;
}
