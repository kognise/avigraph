use crate::arinc424::fields::*;
use crate::autoparse_struct;

autoparse_struct! {
    /**
    The VHF Navaid file contains details of all VOR, VOR/DME, VORTAC,
    DME and TACAN stations within the geographical area of interest.
    For VOR and TACAN stations having the same identifier but different
    operating frequencies, the TACAN is available and the VOR is suppressed
    unless the VOR is required to support Sections 3.2.3.3, 3.2.3.4, 3.2.4.4,
    3.2.4.5, 3.2.4.6, 3.2.4.11, 3.2.5, 3.2.9, 3.3.5, 3.3.6, 3.3.7 or 3.3.8.
    In such cases the VOR is available and the TACAN is suppressed.
    */
    pub VHFNavaid {
        airport_identifier: AirportHeliportIdentifier,
        airport_icao_code: ICAOCode,
        #(skip 1),

        vor_identifier: VORNDBIdentifier,
        #(skip 2),
        vor_icao_code: ICAOCode,
        #(continuation 1),

        vor_frequency: VORFrequency,
        vor_class: VORNavaidClass,
        vor_latitude: Latitude,
        vor_longitude: Longitude,

        dme_identifier: DMEIdentifier,
        dme_latitude: Latitude,
        dme_longitude: Longitude,
        dme_declination: StationDeclination,
        dme_elevation: DMEElevation,

        figure_of_merit: FigureOfMerit,
        ils_dme_bias: ILSDMEBias,
        frequency_protection: FrequencyProtectionDistance,
        datum: DatumCode,
        name: Name30,

        record_number: FileRecordNumber,
        cycle: Cycle,
    }
}
