use crate::enum_parseable;

enum_parseable! {
    /**
    The Level field defines the airway structure of which the record is an element.
    */
    pub enum Level {
        AllAltitudes = "B",
        HighLevel = "H",
        LowLevel = "L",
        None = " ",
    }
}
