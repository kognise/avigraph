use crate::enum_parseable;

enum_parseable! {
    pub enum EnrouteSubsection {
        Waypoint = "A",
        Marker = "M",
        HoldingPattern = "P",
        AirwaysAndRoutes = "R",
        PreferredRoute = "T",
        AirwayRestriction = "U",
        Communication = "V",
    }
}
