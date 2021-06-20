use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    **ATA/IATA Designator**

    The "ATA/IATA" field contains the Airport/Heliport ATA/IATA designator code to which the
    data contained in the record relates.

    The content of this field should be derived from IATA Reservations Manual Part II, IATA
    Resolution 763/Location Identifiers.
    */
    pub IATADesignator: 3;
}
