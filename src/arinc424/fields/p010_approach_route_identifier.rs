use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Approach Route Identifier" field contains the identifier of the approach route to be
    flown. To facilitate the provision of multiple approach procedures of the same type to a
    given runway, the field also is used to provide a "multiple indicator."

    For Approach procedures that are not specific to a runway such as circle-to-land
    procedures a 4 character Alpha entry should be used with the fifth alphanumeric character
    used for multiple indicator if necessary.

    For Helicopter Approach Procedures to Runways, the first position of the identifier will
    be the type of approach. The second through fourth positions will carry a three digit
    numeric character representing the runway designated or the procedure final approach
    course, expressed in full degrees. Where the same final approach course is used multiple
    times in official source, the fifth position will carry a multiple indicator.

    For Helicopter Approach Procedures to Heliports and coded to a specific pad, the first
    position of the identifier will carry a character indicating the type of approach. The
    second through sixth characters will carry the pad identification. There is no provision
    for a multiple indicator for more than one approach of the same type to the same pad in
    this identifier field. When required, a multiple indicator is provided in a separate
    field.

    *This field is very complex to parse and has a lot of variants, and often the parsed
    value won't be required, so I opted to simply leave this as a string for now.*
    */
    pub ApproachRouteIdentifier: 6;
}
