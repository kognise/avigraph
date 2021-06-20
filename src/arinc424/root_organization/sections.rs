use crate::enum_parseable;

enum_parseable! {
    pub enum StandardSection {
        MORA = "A",
        Navaid = "D",
        Enroute = "E",
        Heliport = "H",
        Airport = "P",
    }
}
