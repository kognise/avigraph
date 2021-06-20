use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    This field is used to identify the desired enroute transition of the applicable SID or
    STAR. It can also be used to identify the desired approach transition of an approach.

    *This field is very complex to parse and has a lot of variants, and often the parsed
    value won't be required, so I opted to simply leave this as a string for now.*
    */
    pub EnrouteTransition: 5;
}
