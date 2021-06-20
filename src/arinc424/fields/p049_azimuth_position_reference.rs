use crate::enum_parseable;

enum_parseable! {
    /**
    **Localizer/Azimuth Position Reference**

    The "Localizer/Azimuth Position Reference" field indicates whether the antenna is
    situated beyond the stop end of the runway, ahead of or beyond the approach end of the
    runway. The "Back Azimuth Position Reference" field indicates whether the antenna is
    situated ahead of the approach end of the runway, ahead of or beyond the stop end of the
    runway.

    For Localizer and Azimuth positions the field is blank (@) when the antenna is situated
    beyond the stop end of the runway, it contains a plus (+) sign when the antenna is
    situated ahead of the approach end of the runway or a minus (-) sign when it is located
    off to one side of the runway. For Back Azimuth positions the field is blank (@) when the
    antenna is situated ahead of the approach end of the runway, it contains a plus (+) sign
    when the antenna is situated beyond the stop end of the runway or a minus (-) sign when
    it is located off to one side of the runway.
    */
    pub enum AzimuthPositionReference {
        /**
        Localizer/azimuth: Beyond stop end of runway.

        Back azimuth: Ahead of approach end of runway.
        */
        Variable1 = " ",

        /**
        Localizer/azimuth: Ahead of approach end of runway.

        Back azimuth: Beyond stop end of runway.
        */
        Variable2 = "+",

        RunwaySide = "-",
    }
}
