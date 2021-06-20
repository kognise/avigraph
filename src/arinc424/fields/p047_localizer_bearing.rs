use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Localizer Bearing" field defines the magnetic bearing of the localizer course of the
ILS facility/GLS approach described in the record.

Localizer courses, derived from official government sources, are entered into the field
in degrees and tenths of a degree, with the decimal point suppressed. For localizer
courses charted with true courses, the last character of this field will contain a "T" in
place of tenths of a degree.
*/
#[derive(Debug)]
pub enum LocalizerBearing {
    Magnetic(f32),
    True(f32),
}

impl Parseable for LocalizerBearing {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let mut course: f32 = reader.read(3)?.parse().ok()?;

        let true_or_tenths = reader.read(1)?;
        if true_or_tenths == "T" {
            Some(LocalizerBearing::True(course))
        } else {
            course += true_or_tenths.parse::<f32>().ok()? / 10.0;
            Some(LocalizerBearing::Magnetic(course))
        }
    }
}
