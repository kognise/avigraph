use crate::arinc424::utils::{parseable::Parseable, string_reader::StringReader};

/**
This field is used in conjunction with Turn direction to indicate that a turn is required
prior to capturing the path defined in a terminal procedure.

The field contains the alpha character "Y" when a turn is required prior to beginning the
leg defined by the Path Term. The direction of the turn is specified in Section 5.20.
*/
#[derive(Debug)]
pub enum TurnDirectionValid {
    TurnRequired,
    TurnNotRequired,
}

impl Parseable for TurnDirectionValid {
    fn parse(reader: &mut StringReader) -> Option<Self> {
        if reader.read(1)? == "Y" {
            Some(TurnDirectionValid::TurnRequired)
        } else {
            Some(TurnDirectionValid::TurnNotRequired)
        }
    }
}
