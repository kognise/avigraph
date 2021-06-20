use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Region Code" permits the categorization of waypoints and holding patterns
as either enroute or terminal area waypoints. In the latter case the terminal
area airport is identified in the field.

The field contains the alpha characters ENRT for enroute waypoints and airport
identification code (Airport Ident) for terminal waypoints. In the holding
pattern file, the content will match that of the holding fix, e.g. if the holding
fix is an enroute waypoint or enroute Navaid, the content with be ENRT; if the
holding fix is a terminal waypoint or terminal NDB, the content will be the
airport identification.
*/
#[derive(Debug)]
pub enum RegionCode {
    Enroute,
    Terminal(String),
}

impl Parseable for RegionCode {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let identifier = reader.read(4)?;
        if identifier == "ENRT" {
            Some(RegionCode::Enroute)
        } else {
            Some(RegionCode::Terminal(identifier.to_owned()))
        }
    }
}
