use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
"Speed Limit Altitude" is the altitude below which speed limits may be imposed.

The "Speed Limit Altitude" will be derived from official government sources in feet MSL
or FL's.
*/
#[derive(Debug)]
pub enum SpeedLimitAltitude {
    Feet(i32),
    FlightLevel(u16),
    None,
}

impl Parseable for SpeedLimitAltitude {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(5)?;

        if data.starts_with("FL") {
            let flight_level = data.get(2..)?.parse().ok()?;
            Some(SpeedLimitAltitude::FlightLevel(flight_level))
        } else if data.trim().is_empty() {
            Some(SpeedLimitAltitude::None)
        } else {
            let feet = data.parse().ok()?;
            Some(SpeedLimitAltitude::Feet(feet))
        }
    }
}
