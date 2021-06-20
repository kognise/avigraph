use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Airport Identifier" and the "Heliport Identifier" fields contain the identification
    of the airport or heliport to which the data contained in the record relates.

    The content of this field is derived from official government sources. It will be the
    four character ICAO Location Identifier of the airport or heliport when such is
    published. It will be the three or four character Domestic Identifier when published and
    no ICAO Location Identifier is available for the airport or heliport. When used on
    Airport or Heliport Flight Planning Continuation Records, it will be the Airport or
    Heliport Identifier owning the terminal controlled airspace referenced in that record.
    */
    pub AirportHeliportIdentifier: 4;
}
