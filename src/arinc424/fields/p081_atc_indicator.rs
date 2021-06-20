use crate::enum_parseable;

enum_parseable! {
    /**
    The "ATC Indicator" field will be used to indicate that the altitudes shown in the
    altitude fields can be modified by ATC or the altitude will be assigned by ATC.
    */
    pub enum ATCIndicator {
        /**
        This field will contain the alpha character "A" when the official government source
        states that the altitude can be modified or assigned by ATC.
        */
        CanBeAssigned = "A",

        /**
        This field will contain the
        alpha character "S" when the official government source states that the altitude will be
        assigned by ATC or if no altitude is supplied.
        */
        WillBeAssigned = "S",

        None = " ",
    }
}
