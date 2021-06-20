use crate::enum_parseable;

enum_parseable! {
    /**
    The content of the "TDZE Location" field indicates whether the TDZ elevation was obtained
    from official government sources or from other sources.

    The field will contain a "T" for official source or a "L" if the landing threshold
    elevation is used, or an "A" if the airport elevation is used.
    */
    pub enum TDZELocation {
        OfficialSource = "T",
        LandingThreshold = "L",
        AirportElevation = "A",
        None = " ",
    }
}
