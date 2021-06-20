use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    **Notes (Continuation Records**

    The "Notes" field (continuation record) is provided to accommodate any information that
    cannot be entered in the primary record.

    Appropriate contents for the field will be determined at the time the primary record is
    assembled.
    */
    pub Notes: 70;
}
