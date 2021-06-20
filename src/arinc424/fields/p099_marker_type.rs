use crate::autoparse_struct;
use crate::enum_parseable;
use crate::trimmed_str_parseable;

autoparse_struct! {
    /**
    The "Marker Type" field defines the type of marker.
    */
    pub MarkerType {
        locator: MarkerTypeLocator,
        variant: MarkerTypeVariant,
        useless: MarkerTypeUnknown,
    }
}

enum_parseable! {
    pub enum MarkerTypeLocator {
        LocatorAtMarker = "L",
        None = " ",
    }
}

enum_parseable! {
    pub enum MarkerTypeVariant {
        Inner = "I",
        Middle = "M",
        Outer = "O",
        Back = "B",
        None = " ",
    }
}

trimmed_str_parseable! {
    /**
    *I'm not sure why this exists because it seems to always be "M," but I'm leaving it in
    the parser for the sake of completeness. Should be safe to disregard.*
    */
    pub MarkerTypeUnknown: 1;
}
