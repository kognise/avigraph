use crate::num_parseable;

num_parseable! {
    /**
    The "Communications Frequency" field specifies a frequency for the facility identified in
    the "Communications Type" (5.101) field.

    Content is derived from official government sources. The following details apply:

    HF frequencies are provided as five significant digits in kilohertz for 10 thousands,
    thousands, hundreds, tens and units. The remaining two positions of the seven-character
    field is zero filled.

    The decimal point is always suppressed. As all of these numeric expressions look alike,
    the "Frequency Units" field (Section 5.104) is provided to assist in actual frequency
    determination.
    */
    pub CommunicationsFrequencyHF: 7, div 100.0;
}

num_parseable! {
    /**
    The "Communications Frequency" field specifies a frequency for the facility identified in
    the "Communications Type" (5.101) field.

    Content is derived from official government sources. The following details apply:

    VHF frequencies with 100, 50 or 25 kilohertz spacing are provided as three significant
    digits and three decimals in megahertz for hundreds, tens, units, tenths, hundredths and
    thousandths. The remainder of the seven-character field is zero filled.

    VHF frequencies with 8.33 kHz spacing are provided as four significant digits and three
    decimals for the assigned channel number. The actual frequency (which would be three
    significant digits and four decimal places) is not provided.

    The decimal point is always suppressed. As all of these numeric expressions look alike,
    the "Frequency Units" field (Section 5.104) is provided to assist in actual frequency
    determination.
    */
    pub CommunicationsFrequencyVHF: 7, div 1000.0;
}

num_parseable! {
    /**
    The "Communications Frequency" field specifies a frequency for the facility identified in
    the "Communications Type" (5.101) field.

    Content is derived from official government sources. The following details apply:

    UHF frequencies are provided as three significant digits and two decimals in megahertz
    for hundreds, tens, units, tenths and hundredths. The remainder of the seven-character
    field is zero filled.

    The decimal point is always suppressed. As all of these numeric expressions look alike,
    the "Frequency Units" field (Section 5.104) is provided to assist in actual frequency
    determination.
    */
    pub CommunicationsFrequencyUHF: 7, div 100.0;
}
