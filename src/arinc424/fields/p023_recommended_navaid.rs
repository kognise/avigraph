use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The "Recommended Navaid" field allows the reference facility for the waypoint in a given
    record "Fix Ident" field or for an Airport or Heliport to be specified. VHF, NDB (Enroute
    and Terminal), Localizer, TACAN, GLS and MLS Navaids may be referenced.

    The 1, 2, 3 or 4 character identification of the Navaid appears in this field. Navaids
    recommended for waypoint reference in official government publications will be used when
    available. The following general rules on field content apply:

    1. A "VHF Navaid" may be any VOR, DME, VORDME, VORTAC, TACAN or Un-Biased ILSDME available
    in the database.
    2. A "NDB Navaid" may be any NDB or Locator (Terminal NDB) available in the database.
    3. Localizers, GLS Reference Path, and MLS Azimuth are used as Recommended Navaids for
    procedures that reference those navaids.
    4. The Recommended Navaid in final approach procedure coding will be the procedure
    reference facility (when Recommended Navaid is provided in coding).
    5. The Recommended Navaid in Airport and Heliport Records will be any VOR, VORDME, TACAN
    or VORTAC available in the database.
    6. The Recommended Navaid in any Enroute Airway Record will be any VORDME or VORTAC available
    in the database.
    7. The Recommended Navaid in any Terminal Procedure Record other than the final approach
    coding will be the procedure reference facility of a type from the Definition/Description
    paragraph above and will be in accordance with the rules governing Recommended Navaids
    for Path Terminators and coding rule as defined in Attachment 5 of this Specification.
    8. The rules for Recommended Navaids for Converging ILS Approach Procedures are the same
    as for ILS Approach Procedures.
    9. The Recommended Navaid used in a GLS Approach Procedure will be the GLS Ref Path
    identifier appropriate to the runway and approach.
    */
    pub RecommendedNavaid: 4;
}
