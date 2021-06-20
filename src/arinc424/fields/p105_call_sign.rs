use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Call Sign" field specifies the name of the facility being called.

    Call Signs are derived from official government sources. On airport Communications
    records, the type of facility being called will be omitted when is it the same as the
    communication type. On Enroute Communication records, the Call Name will be shown with
    the first record only of any Flight Information Region or Flight Service Station.
    */
    pub CallSign: 25;
}
