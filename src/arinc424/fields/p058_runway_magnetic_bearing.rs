use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The magnetic bearing of the runway identified in the "runway identifier" field of
the record is specified in the "Runway Magnetic Bearing" field.

Runway magnetic bearings derived from official government sources are entered into
the field in degrees and tenths of a degree, with the decimal point suppressed. For
runway bearings charted with true bearings, the last character of this field will
contain a "T" in place of tenths of a degree.
*/
#[derive(Debug)]
pub enum RunwayMagneticBearing {
    Magnetic(f32),
    True(f32),
}

impl Parseable for RunwayMagneticBearing {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let mut course: f32 = reader.read(3)?.parse().ok()?;

        let true_or_tenths = reader.read(1)?;
        if true_or_tenths == "T" {
            Some(RunwayMagneticBearing::True(course))
        } else {
            course += true_or_tenths.parse::<f32>().ok()? / 10.0;
            Some(RunwayMagneticBearing::Magnetic(course))
        }
    }
}
