use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    A standard cruising level table is established by ICAO and is to be observed except when, on
    the basis of regional air navigation agreements, a modified table of cruising levels is
    prescribed for use. This field permits the enroute airway record to identify the Cruise Table
    record that is to be used for cruise levels.

    Cruise Levels will be derived from official government sources. For the standard ICAO cruise
    table this field will contain the alpha characters "AA". For those countries not using the
    standard ICAO table and having a modified table this field will contain the alpha characters
    "BB," "CC," etc. If a country uses the standard ICAO table or a Modified table but indicates
    that an airway or portion of an airway is to be flown opposite of the cruise table, the field
    will contain alpha/numeric characters that identify the table to be used.

    *I do not fully understand this field, thus it is being parsed as a string instead of something
    else. Suggestions are welcome.*
    */
    pub CruiseTableIdentifier: 2;
}
