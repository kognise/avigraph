use crate::num_parseable;

num_parseable! {
    /**
    The "Localizer Width" field specifies the localizer course width of the ILS facility
    defined in the record.

    Localizer course widths from official government sources are entered into the field in
    degrees, tenths of a degree and hundredths of a degree with the decimal point suppressed.
    */
    pub LocalizerWidth: 4, div 100.0;
}
