use crate::autoparse_struct;
use crate::enum_parseable;
use crate::num_parseable;

autoparse_struct! {
    /**
    The 'Duplicate Identifier' field is used to further define holding patterns when
    official government source has designated more than one Holding Pattern on a Navaid
    or Waypoint.

    Holding Patterns are derived from official government sources documents. That
    documentation will normally specify the airspace structure in which the holding is
    to be used. That documentation may also designate more than one Holding Pattern for
    a single Navaid or Waypoint. This field will contain details on airspace structure
    and multiple designations. More than one holding is designated on a single 'fix'
    when one or more of the following elements are different for holdings within the
    same airspace structure.Inbound Holding Course, Turn Direction, Altitude, Leg Length
    or Leg Time, and Holding Speed.

    If only one Holding Pattern is designated for a 'fix' and the airspace structure in
    which that holding is to be used is not defined, the field will contain '00.' If only
    one Holding Pattern is designated for a 'fix' and the airspace structure in which that
    holding is to be used is defined or if the same holding is designated for more than
    one airspace structure, the first position of the 'Duplicate Indicator' will contain a
    digit of 1 through 6 and the second position will contain a zero. If more than one
    holding is designated for a single 'fix' in one type of airspace structure, the first
    position will contain a digit of 1 through 6 and the second position will contain a digit
    of 1 through 9, depending on the number of holdings on that 'fix' within that airspace
    structure.

    If multiple holdings are designated in official source documents for a single 'fix' and
    the airspace structure is not defined for all holdings, those with 'undefined airspace
    structure' will carry the digit 7 in position one and a digit of 0 through 9 in position
    two.
    */
    pub DuplicateIdentifier {
        airspace: DuplicateIdentifierAirspace,
        multiple: DuplicateIdentifierMultiple,
    }
}

enum_parseable! {
    pub enum DuplicateIdentifierAirspace {
        AllUndefined = "0",
        HighAltitude = "1",
        LowAltitude = "2",
        SID = "3",
        STAR = "4",
        Approach = "5",
        MissedApproach = "6",
        ThisUndefined = "7",
        AllAltitude = "8",
    }
}

num_parseable! {
    /**
    If there is only one holding pattern on a given fix within an airspace structure,
    position 2 will contain a '0.' For additional (multiple) holdings on that same
    fix within the same airspace structure, position 2 will be incremented by 1, e.g.
    '0' for the first '1' for the second, etc.
    */
    pub DuplicateIdentifierMultiple u8: 1;
}
