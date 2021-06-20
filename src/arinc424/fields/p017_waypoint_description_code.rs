use crate::{autoparse_struct, enum_parseable};

autoparse_struct! {
    /**
    Fixes are located at positions significant to navigation in the Enroute, Terminal Area and
    Approach Procedure path definitions. The "Waypoint Description Code" field enables that
    significance or function of a fix at a specific location in a route to be identified. The
    field provides information on the type of fix. As a single fix can be used in different route
    structures and multiple times within a given structure, the field provides the function for
    each occurrence of a fix.

    The contents of Column 40 provide information on the fix type. Column 41 is used to define
    whether the fix is a "fly-over" or "fly-by" fix and to indicate the charting status of some
    waypoints. Columns 42 and 43 provide the fix function information. Column 40, Code "G," is
    valid for Runway as Waypoint and Helipad as Waypoint.
    */
    pub WaypointDescriptionCode {
        type1: WaypointDescriptionCodeType1,
        type2: WaypointDescriptionCodeType2,
        type3: WaypointDescriptionCodeType3,
        type4: WaypointDescriptionCodeType4,
    }
}

enum_parseable! {
    pub enum WaypointDescriptionCodeType1 {
        /**
        *Used on STAR, APCH.*
        */
        AirportAsWaypoint = "A",

        /**
        Any waypoint (not Navaid, Airport or Runway) in Terminal Procedures or any waypoint
        (not Navaid or airport) on Enroute Airways, required for navigation such as a change
        in bearing, intersection of two airways, beginning or end of continuous segment.

        *Used on Enroute, SID, STAR, APCH.*
        */
        EssentialWaypoint = "E",

        /**
        Any waypoint published by government source but not part of any route structure.

        *Used on Enroute.*
        */
        OffAirwayWaypoint = "F",

        /**
        *Used on SID, STAR, APCH.*
        */
        RunwayOrHelipadAsWaypoint = "G",

        /**
        *Used on STAR, APCH.*
        */
        HeliportAsWaypoint = "H",

        /**
        *Used on Enroute, SID, STAR, APCH.*
        */
        NDBNavaidAsWaypoint = "N",

        /**
        A waypoint established during procedure coding on the nominal track.

        *Used on SID, STAR, APCH.*
        */
        PhantomWaypoint = "P",

        /**
        Any waypoint (not Navaid or airport) on Enroute Airways that is not considered "Essential"
        or "Transition Essential."

        *Used on Enroute.*
        */
        NotEssentialWaypoint = "R",

        /**
        Any waypoint (not Navaid or airport) on Enroute Airways for the purpose of transitioning
        between the Enroute and Terminal structures.

        *Used on Enroute.*
        */
        TransitionEssentialWaypoint = "T",

        /**
        *Used on Enroute, SID, STAR, APCH.*
        */
        VHFNavaidAsWaypoint = "V",

        None = " ",
    }
}

enum_parseable! {
    pub enum WaypointDescriptionCodeType2 {
        /**
        **Fly-Over Waypoint, End of SID, STAR Route Type, APCH Transition or Final Approach**

        A fly-over waypoint (including Navaid) specified by the procedure: (a) at the end of
        a SID or STAR Route Type; (b) at the end of an Approach Transition for FMS, GPS, or
        MLS/RNAV approach; or (c) at the missed approach point in an Approach Procedure.

        *Used on SID, STAR, APCH.*
        */
        EndingFlyOverWaypoint = "B",

        /**
        **End of Enroute Airway or Terminal Procedure Route Type**

        *Used on Enroute, SID, STAR, APCH.*
        */
        AirwayOrTERPEnd = "E",

        /**
        Any waypoint (not Navaid and airport) on Enroute Airways that has not been established
        by government source. Used only in conjunction with "E" (Essential Waypoint) in Column 40.

        *Used on Enroute.*
        */
        UnchartedAirwayIntersection = "U",

        /**
        Any waypoint (including Navaid and airport) that must be over flown before establishing
        on the following leg.

        *Used on SID, STAR, APCH.*
        */
        FlyOverWaypoint = "Y",

        None = " ",
    }
}

