use crate::enum_parseable;

enum_parseable! {
    /**
    **ILS/MLS/GLS Category**

    The Localizer/MLS/GLS Performance Categories have established operating minimums and are
    listed as Category I, II, and III. The level of Performance Category does not imply that
    permission exists to use the facility for landing guidance to that level and does not
    limit minimal using designated classification. This field is also used to define the
    classification, non- ILS/MLS/GLS, and localizer installation such as IGS, LDA, or SDF. As
    used in the runway record, there are two fields, one labeled Localizer/MLS/GLS
    Category/Classification and the other labeled Second Localizer/MLS/GLS
    Category/Classification.

    The Localizer/MLS/GLS Category/Classification will be derived from official government
    sources.
    */
    pub enum ILSCategory {
        /// **ILS Localizer Only, No Glideslope**
        ILSNoGlideslope = "0",

        /// **ILS Localizer/MLS/GLS Category I**
        ILSCategoryI = "1",

        /// **ILS Localizer/MLS/GLS Category II**
        ILSCategoryII = "2",

        /// **ILS Localizer/MLS/GLS Category III**
        ILSCategoryIII = "3",

        /// **IGS Facility**
        IGS = "I",

        /// **LDA Facility with Glideslope**
        LDAGlideslope = "L",

        /// **LDA Facility, no Glideslope**
        LDANoGlideslope = "A",

        /// **SDF Facility with Glideslope**
        SDFGlideslope = "S",

        /// **SDF Facility, no Glideslope**
        SDFNoGlideslope = "F",

        None = " ",
    }
}
