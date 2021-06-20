use crate::enum_parseable;

enum_parseable! {
    /**
    The "Radar" field indicates whether or not the communications unit has access to
    information derived from primary or secondary radar and can use that information in
    fulfilling their assigned tasks.

    The availability or radar capability will be drived from official government source
    documentation. If the communications unit has radar capabilities, the field will contain
    the character "R". If no capability exists, the field will contain the character "N".
    */
    pub enum Radar {
        HasRadar = "R",
        NoRadar = "N",
        None = " ",
    }
}
