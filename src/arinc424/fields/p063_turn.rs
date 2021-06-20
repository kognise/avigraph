use crate::enum_parseable;

enum_parseable! {
    /**
    The "Turn" field specifies the direction in which holding pattern turns are to be made.

    The "Turn" field will always contain either L or R.
    */
    pub enum Turn {
        Left = "L",
        Right = "R",
    }
}
