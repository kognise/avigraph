use crate::arinc424::utils::parseable::Parseable;
use crate::arinc424::utils::string_reader::StringReader;

/**
The "Altitude/Minimum Altitude" field indicates the reference altitude associated with
(1) Enroute Airways (MEA, MFA or other minimum altitudes as defined by source), (2) holding
pattern path of Holding Pattern record, (3) altitudes at fixes in terminal procedures and
terminal procedure path termination defined by the Path Terminator in the Airport or Heliport
SID/STAR/Approach Record and (4) lowest altitude of the "blocked altitudes" for a Preferred
Route.

Reference altitudes are determined during route definition. The values are derived from
official government source when available. This specification includes specific rules for
altitude provision and when those altitudes are not provided by source documents, they will
be included by data suppliers according to those rules. The field may contain altitudes
(all numeric) or flight level (alpha/numeric). The allnumeric fields will contain altitudes
in feet with a resolution of one foot. The alpha/numeric fields will contain the alpha
characters "FL" followed by the altitude expressed in hundreds of feet (three digits) or a
code as indicated below.

On Airport and Heliport SID, STAR and Approach Route records, the first "Altitude" field will
contain an altitude when "Altitude Description" field contains a plus (+), a minus (-), or one
of the following characters: "B," "G," "H," or "V." The first "Altitude" field may contain an
altitude when the "Altitude Description" field contains a "I" or "J" or when it is blank. The
second "Altitude" field will contain an altitude when the "Altitude Description" field contains
one of the following characters: "B," "C," "G," "H," "I," "J," or "V." In approach procedure coding,
some fix "Altitudes" may be below sea level, specifically altitudes at runway fixes when the runway
threshold elevation is below sea level. In these cases, the "Altitude" will be expressed in feet
with a minus (-) sign in the first character of the five-character field, see examples.

On Airport and Heliport SID/STAR/Approach Continuation Records, the "Localizer Only Altitude"
field will contain an altitude when there is a non-precision altitude at the fix that is associated
with using the ILS procedure as a Localizer Only (Glide Slope out) procedure. Such an altitude may
be provided for the Final Approach Fix (FAF) or any step-down fix from the FACF inbound on the final
approach course.

On Enroute Airway records, the first "Minimum Altitude" field will contain the MEA or MFA if the
altitude is the same for both directions of flight and the second "minimum Altitude" will be blank.
If the airway segment has directional MEAs/MFAs, the first "Minimum Altitude" field will contain the
value for the direction of flight in which the airway is coded and the second "Minimum Altitude" field
will contain the value for the opposite direction of flight. The first "Minimum Altitude" field may
contain the alpha characters UNKNN when the MEA/MFA is unknown or the alpha characters NESTB when the
MEA/MFA has not been established by the appropriate authority.

On Preferred Routes, the "Minimum Altitude" and the "Maximum Altitude" apply to the entire route and are
a minimum and maximum block. Altitude 1 and Altitude 2 are fix related apply only to the fix in the
sequence in which they occur and are defined by the Altitude Description field.
*/
#[derive(Debug)]
pub enum AltitudeOrMinimumAltitude {
    Feet(i32),
    FlightLevel(u16),
    Unknown,
    NotEstablished,
    None,
}

impl Parseable for AltitudeOrMinimumAltitude {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        let data = reader.read(5)?;

        if data == "UNKNN" {
            Some(AltitudeOrMinimumAltitude::Unknown)
        } else if data == "NESTB" {
            Some(AltitudeOrMinimumAltitude::NotEstablished)
        } else if data.starts_with("FL") {
            let flight_level: u16 = data.get(2..)?.parse().ok()?;
            Some(AltitudeOrMinimumAltitude::FlightLevel(flight_level))
        } else if data.trim().is_empty() {
            Some(AltitudeOrMinimumAltitude::None)
        } else {
            let feet: i32 = data.parse().ok()?;
            Some(AltitudeOrMinimumAltitude::Feet(feet))
        }
    }
}
