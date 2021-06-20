use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    "The SID/STAR Route Identifier" field contains the name of the SID or STAR, using the
    basic indicator, validity indicator and route indicator abbreviated to six characters
    with the naming rules in Chapter 7 of this document.

    SID/STAR route identifier codes should be derived from official government publications
    describing the terminal procedures structure.
    */
    pub SIDSTARRouteIdentifier: 6;
}
