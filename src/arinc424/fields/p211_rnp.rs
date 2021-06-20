use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
Required Navigation Performance (RNP) is a statement of the Navigation Performance
necessary for operation within a defined airspace in accordance with ICAO Annex 15
and/or State published rules.

RNP values derived from official government source will be used when available. They
are entered into the field in nautical miles (two digits) with a zero or negative
exponent (one digit). The contents can be:

- When used on Enroute Airway segments, RNP shall apply inbound to the fix when viewed
  in increasing sequence number order. The RNP applies only to the airway leg on which
  it is specified. If no RNP values is coded on a segment, there is not a database
  specified RNP for that segment.

- When used on a SID, STAR and Approach Procedure records, the RNP shall apply to the
  segment on which it is coded. RNP will be coded on every segment where it is
  specified by source. Lack of a RNP value on a segment indicates no source supplied
  RNP value was available for that segment.

- When used on Holding Patterns, the RNP applies to the holding pattern as defined in
  the record.

The RNP concept will also be applied to defined airspaces, in addition to the specific
flight paths as defined above. ARINC 424-13 addresses an "airspace record" that includes
a reservation for RNP until actual content can be defined.

There are no provisions for "Vertical RNP" in ARINC 424 at this time.
*/
#[derive(Debug)]
pub struct RNP(f32);

impl Parseable for RNP {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(3)?;
        if data.trim().is_empty() {
            return Some(Self(0.0));
        }

        let base = data.get(..2)?.parse::<f32>().ok()?;
        let index = -data.get(2..3)?.parse::<i32>().ok()?;
        let power = base.powi(index);

        Some(RNP(power))
    }
}
