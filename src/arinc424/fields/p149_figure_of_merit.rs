use crate::enum_parseable;

enum_parseable! {
    /**
    The 'Figure of Merit' field is used to specify VHF Navaid facility usable ranges beyond
    that specified in the Class field. It is also used to specify when a VHF Navaid contained
    in the database is not available for operational use, i.e., is out of service and is used
    to flag a VHF Navaid that is not included in a civilian international NOTAM system.

    Actual Field Entry Values are not contained in official government source but rather are
    derived values based on usage, class, availability etc. These may be further adjusted by
    input from actual users.
    */
    pub enum FigureOfMerit {
        Terminal = "0",
        LowAltitude = "1",
        HighAltitude = "2",
        ExtendedHighAltitude = "3",

        /// Navaid not included in a civil international NOTAM system.
        NoNOTAM = "7",

        OTS = "9",
        Unknown = " ",
    }
}
