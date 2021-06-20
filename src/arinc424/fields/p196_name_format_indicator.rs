use crate::autoparse_struct;
use crate::enum_parseable;

autoparse_struct! {
    /**
    The "Name Format Indicator" field is used to describe the format of the
    "Waypoint Name/Description" field (5.43). This field will be formatted
    according to the rules described in Chapter 7 of this Specification,
    Waypoint Naming Conventions.

    Values for this field have no official government source and are adjusted
    by input from the following table. Code may not be used in combination
    between columns.
    */
    pub NameFormatIndicator {
        fix: NameFormatIndicatorFix,
        marker: NameFormatIndicatorMarker,
        unused: NameFormatIndicatorUnused,
    }
}

enum_parseable! {
    pub enum NameFormatIndicatorFix {
        Abeam = "A",
        BearingAndDistance = "B",
        AirportName = "D",
        FIR = "F",
        PhoneticLetterName = "H",
        AirportIdent = "I",
        Coordinate = "L",
        MultipleWordName = "M",
        NavaidIdent = "N",

        /// Published Five Letter Name Fix
        PublishedNameFive = "P",

        /// Published Name Fix, less than five letters
        PublishedNameLTFive = "Q",

        /// Published Name Fix, more than five letters
        PublishedNameGTFive = "R",

        /// The "T" indicator will be used with all fixes established in accordance
        /// with Chapter 7, Section 7.2.6, Terminal Waypoints.
        AirportRunwayRelated = "T",

        UIR = "U",
        None = " ",
    }
}

enum_parseable! {
    pub enum NameFormatIndicatorMarker {
        /// Localizer Marker with officially published five-letter identifier
        LocalizerMarkerPublished = "O",

        /// Localizer Marker without officially published five-letter identifier
        LocalizerMarkerUnublished = "M",

        None = " ",
    }
}

enum_parseable! {
    /// Column 98 is reserved for future expansion of the Name-Format-Indicator concept.
    pub enum NameFormatIndicatorUnused {
        None = " ",
    }
}
