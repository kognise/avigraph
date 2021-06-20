use crate::num_parseable;

num_parseable! {
    /**
    The "VOR/NDB Frequency" field specifies the frequency of the Navaid
    identified in the "VOR/NDB Identifier" field of the record.

    Frequencies are derived from official government sources. VHF Navaid
    frequencies contain characters for hundreds, tens, units, tenths and
    hundredths of megahertz. The decimal point following the unit entry is
    suppressed.

    *This is a split field, `NDBFrequency` contains the equivalent for NDBs.*
    */
    pub VORFrequency: 5, div 100.0;
}

num_parseable! {
    /**
    The "VOR/NDB Frequency" field specifies the frequency of the Navaid
    identified in the "VOR/NDB Identifier" field of the record.

    Frequencies are derived from official government sources. NDB Navaid
    frequencies contain characters for thousands, hundreds, tens, units
    and tenths of kilohertz. The decimal point following the unit entry
    is suppressed.

    *This is a split field, `VORFrequency` contains the equivalent for VORs.*
    */
    pub NDBFrequency: 5, div 1000.0;
}
