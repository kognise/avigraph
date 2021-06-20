use crate::num_parseable;

num_parseable! {
    /**
    "Stopway" means the length of an area beyond the take-off runway, no less wide than the
    runway and centered upon the extended centerline of the runway, and designated for use in
    decelerating the airplane during an aborted takeoff.

    The Stopway will be derived from official government sources and shown in feet.
    */
    pub Stopway u16: 4;
}
