use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    This field is used to identify the desired runway transition of the applicable SID or
    STAR. It is used to link directly to the SID/STAR procedure records depending on the
    Company Route record "VIA" field (Section 5.77) and whether or not the SID/STAR has
    explicit runway transitions.

    If the applicable SID/STAR has explicit runway transitions then this field uniquely
    identifies the desired runway transition. If no runway transition is desired, the field
    is blank. If the applicable SID/STAR does not have explicit runway transitions this field
    is always non-blank and exactly matches the "TRANS IDENT" field of the SID/STAR procedure
    records.

    *This field is very complex to parse and has a lot of variants, and often the parsed
    value won't be required, so I opted to simply leave this as a string for now.*
    */
    pub RunwayTransition: 5;
}
