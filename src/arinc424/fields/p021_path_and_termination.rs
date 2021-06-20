use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The Path and Termination defines the path geometry for a single record of an ATC terminal
    procedure.

    Attachment 5 to this document, "Path and Terminator," contains the various Path Term
    codes available for coding an ATC terminal procedure.

    *This field is very complex to parse and has a lot of variants, and often the parsed
    value won't be required, so I opted to simply leave this as a string for now.*
    */
    pub PathAndTermination: 2;
}
