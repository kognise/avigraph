use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
"Outbound Magnetic Course" is the published outbound magnetic course from the waypoint
identified in the record's "Fix Ident" field. In addition, this field is used for
Course/Heading/Radials on SID/STAR Approach Records through requirements of the Path
Terminator and coding rules contained in Attachment 5 of this specification.

Values from official government sources will be used when available. The field contains
magnetic information expressed in degrees and tenths of a degree, with the decimal point
suppressed. For route and procedure segments charted in "degrees true," the last character
(tenths position) of the field will contain the character "T".
*/
#[derive(Debug)]
pub enum OutboundMagneticCourse {
    Magnetic(f32),
    True(f32),
    None,
}

impl Parseable for OutboundMagneticCourse {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let course_data = reader.read(3)?;
        if course_data.trim().is_empty() {
            reader.skip(1);
            return Some(OutboundMagneticCourse::None);
        }
        let mut course: f32 = course_data.parse().ok()?;

        let true_or_tenths = reader.read(1)?;
        if true_or_tenths == "T" {
            Some(OutboundMagneticCourse::True(course))
        } else {
            course += true_or_tenths.parse::<f32>().ok()? / 10.0;
            Some(OutboundMagneticCourse::Magnetic(course))
        }
    }
}
