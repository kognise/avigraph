use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Datum Code" field defines the Local Horizontal Reference Datum to
    which a geographical position, expressed in latitude and longitude,
    is associated.

    Local Horizontal Reference Datums will be derived from official government
    documentation. The "Datum Code" field will contain a three letter code
    corresponding to that government publication.
    */
    pub DatumCode: 3;
}
