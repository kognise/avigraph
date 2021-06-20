use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    If required, additional information concerning a runway can be included in a record in
    the "Runway Description" field.

    Appropriate contents for the field will be determined when the record is assembled.
    */
    pub RunwayDescription: 22;
}
