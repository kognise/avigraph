use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The airport gate defined in the record is identified in the "Gate Identifier" field.

    Coded gate identity information is derived from official government sources and
    navigation system users.
    */
    pub GateIdentifier: 5;
}
