use crate::arinc424::fields::*;
use crate::autoparse_struct;

autoparse_struct! {
    /**
    The Enroute NDB Navaid file (DB) contains all enroute on-airway and off-airway
    NDBs within the geographical area of interest. The Terminal NDB Navaid file (PN)
    contains NDBs associated with the Airports contained in Subsection 3.2.4.1 and
    Heliport contained in Section 3.3.3. Terminal NDBs referenced to wo or more Airports
    or Heliports will be available in the Enroute NDB Subsection unless that handling
    would create duplicate NDB identifiers within that Subsection. Marine Beacons shown
    on aeronautical charts may also be included in this record type.
    */
    pub NDBNavaid {
        airport_identifier: AirportHeliportIdentifier,
        airport_icao_code: ICAOCode,

        #(skip 1),
        ndb_identifier: VORNDBIdentifier,
        #(skip 2),
        ndb_icao_code: ICAOCode,
        #(continuation 1),

        ndb_frequency: NDBFrequency,
        ndb_class: NDBNavaidClass,
        ndb_latitude: Latitude,
        ndb_longitude: Longitude,

        #(skip 23),
        magnetic_variation: MagneticVariation,
        #(skip 6),
        #(skip 5),
        datum: DatumCode,
        name: Name30,

        record_number: FileRecordNumber,
        cycle: Cycle,
    }
}
