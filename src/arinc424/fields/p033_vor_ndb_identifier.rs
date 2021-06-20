use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "VOR/NDB Identifier" field identifies the VHF/MF/LF facility defined
    in the record.

    The field contains the official government 1-, 2-, 3- and 4-character
    facility identification code.
    */
    pub VORNDBIdentifier: 4;
}
