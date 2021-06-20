num_parseable! {
    /**
    This "Elevation" field defines the elevation of the VOR, NDB, ILS Marker, Airways Marker
    and Airport Communications stations. 

    Facility elevations specified in official government publications are entered into this
    field in feet with respect to MSL. When the elevation is below MSL, the first column of
    the field contains a minus (-) sign.
    */
    pub FacilityElevation i32: 5;
}