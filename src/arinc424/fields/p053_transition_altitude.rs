use crate::num_parseable;

num_parseable! {
    /**
    **Transition Altitude/Level**

    The "Transition Altitude" field defines the altitude in the vicinity of an airport or
    heliport at or below which the vertical position of an aircraft is controlled by
    reference to altitudes (MSL). The "Transition Level" field defines the lowest flight
    level available for use above the transition altitude. Aircraft descending through the
    transition layer will use altimeters set to local station pressure, while departing
    aircraft climbing through the layer will be using standard altimeter setting (QNE) of
    29.92 inches of mercury, 1013.2 millibars or 1013.2 hectopascals.

    Transition Altitudes/Levels are derived from official government sources. For STAR and
    Approach records, the field defines the level, expressed in feet, at which the altimeter
    barometric setting is changed from standard to local values for the airport or heliport
    identified in the record. For SID records, the field will contain the Transition Altitude
    expressed in feet. The first leg of each Airport and Heliport SID/ STAR/Approach
    procedure shall contain the appropriate transition altitude with a resolution of one
    foot. If the transition altitude is unknown "by ATC," the field will be blank in
    procedure records. For Airport and Heliport records, the Transition Altitude and
    Transition Level will be entered into the appropriate fields, in feet with a resolution
    of one foot. If the Transition Altitude or Level is unknown, "by ATC" or has different
    values for varying procedures at the airport or heliport, the field will be blank.
    */
    pub TransitionAltitude u32: 5;
}
