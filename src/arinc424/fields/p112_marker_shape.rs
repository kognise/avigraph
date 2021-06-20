use crate::enum_parseable;

enum_parseable! {
    /**
    The "Shape" field defines the radiation pattern of an airways marker as being either
    "bone" or "elliptical."

    The field contains the shape of the marker derived from official government sources when
    available. The character "B" will designate the "bone" shape and the character "E" will
    designate the elliptical shape. "E" will be entered when the source does not supply shape
    information.
    */
    pub enum MarkerShape {
        Bone = "B",
        Elliptical = "E",
        None = " ",
    }
}
