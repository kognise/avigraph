use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    **Localizer/MLS/GLS Identifier**

    The "Localizer/MLS/GLS Identifier" field identifies the localizer, MLS facility or GLS
    Ref Path defined in the record. In the Runway Record, two "Landing Systems" may be
    defined.

    The field contains the identification code of the Localizer or MLS facility or GLS
    Reference Path derived from official government sources. In the Runway Record, there are
    two fields labeled Localizer/MLS/GLS Reference Path identifier and second Localizer/MLS/
    GLS Reference Path identifier to encode multiple Localizers, such as an ILS and a LDA
    associated with a single runway.
    */
    pub LocalizerIdentifier: 4;
}
