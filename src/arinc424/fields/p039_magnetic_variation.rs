use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Magnetic Variation" field specifies the angular difference between True North and Magnetic
North at the location defined in the record. "Dynamic Magnetic Variation" is a computer model
derived value and takes location and date into consideration. For the "Station Declination" used
in some record types, refer to Section 5.66.

Magnetic variations are obtained from official government data sources and other geographical magnetic
variation source. A number of different terms are used in government documentation that have specific
connotations for the information provided by that government. The most common is "Epoch Year Variation."
In theory, this is a value determined by a government agency once every five years and published for
general use. Along with Epoch Year Variation, some governments also publish an annual drift value. Data
suppliers do not include annual drift derived figures in their databases but rather stay with the Epoch
Year value. Another term encountered in source documentation is "Magnetic Variation of Record." This is
generally an Epoch Year value. The difference here is that the government authority has established the
value as valid for everything associated with a given location. For example, if a Magnetic Variation of
Record is established for an airport location, everything referenced to that airport will use the same
value. This is of interest as it means that Terminal Procedure design is also based on that value.

Obvious differences can occur between a database supplied, semi-static value, and a value derived
dynamically, either by the airborne systems or supplier ground systems. Dynamic Magnetic Variation,
contained in the VHF Navaid Continuation Record and Enroute/Terminal Waypoint Primary Records, is a
computed, earth model derived figure, and is updated dynamically on a schedule established by the data
base supplier. Position one of the field contains an alpha character taken from the table below followed by
the value of magnetic variation expressed in degrees and tenths of a degree, with the decimal point suppressed.
When the first column is coded with the character "T," the value will be all zeros.
*/
#[derive(Debug)]
pub struct MagneticVariation(f32);

impl Parseable for MagneticVariation {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(5)?;
        if data.trim().is_empty() {
            return Some(MagneticVariation(0.0));
        }

        match data.get(..1)? {
            "T" => Some(MagneticVariation(0.0)),
            "E" => Some(MagneticVariation(
                data.get(1..5)?.parse::<f32>().ok()? / 10.0,
            )),
            "W" => Some(MagneticVariation(
                -data.get(1..5)?.parse::<f32>().ok()? / 10.0,
            )),
            _ => None,
        }
    }
}
