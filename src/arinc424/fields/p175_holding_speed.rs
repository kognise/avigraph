use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Holding Speed" will be the maximum speed in a holding pattern.

The speed limit will be derived from official government sources. If the
value is different from the limit given with ICAO rules, it will be shown
in knots, else the field will be blank.
*/
#[derive(Debug)]
pub enum HoldingSpeed {
    Standard,
    Nonstandard(u16),
}

impl Parseable for HoldingSpeed {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(3)?;

        if data.trim().is_empty() {
            Some(HoldingSpeed::Standard)
        } else {
            let knots = data.parse().ok()?;
            Some(HoldingSpeed::Nonstandard(knots))
        }
    }
}
