use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "ICAO Code" field permits records to be categorized geographically
    within the limits of the categorization performed by the "Area Code" field.

    The code is to be employed in the ICAO code field may be found in ICAO Document
    No. 7910, "Location Indicators." In order to permit sub-division of the United
    States into more easily manageable regions, the ICAO code for the USA (K) is
    followed by a numeric character.
    */
    pub ICAOCode: 2;
}
