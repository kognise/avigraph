use crate::num_parseable;

num_parseable! {
    /**
    The "Threshold Crossing Height" specifies the height above the landing threshold on a
    normal glide path.

    The Threshold Crossing Height will be derived from official government sources when
    available. As provided on Runway Records, the TCH value will be the Glide Slope Height at
    the landing threshold for runways with ILS or MLS approaches. If an ILS or MLS is not
    available and an RNAV approach is available it will be the published TCH for that
    procedure. When ILS/MLS or RNAV values are not available but a published VGSI with TCH is
    available, it will be used. If none of these values are available, it will be 50 feet.

    Based on the information contained in the Source/Content paragraph, it should be noted
    that the single TCH value provided on the Runway Record may be different than the TCH
    value provided on the Approach Continuation Record for a procedure to that same runway.
    These differences may be significant. A comparison of procedure altitude data to
    threshold elevation and threshold crossing heights should only be made to the Approach
    Continuation Record. It should also be noted that a TCH associated with the VGSI for the
    same runway may be different than either value. Some government sources will provide
    information on these procedure to VGSI differences when they are three feet or greater.

    *This is a split field, `ThresholdCrossingHeightContinuation` contains the equivalent for
    continuation records.*
    */
    pub ThresholdCrossingHeight u16: 2;
}

num_parseable! {
    /**
    The "Threshold Crossing Height" specifies the height above the landing threshold on a
    normal glide path.

    The Threshold Crossing Height will be derived from official government sources when
    available. As provided on Runway Records, the TCH value will be the Glide Slope Height at
    the landing threshold for runways with ILS or MLS approaches. If an ILS or MLS is not
    available and an RNAV approach is available it will be the published TCH for that
    procedure. When ILS/MLS or RNAV values are not available but a published VGSI with TCH is
    available, it will be used. If none of these values are available, it will be 50 feet.

    When used on Approach Continuation Records, the field will contain the published TCH for
    that procedure. When used on ILS or MLS Records, it will be the height of the glide slope
    at the landing threshold.

    Based on the information contained in the Source/Content paragraph, it should be noted
    that the single TCH value provided on the Runway Record may be different than the TCH
    value provided on the Approach Continuation Record for a procedure to that same runway.
    These differences may be significant. A comparison of procedure altitude data to
    threshold elevation and threshold crossing heights should only be made to the Approach
    Continuation Record. It should also be noted that a TCH associated with the VGSI for the
    same runway may be different than either value. Some government sources will provide
    information on these procedure to VGSI differences when they are three feet or greater.

    *This is a split field, `ThresholdCrossingHeight` contains the equivalent for
    non-continuation records.*
    */
    pub ThresholdCrossingHeightContinuation u16: 2;
}
