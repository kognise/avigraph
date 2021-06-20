use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
**Route Distance From, Holding Distance/Time**

In Enroute Airways, "Route Distance From" is the distance in nautical miles from the
waypoint identified in the record's "Fix Ident" field to the next waypoint of the route. In
SID, STAR and Approach Procedure records, the field may contain segment distances/along
track distances/excursion distances/DME distances. The actual content is dependent on the
Pathand Termination.

The field contains distances, from official government sources where available, expressed
in nautical miles and tenths of nautical miles with the decimal point suppressed. For
Holding Pattern Records and/or Path and Terminations defining holdings patterns, content
may be "holding time" expressed in minutes and tenths of minutes preceded by the character
"T" with the decimal point suppressed.
*/
#[derive(Debug)]
pub enum RouteHoldingDistanceTime {
    Miles(f32),
    Minutes(f32),
    None,
}

impl Parseable for RouteHoldingDistanceTime {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let distance_or_time = reader.peek(1)?;

        if distance_or_time == "T" {
            let minutes = reader.skip(1).read(3)?.parse::<f32>().ok()? / 10.0;
            Some(RouteHoldingDistanceTime::Minutes(minutes))
        } else if distance_or_time == " " {
            reader.skip(4);
            Some(RouteHoldingDistanceTime::None)
        } else {
            let miles = reader.read(4)?.parse::<f32>().ok()? / 10.0;
            Some(RouteHoldingDistanceTime::Miles(miles))
        }
    }
}
