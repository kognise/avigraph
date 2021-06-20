use crate::enum_parseable;

enum_parseable! {
    /**
    The "High/Low" field indicates the power of the enroute marker.

    The field contains the power derived from official government sources. The character "L"
    indicates low power for use at low altitudes. The character "H" indicates high power for
    general use.
    */
    pub enum HighLow {
        LowPower = "L",
        HighPower = "H",
        None = " ",
    }
}
