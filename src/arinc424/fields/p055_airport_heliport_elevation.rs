use crate::num_parseable;

num_parseable! {
    /**
    The elevation of the Airport/Heliport specified in the record is defined in the "Airport
    Elevation" and "Heliport Elevation" field.

    Airport/Heliport elevations are to be derived from official government sources and
    entered into the field in feet to a resolution of one foot. For elevations above MSL, the
    field contains the numeric characters of the elevation only. For below MSL elevations the
    first character of the field is a minus (-) sign. In most cases, airport elevation is
    defined as the highest elevation of any landing surface on the airport.
    */
    pub AirportHeliportElevation i32: 5;
}
