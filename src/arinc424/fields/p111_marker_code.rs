use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Marker Code" field contains the coded ident that provides an aural and visual
    indication of station passage in the cockpit. The code shall be keyed so as to transmit
    dots or dashes, or both, in an appropriate sequence on a radio frequency of 75 MHz. The
    frequency of the modulating tone is 3000 Hz.

    The field contains the morse code ident (dots and dashes) derived from official
    government sources.
    */
    pub MarkerCode: 4;
}
