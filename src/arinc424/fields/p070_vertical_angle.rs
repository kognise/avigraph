use crate::num_parseable;

num_parseable! {
    /**
    The "Vertical Angle" field defines the vertical navigation path prescribed for the
    procedure. The vertical angle should cause the aircraft to fly at the last coded altitude
    and then descend on the angle, projected back from the fix and altitude code for that fix
    at which the angle is coded. Vertical Angle information is provided only for descending
    vertical navigation. The angle is preceded by a "–" (minus sign) to indicate the
    descending flight.

    Values from official government sources will be used when available. In coding of
    procedures with navaids providing an electronic glide slope, the Vertical Angle is the
    angle assigned to that glide slope. In coding of procedures with a published VNAV Angle,
    it is that angle or one calculated by the data supplier. In coding of procedures with no
    government source vertical angle data, it is always a data supplier calculated value; see
    Attachment Five, Procedure Coding Rules. The angles are expressed in degrees, tenths and
    hundredths of a degree, with the decimal point suppressed. The Localizer Only Vertical
    Angle is a value provided for non-precision use of an ILS procedure as a Localizer Only
    (Glide Slope Out) procedure.
    */
    pub VerticalAngle: 4, div 100.0;
}
