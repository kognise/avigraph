use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Transition Identifier" field describes the type of transition to be made from the
    enroute environment into the terminal area and vice versa, and from the terminal area to
    the approach or from the runway or helipad to the terminal area.

    The content of the transition identifier field should be determined from the content of
    the Route Type field.

    Note 1: If there is no Route Type 1 or 4 or F for the SID, then the SID records with the
    Route Type of 2 or 5 or M will have an entry in the Transition Identifier field. If there
    is a Route Type of 1 or 4 or F for the procedure, then the records with the Route Type of
    2 or 5 or M will carry a blank transition identifier field.

    Note 2: If there is no Route Type 3 or 6 or 9 or S for the STAR, then the STAR record
    with the Route Type of 2 or 5 or 8 or M will have an entry in the Transition Identifier
    field. If there is a Route Type 3 or 6 or 9 or S for the procedure, then the Transition
    Identifier in the Route Type 2 or 5 or 8 or M will carry a blank transition identifier
    field.

    Note 3: The use of "ALL" in the Transition Identifier field indicates that the procedure
    is valid for two or more runways at an airport or all helipads at a heliport. The use of
    the character "B" along with a runway designation such as RW08B in the Transition
    Identifier field indicates that the transition is valid for two or more parallel runways,
    e.g. RW08L and RW08R.

    Note 4: The Missed Approach Transition Identifier will be the identifier of the Missed
    Approach Holding Fix or the last fix in the missed approach path if there is no holding
    fix. If multiple missed approach paths are published to the same termination but with
    different paths or constraints, a transition identifier closely aligned with the
    published source indication for each missed approach will be used.

    *This field is very complex to parse and has a lot of variants, and often the parsed
    value won't be required, so I opted to simply leave this as a string for now.*
    */
    pub ApproachRouteIdentifier: 6;
}
