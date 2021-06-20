use crate::enum_parseable;

enum_parseable! {
    /**
    The content of the source field indicates whether the "True Bearing" is derived from
    official government sources or from other sources.

    The field contains "Y" when the "True Bearing" is derived from official government
    sources and "N" when it is derived from other sources.
    */
    pub enum GovernmentSource {
        Yes = "Y",
        OtherSource = "N",
        None = " ",
    }
}
