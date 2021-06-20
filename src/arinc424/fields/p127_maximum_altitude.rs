use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Maximum Altitude" field is used to indicate the maximum altitude allowed.

Maximum altitudes should be derived from official government publications
describing the upper limit of the airway in feet or flight level.
*/
#[derive(Debug)]
pub enum MaximumAltitude {
    Feet(i32),
    FlightLevel(u16),
    Unlimited,
    None,
}

impl Parseable for MaximumAltitude {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(5)?;

        if data == "UNLTD" {
            Some(MaximumAltitude::Unlimited)
        } else if data.starts_with("FL") {
            let flight_level = data.get(2..)?.parse().ok()?;
            Some(MaximumAltitude::FlightLevel(flight_level))
        } else if data.trim().is_empty() {
            Some(MaximumAltitude::None)
        } else {
            let feet = data.parse().ok()?;
            Some(MaximumAltitude::Feet(feet))
        }
    }
}
