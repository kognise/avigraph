use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
Inbound Magnetic Course" is the published inbound magnetic course to the waypoint
identified in the record's "Fix Ident" field in which it is employed.

The "HX" group of Path Terminator codes is used to provide racetrack type course
reversal flight paths. Government publications for these course reversal include
an "inbound magnetic bearing." The SID/STAR/Approach Procedures records do not
include a dedicated field for this inbound course. Instead, the information is
included in the "Outbound Magnetic Course" field of such records.

Values from official government sources will be used when available. The field
contains magnetic bearing in degrees and tenths of a degree, with the decimal point
suppressed. For routes charted with true courses, the last character of this field
will contain a "T" in place of tenths of a degree.
*/
#[derive(Debug)]
pub enum InboundMagneticCourse {
    Magnetic(f32),
    True(f32),
    None,
}

impl Parseable for InboundMagneticCourse {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let course_data = reader.read(3)?;
        if course_data.trim().is_empty() {
            reader.skip(1);
            return Some(InboundMagneticCourse::None);
        }
        let mut course: f32 = course_data.parse().ok()?;

        let true_or_tenths = reader.read(1)?;
        if true_or_tenths == "T" {
            Some(InboundMagneticCourse::True(course))
        } else {
            course += true_or_tenths.parse::<f32>().ok()? / 10.0;
            Some(InboundMagneticCourse::Magnetic(course))
        }
    }
}
