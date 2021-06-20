use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Fix Identifier" field contains the five-character-name-code, or other
    series of characters, with which the fix is identified. This includes Waypoint
    Identifiers, VHF Navaid Identifiers, NDB Navaid identifier, Airport Identifiers,
    and Runway Identifiers.

    Officially published identifiers or identifiers derived in accordance with
    Chapter 7, Naming Conventions, of this document.
    */
    pub FixIdentifier: 5;
}
