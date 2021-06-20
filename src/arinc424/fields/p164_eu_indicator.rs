use crate::enum_parseable;

enum_parseable! {
    /**
    The "EU Indicator" field is used to identify those Enroute Airway records that have
    an Airway Restriction record without identifying the restriction.

    The field will contain the alpha character "Y" when a restriction for the segment is
    contained in the restriction file or a blank when no restriction record exists.
    */
    pub enum EUIndicator {
        Yes = "Y",
        No = " ",
    }
}
