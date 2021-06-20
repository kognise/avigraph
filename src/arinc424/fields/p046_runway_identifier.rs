use crate::arinc424::utils::{parseable::Parseable, string_reader::StringReader};

/**
The "Runway Identifier" field identifies the runways described in runway records and
runways served by the ILS/MLS described in ILS/MLS records.

Runway identifiers are derived from official government sources and are shown in the
following format:

The two letters "RW" are followed by two numerics, 01 thru 36, and may contain a fifth
character designation of one of the following:


- C = Center (Runway of three parallel runways)
- L = Left (Runway of two or three parallel runways)
- R = Right (Run way of two or three parallel runways)
- T = (Runway and associated flight maneuvers referenced only in degrees true)

Any other designations (suffixes), such as North, South, East, West or STOL will not be
included in the ARINC file.
*/
#[derive(Debug)]
pub enum RunwayIdentifier {
    Center(String),
    Left(String),
    Right(String),
    True(String),
    None,
}

impl Parseable for RunwayIdentifier {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let numbers = reader.skip(2).read(2)?.to_owned();

        match reader.read(1)? {
            "C" => Some(Self::Center(numbers)),
            "L" => Some(Self::Left(numbers)),
            "R" => Some(Self::Right(numbers)),
            "T" => Some(Self::True(numbers)),
            " " => Some(Self::None),
            _ => None,
        }
    }
}
