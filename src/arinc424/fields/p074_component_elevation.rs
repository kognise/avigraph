use crate::num_parseable;

num_parseable! {
    /**
    The "Component Elevation" field defines the elevation of a given component in the
    Localizer, GLS and MLS records. The "Glide Slope Elevation (GS ELEV)" defines the
    elevation of the Glide Slope component in the Localizer Records. The "EL Elevation (EL
    ELEV)" defines the elevation of the Elevation component of the MLS Record, the "Azimuth
    Elevation (AZ ELEV)" defines the elevation of the Azimuth component of the MLS Record and
    the "Back Azimuth Elevation (BAZ ELEV)" defines the elevation of the Back Azimuth
    component of the MLS Record. The "GLS station elevation (GLS ELEV)" defines the elevation
    of the GLS ground station in the GLS record.

    Elevations specified in official government publications are entered in this field with
    respect to MSL. When the elevation is below MSL, the first column of the field contains a
    minus (-) sign.
    */
    pub ComponentElevation i32: 5;
}
