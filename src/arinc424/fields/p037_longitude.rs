use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The Longitude field contains the longitude of the geographic position
of the navigational feature identified in the record.

Geographic positions whose longitudes must be included in the data base
are defined during route design, many of them in official government
publications. The field is constructed as follows: The first character
position will contain the alpha character "E" or "W," indicating whether
the longitude is east or west of the prime (zero degree) meridian. For
longitudes falling on the 0 or 180 degree meridians, "E" is entered. The
following nine numeric characters define the longitude in degrees, minutes,
seconds, tenths of seconds and hundredths of seconds. Degree, minute and
second symbols and the decimal point are suppressed.
*/
#[derive(Default, Debug)]
pub struct Longitude(f32);

impl Parseable for Longitude {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(10)?;
        if data.trim().is_empty() {
            return Some(Self(0.0));
        }

        let sign = match data.get(..1)? {
            "E" => Some(1.0),
            "W" => Some(-1.0),
            _ => None,
        }?;

        let mut value = 0.0;
        value += data.get(1..4)?.to_owned().parse::<f32>().ok()?;
        value += data.get(4..6)?.to_owned().parse::<f32>().ok()? / 60.0;
        value += data.get(6..10)?.to_owned().parse::<f32>().ok()? / 360000.0;
        value *= sign;

        Some(Self(value))
    }
}
