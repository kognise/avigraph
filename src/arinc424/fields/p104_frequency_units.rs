use crate::enum_parseable;

enum_parseable! {
    /**
    The "Frequency Units" field will designate the frequency spectrum area for the frequency
    in the "Communications Frequency" (Section 5.103) field as indicated in the table or will
    designate the content of the "Communications Frequency" field as a channel.
    */
    pub enum FrequencyUnits {
        HF = "H",
        VHF = "V",
        UHF = "U",
        Channel = "C",
        None = " ",
    }
}
