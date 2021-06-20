use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
For VHF NavaidS, the "Station Declination" field contains the angular
difference between true north and the zero degree radial of the Navaid
at the time the Navaid was last site checked. For ILS localizers, the
field contains the angular difference between true north and magnetic
north at the localizer antenna site at the time the magnetic bearing of
the localizer course was established.

Station declinations are derived from official government sources. The
field contains one of the alpha characters shown in the following table
followed by the value of the declination in degrees and tenths of a degree,
with the decimal point suppressed. When the first column of the Station
Declination field is coded T or G, the remainder of the field should be
coded all zeros.

The appearance of the character "G" in column 1 of this field will alert
users that although a Navaid declination may not be zero, the fact that
the grid reference is unknown prevents a value from being defined.
*/
#[derive(Debug)]
pub enum StationDeclination {
    TrueNorth(f32),
    GridNorth,
}

impl Parseable for StationDeclination {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(5)?;
        if data.trim().is_empty() {
            return Some(StationDeclination::TrueNorth(0.0));
        }

        match data.get(..1)? {
            "G" => Some(StationDeclination::GridNorth),
            "T" => Some(StationDeclination::TrueNorth(0.0)),
            "E" => Some(StationDeclination::TrueNorth(
                data.get(1..5)?.parse::<f32>().ok()? / 10.0,
            )),
            "W" => Some(StationDeclination::TrueNorth(
                -data.get(1..5)?.parse::<f32>().ok()? / 10.0,
            )),
            _ => None,
        }
    }
}
