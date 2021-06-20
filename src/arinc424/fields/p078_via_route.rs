use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    *SID/STAR/App/Awy**

    This field is used to identify the particular route to be flown as referenced by the
    "VIA" field (Section 5.77).

    For Company Route records this field can contain the SID/STAR, Approach, Enroute Airway,
    or Preferred Route Identifier (Sections 5.8, 5.9, and 5.10). For Preferred Route records
    this field can contain the SID/STAR or Enroute Airway Route Identifier (Section 5.8).
    This field will be blank for certain records depending on the "VIA" field content
    (Section 5.77).

    *This field is very complex to parse and has a lot of variants, and often the parsed
    value won't be required, so I opted to simply leave this as a string for now.*
    */
    pub ViaRoute: 6;
}
