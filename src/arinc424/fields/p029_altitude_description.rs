use crate::enum_parseable;

enum_parseable! {
    /**
    The "Altitude Description" field will designate whether a waypoint should be crossed
    "at," "at or above," "at or below" or "at or above to at or below" specified altitudes.
    The field is also used to designate recommended altitudes and cases where two distinct
    altitudes are provided at a single fix.

    Localizer Only Altitude information is provided in the Approach Continuation Record for
    Precision Approach Procedures with electronic Glide Slope using the codes for "At," "At
    or Above" and "At or Below" appropriately based on government source publications. This
    Altitude is the non-precision altitude at the fix on which it is coded. Provided on FAF
    and step- down fix waypoints.
    */
    pub enum AltitudeDescription {
        /**
        "At or above" altitude specified in first "Altitude" field. Also used with Localizer Only
        Altitude field.
        */
        AtOrAboveFirst = "+",

        /**
        "At or below" altitude specified in first "Altitude" field. Also used with Localizer Only
        Altitude field.
        */
        AtOrBelowFirst = "-",

        /**
        "At" altitude specified in first "Altitude" field. Also used with Localizer Only Altitude
        field.

        In Final Approach Route Coding for Precision Approach Procedures with electronic Glide
        Slope the codes "@ (for blank)," "G," "H," "I," and "J," are applied.
        */
        AtFirst = " ",

        /**
        "At or above to at or below" altitudes specified in the first and second "Altitude"
        fields.

        The "B" entry may only appear in Airport and Heliport SID/STAR/Approach Route,
        Airport/Enroute/Heliport Communications, VHF Navaid Limitation and Preferred Route
        Records. The higher value will always appear first in records with two altitude fields,
        or as the first three digits of the Altitude Limitation field.

        In Approach Records, use is limited to Approach Transitions with the exception of the
        last leg of a transition and to Missed Approach with the exception of the first leg of a
        missed approach.
        */
        Range = "B",

        /**
        "At or above" altitude specified in second "Altitude" field.

        The "C" entry may only appear in SID records. It is used to indicate that the leg has a
        conditional altitude termination.
        */
        AtOrAboveSecond = "C",

        /**
        Glide Slope altitude (MSL) specified in the second "Altitude" field and "at" altitude
        specified in the first "Altitude" field on the FAF Waypoint in Precision Approach Coding
        with electronic Glide Slope.

        In Final Approach Route Coding for Precision Approach Procedures with electronic Glide
        Slope the codes "@ (for blank)," "G," "H," "I," and "J," are applied.
        */
        AtGlideSlopeFAF = "G",

        /**
        Glide Slope Altitude (MSL) specified in second "Altitude" field and "at or above"
        altitude specified in first "Altitude" field on the FAF Waypoint in Precision Approach
        Coding with electronic Glide Slope.

        In Final Approach Route Coding for Precision Approach Procedures with electronic Glide
        Slope the codes "@ (for blank)," "G," "H," "I," and "J," are applied.
        */
        AtOrAboveGlideSlopeFAF = "H",

        /**
        Glide Slope Intercept Altitude specified in second "Altitude" field and "at" altitude
        specified in first "Altitude" field on the FACF Waypoint in Precision Approach Coding
        with electronic Glide Slope.

        In Final Approach Route Coding for Precision Approach Procedures with electronic Glide
        Slope the codes "@ (for blank)," "G," "H," "I," and "J," are applied.

        Codes "I" and "J" are only used when the first altitude for the FACF fix is not blank. If
        that altitude is blank, the Altitude Description is also blank, even when the second
        altitude is not blank
        */
        AtGlideSlopeFACF = "I",

        /**
        Glide Slope Intercept Altitude specified in second "Altitude" field and "at or above"
        altitude specified in first "Altitude" field on the FACF Waypoint in Precision Approach
        Coding with electronic Glide Slope.

        In Final Approach Route Coding for Precision Approach Procedures with electronic Glide
        Slope the codes "@ (for blank)," "G," "H," "I," and "J," are applied.

        Codes "I" and "J" are only used when the first altitude for the FACF fix is not blank. If
        that altitude is blank, the Altitude Description is also blank, even when the second
        altitude is not blank
        */
        AtOrAboveGlideSlopFACF = "J",

        /**
        "At" altitude on the coded vertical angle in the second "Altitude" field and "at or
        above" altitude specified in first "Altitude" field on step-down fix waypoints.

        The codes "V," "X" and "Y" are used with all fixes defined in government source as
        step-down fixes and from the FACF inbound on final approach coding. There can be two
        altitudes provided on every fix in final approach coding. Altitude 1 is the altitude and
        constraint specified at the fix, the so-called "procedure altitude." Altitude 2 is the
        "at" altitude on the coded vertical path at the fix.
        */
        AtOrAboveStepDownFix = "V",

        /**
        "At" altitude on the coded vertical angle in the second "Altitude" field and "at"
        altitude specified in the first "Altitude" field on step- down fix waypoints.

        The codes "V," "X" and "Y" are used with all fixes defined in government source as
        step-down fixes and from the FACF inbound on final approach coding. There can be two
        altitudes provided on every fix in final approach coding. Altitude 1 is the altitude and
        constraint specified at the fix, the so-called "procedure altitude." Altitude 2 is the
        "at" altitude on the coded vertical path at the fix.
        */
        AtStepDownFix = "X",

        /**
        "At" altitude on the coded vertical angle in the second "Altitude" field and "at or
        below" altitude specified in the first "Altitude" field on step-down fix waypoints.

        The codes "V," "X" and "Y" are used with all fixes defined in government source as
        step-down fixes and from the FACF inbound on final approach coding. There can be two
        altitudes provided on every fix in final approach coding. Altitude 1 is the altitude and
        constraint specified at the fix, the so-called "procedure altitude." Altitude 2 is the
        "at" altitude on the coded vertical path at the fix.
        */
        AtOrBelowStepDownFix = "Y",
    }
}
