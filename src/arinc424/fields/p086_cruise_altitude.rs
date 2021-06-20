use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
This field will be used to establish an Enroute Cruise Altitude. It will be entered on
Company Route records as specified by the customer.

The customer will supply the Cruise Attitude in feet or flight level.
*/
#[derive(Debug)]
pub enum CruiseAltitude {
    Feet(i32),
    FlightLevel(u16),
    None,
}

impl Parseable for CruiseAltitude {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(5)?;

        if data.starts_with("FL") {
            let flight_level = data.get(2..)?.parse().ok()?;
            Some(CruiseAltitude::FlightLevel(flight_level))
        } else if data.trim().is_empty() {
            Some(CruiseAltitude::None)
        } else {
            let feet = data.parse().ok()?;
            Some(CruiseAltitude::Feet(feet))
        }
    }
}
