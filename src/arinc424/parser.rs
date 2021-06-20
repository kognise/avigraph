use super::records::*;
use super::root_organization::*;
use super::utils::parseable::Parseable;
use super::utils::string_reader::StringReader;

/**
Macro which will be removed later in production, allows cleaner one-line
discards in match statements to increase readability.

For example, the difference between these:

```rust
Case => {
    Thing::parse(reader).unwrap();
}
```

Versus:

```rust
Case => trash!(Thing::parse(reader).unwrap())
```

Doesn't look like much but I promise it helps.
*/
macro_rules! trash {
    ($stuff:expr) => {{
        $stuff;
    }};
}

pub struct Parser {
    data: String,
}

impl Parser {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn parse(&mut self) -> Option<()> {
        for line in self.data.lines() {
            self.parse_line(line);
        }

        Some(())
    }

    fn parse_line(&self, line: &str) -> Option<()> {
        let ref mut reader = StringReader::new(line);

        match RecordType::parse(reader)? {
            // Standard
            RecordType::Standard => {
                let _area_code = reader.read(3)?.to_owned();

                use StandardSection::*;
                match StandardSection::parse(reader)? {
                    Navaid => match NavaidSubsection::parse(reader)? {
                        NavaidSubsection::VHFNavaid => trash!(VHFNavaid::parse(reader).unwrap()),
                        NavaidSubsection::NDBNavaid => trash!(NDBNavaid::parse(reader).unwrap()),
                    },

                    Enroute => match EnrouteSubsection::parse(reader)? {
                        EnrouteSubsection::Waypoint => trash!(Waypoint::parse(reader).unwrap()),
                        EnrouteSubsection::HoldingPattern => {
                            trash!(HoldingPattern::parse(reader).unwrap())
                        }
                        EnrouteSubsection::AirwaysAndRoutes => {
                            trash!(EnrouteAirway::parse(reader).unwrap())
                        }
                        _ => (),
                    },

                    Airport => match AirportSubsection::parse(reader)? {
                        AirportSubsection::TerminalNDBs => {
                            trash!(NDBNavaid::parse(reader).unwrap())
                        }
                        AirportSubsection::TerminalWaypoints => {
                            println!("[root airport waypoint, if you see this note please notify the developer]");
                            trash!(Waypoint::parse(reader).unwrap())
                        }

                        // Some odd things like terminal waypoints use a different field
                        _ => match AirportSubsection::parse_from(reader.peek_after(6, 1)?)? {
                            AirportSubsection::TerminalWaypoints => {
                                trash!(Waypoint::parse(reader).unwrap())
                            }
                            _ => (),
                        },
                    },

                    // Very similar in many aspects to Airport.
                    Heliport => match HeliportSubsection::parse(reader)? {
                        HeliportSubsection::TerminalWaypoints => {
                            println!("[root heliport waypoint, if you see this note please notify the developer]");
                            trash!(Waypoint::parse(reader).unwrap())
                        }

                        _ => match HeliportSubsection::parse_from(reader.peek_after(6, 1)?)? {
                            HeliportSubsection::TerminalWaypoints => {
                                trash!(Waypoint::parse(reader).unwrap())
                            }
                            _ => (),
                        },
                    },

                    MORA => (),
                }
            }
        }

        Some(())
    }
}
