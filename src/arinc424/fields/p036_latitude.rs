use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Latitude" field contains the latitude of the navigational feature
identified in the record.

Geographic positions whose latitudes must be included in the data base
are defined during route design, many of them in official government
publications. The field is constructed as follows. The first character
position contains the alpha character "N" or "S" indicating whether the
latitude is north or south of the equator. "N" is entered for latitudes
falling on the equator. The following eight numeric characters define
the latitude in degrees, minutes, seconds, tenths of seconds and
hundredths of seconds. Degree, minute and second symbols and the decimal
point are suppressed.
*/
#[derive(Default, Debug)]
pub struct Latitude(f32);

impl Parseable for Latitude {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(9)?;
        if data.trim().is_empty() {
            return Some(Self(0.0));
        }

        let sign = match data.get(..1)? {
            "N" => Some(1.0),
            "S" => Some(-1.0),
            _ => None,
        }?;

        let mut value = 0.0;
        value += data.get(1..3)?.to_owned().parse::<f32>().ok()?;
        value += data.get(3..5)?.to_owned().parse::<f32>().ok()? / 60.0;
        value += data.get(5..9)?.to_owned().parse::<f32>().ok()? / 360000.0;

        value *= sign;
        Some(Self(value))
    }
}