enum_parseable! {
    pub enum WaypointDescriptionCodeType3 {
        /**
        Any waypoint established by the government source in support of RNAV-GPS/GLS Approach
        Procedures. Path Points are not part of the defined procedure track but are provided in
        a separate record where required. The points are not named and are always referred to
        as Path Point 1 and Path Point 2.

        *Used on APCH.*
        */
        UnnamedStepdownFixAfterFAF = "A",

        /**
        Any waypoint established by the government source in support of RNAV-GPS/GLS Approach
        Procedures. Path Points are not part of the defined procedure track but are provided in
        a separate record where required. The points are not named and are always referred to
        as Path Point 1 and Path Point 2.

        *Used on APCH.*
        */
        UnnamedStepdownFixBeforeFAF = "B",

        /**
        Any waypoint (including Navaid and airport) on Enroute Airways at which a "position
        report" must be made to the appropriate Air Traffic Control unit.

        *Used on Enroute.*
        */
        ATCCompulsoryWaypoint = "C",

        /**
        Any waypoint (including Navaid) designated as the start/end of an oceanic organized
        track system.

        *Used on Enroute.*
        */
        OceanicGatewayWaypoint = "G",

        /**
        **First Leg of Missed Approach Procedure**

        Coded on the first leg after a runway fix or missed approach point fix dependent on
        approach procedure coding rules in Attachment 5. The leg may be the first leg of a
        published missed approach procedure or a leg to the published missed approach point.

        *Used on APCH.*
        */
        FirstLegOfMAP = "M",

        /**
        Any waypoint established by the government source in support of RNAV-GPS/GLS Approach
        Procedures. Path Points are not part of the defined procedure track but are provided
        in a separate record where required. The points are not named and are always referred
        to as Path Point 1 and Path Point 2.

        *Used on APCH.*
        */
        PathPointFix = "P",

        /**
        Any waypoint established and named by the government source lying between the Final
        Approach Fix and the Missed Approach Point or between a published Final Approach
        Course Fix and a Final Approach Fix.

        *Used on APCH.*
        */
        NamedStepdownFix = "S",

        None = " ",
    }
}

enum_parseable! {
    pub enum WaypointDescriptionCodeType4 {
        /**
        **Initial Approach Fix**

        Any waypoint (including Navaid) established as an Initial Approach Fix.

        *Used on APCH.*
        */
        IAF = "A",

        /**
        **Intermediate Approach Fix**

        Any waypoint (including Navaid) established as an Intermediate Approach Fix and not
        coded as a Final Approach Course Fix.

        *Used on APCH.*
        */
        IF = "B",

        /**
        **Initial Approach Fix with Holding**

        *Used on APCH.*
        */
        IAFWithHolding = "C",

        /**
        **Initial Approach Fix with Final Approach Course Fix**

        *Used on APCH.*
        */
        IAFWithFACF = "D",

        /**
        **Final End Point Fix**

        Any waypoint established as the Final End Point. This may be a fix published as the
        FEP by the government source or, when no such fix is published but yet required, one
        established by the data supplier. It is used in vertical coding of non-precision
        approach procedures.

        *Used on APCH.*
        */
        FEPF = "E",

        /**
        **Published Final Approach Fix or Database Final Approach Fix**

        Any waypoint (including Navaid) established as a Final Approach Fix. This may be a
        fix published as the Final Approach Fix by government source or, when no such fix is
        published, one established by a data supplier ARINC SPECIFICATION 424 - Page 65 5.0
        NAVIGATION DATA - FIELD DEFINITIONS according to the rules in Attachment 5.

        *Used on APCH.*
        */
        PublishedFAFOrDBFAF = "F",

        /**
        *Used on Enroute, SID, STAR, APCH.*
        */
        HoldingFix = "H",

        /**
        Any waypoint (including Navaid) established as a Final Approach Course Fix. This may be
        a fix published as the Final Approach Course Fix by government source or, when no such
        fix is published but yet required, one established by a data supplier according to the
        rules in Attachment 5.

        *Used on APCH.*
        */
        FACF = "I",

        /**
        Any waypoint (including Navaid or Runway) established as a Missed Approach Point by
        government source, may follow a Runway Fix when such is required by the rule in
        Attachment 5. The code is used in conjunction with "G" in Column 40 when the Runway is
        the published Missed Approach Point.

        *Used on APCH.*
        */
        PublishedMissedApproachpointFix = "M",

        None = " ",
    }
}
