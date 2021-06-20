use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Inbound Holding Course" field defines the inbound course to the holding
waypoint.

Inbound holding courses derived from official government sources are entered
into the field in degrees and tenths of a degree, with the decimal point
suppressed. For holding courses charted with true bearings, the last character
of this field contains a "T" in place of tenths of a degree.
*/
#[derive(Debug)]
pub enum InboundHoldingCourse {
    Magnetic(f32),
    True(f32),
}

impl Parseable for InboundHoldingCourse {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let mut course: f32 = reader.read(3)?.parse().ok()?;

        let true_or_tenths = reader.read(1)?;
        if true_or_tenths == "T" {
            Some(InboundHoldingCourse::True(course))
        } else {
            course += true_or_tenths.parse::<f32>().ok()? / 10.0;
            Some(InboundHoldingCourse::Magnetic(course))
        }
    }
}
