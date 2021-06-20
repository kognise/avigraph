use crate::enum_parseable;

enum_parseable! {
    /**
    The "IFR Capability" field indicates if the Airport/Heliport has any published Instrument
    Approach Procedures.

    The field contains "Y" if there is an Official Government Instrument Approach Procedure
    published, otherwise the field will contain "N". (Note: The presence of "Y" in this field
    does not necessarily imply that the published instrument approach is coded in the data
    base.)
    */
    pub enum IFRCapability {
        PublishedApproach = "Y",
        NoPublishedApproach = "N",
        None = " ",
    }
}
