use crate::enum_parseable;

enum_parseable! {
    /**
    The "Direction Restriction" field, when used on Enroute Airway records, will indicate the
    direction an Enroute Airway is to be flown. The "Direction Restriction" field, when used on
    Preferred Route records, will indicate whether the routing is available only in the
    direction of "from initial fix to terminus fix" or in both directions.

    Direction Restrictions should be derived from official government sources.
    */
    pub enum DirectionalRestriction {
        /**
        Enroute Airway Records: One way in direction route is coded.

        Preferred Route Records: Uni-directional Preferred Route, usable only from Initial
        Fix to Terminus Fix.
        */
        Forward = "F",

        /**
        Enroute Airway Records: One way in opposite direction route is coded.

        Preferred Route Records: Bi-directional Preferred Route, usable from Initial Fix to
        Terminus Fix or from Terminus Fix to Initial Fix.
        */
        Backward = "B",

        NoRestrictions = " ",
    }
}
