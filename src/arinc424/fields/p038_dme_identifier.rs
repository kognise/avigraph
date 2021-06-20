use crate::trimmed_str_parseable;

trimmed_str_parseable! {
    /**
    The identification of a DME facility, a TACAN facility or the DME (or TACAN)
    component of a VORDME or VORTAC facility.

    The "DME Identifier" field will contain the officially published 2-, 3-, or
    4-character DME facility identifier. For VOR/DME and VORTAC facilities, if
    the identification codes of the VOR and DME components of the Navaid defined
    in the record are the same, the field will be blank. If they are not the same,
    the VOR Identification will be as defined in Section 5.33 and the DME Identifier
    field will carry the identification of the DME component. The field is blank
    when the VHF Navaid facility in the reference record has no DME component. The
    field will always contain the DME Identifier for TACANs, DME Only NavaidS and
    Localizer or MLS DME facilities.
    */
    pub DMEIdentifier: 4;
}
