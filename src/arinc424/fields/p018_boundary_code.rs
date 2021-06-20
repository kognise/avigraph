use crate::enum_parseable;

enum_parseable! {
    /**
    Routes of flight frequently cross geographical boundaries. The "Boundary Code" field
    identifies the area into, or from which a continuous route passes when such a crossing
    occurs.
    */
    pub enum BoundaryCode {
        USA = "U",
        CanadaAndAlaska = "C",
        Pacific = "P",
        LatinAmerica = "L",
        SouthAmerica = "S",
        SouthPacific = "1",
        Europe = "E",
        EasternEurope = "2",
        MiddleEastSouthAsia = "M",
        Africa = "A",
        None = " ",
    }
}
