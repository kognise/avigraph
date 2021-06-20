use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    **From/To - Airport/Fix**

    When used on Company Routes, the "From Airport/Fix" is the waypoint from which the
    company route originates. The "To Airport/Fix" is the waypoint at which the company route
    terminates. When used on Alternate Records, it is the Departure, Destination or Enroute
    Airport/Fix for which the alternate information is being provided.

    The customer is responsible for defining points at which company routes originate and
    terminate and for defining which departure, destination or enroute points are to have
    alternate information.
    */
    pub FromToAirportFix: 5;
}
