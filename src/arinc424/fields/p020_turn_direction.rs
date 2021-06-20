use crate::enum_parseable;

enum_parseable! {
    /**
    The "Turn Direction" field specifies the direction in which Terminal Procedure turns are
    to be made. It is also used to indication direction on course reversals, see Attachment 5
    Path and Termination.

    The field contains the alpha character "L" for Left turns, "R" for Right turns and "E"
    for turns in either direction.
    */
    pub enum TurnDirection {
        Left = "L",
        Right = "R",
        Either = "E",
        None = " ",
    }
}
