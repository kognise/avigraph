use crate::num_parseable;

num_parseable! {
    /**
    The "Touchdown Zone Elevation" is the highest elevation in the first 3,000 feet of the
    landing surface beginning at the threshold.

    Touchdown zone elevations from official government sources will be used when available.
    If official source is not available, the runway threshold elevation will be entered. If
    the runway threshold elevation is not available, the Airport reference point elevation
    will be entered. (See TDZE Location, Section 5.98) The elevation will be entered in feet,
    to a resolution of 1 foot, with respect to MSL. For below MSL elevations, the first
    character of the field is a minus (-) sign.
    */
    pub TouchDownZoneElevation i32: 5;
}
