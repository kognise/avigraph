use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Marker Ident" field contains a unique computer ident assigned to each enroute
    marker.

    A unique identifier will be created for each enroute marker since such idents are not
    designated by official sources. Marker idents will be established using the 2-character
    ICAO code followed by two numeric digits assigned to keep markers unique within a given
    ICAO region.
    */
    pub MarkerIdent: 4;
}
