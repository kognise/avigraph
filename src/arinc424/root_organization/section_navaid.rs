use crate::enum_parseable;

enum_parseable! {
    pub enum NavaidSubsection {
        VHFNavaid = " ",
        NDBNavaid = "B",
    }
}
