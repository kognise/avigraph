use crate::num_parseable;

num_parseable! {
    /**
    The "Speed Limit" field defines a speed, expressed in Knots, Indicated (K.I.A.S.), for a
    fix in a terminal procedure or for an airport or heliport terminal environment.

    The speed limit will be derived from official government source documentation and shown
    in Knots. When used on an Airport or Heliport Record, the field is an indication of the
    maximum allowed speed and applies to all flight segments departing or arriving that
    airport's or heliport's terminal area, at and below the specified Speed Limit Altitude
    (5.73). When used on Airport and Heliport SID/STAR/Approach Records, the field is an
    indication of a speed for a fix in the procedure description, used in conjunction with
    Speed Limit Description (5.261).

    On SID Procedure Records, the speed limit will apply to all legs up to and including the
    termination of the leg on which the speed is coded from the beginning of the procedure or
    a previous speed limit. If a different speed is coded on a subsequent leg, the limit will
    be applied for that leg and from that leg backwards to the previous terminator which
    contained a speed limit.

    On STAR and Approach Procedure Records, the speed limit will be applied forward to the
    end of the arrival (e.g. throughout the procedure until the end of the Flight Plan) or
    until superceded by another speed limit.

    The intent in both SIDs and STARs is to exclude speed changes inconsistent with the
    procedure.
    */
    pub SpeedLimit u16: 3;
}
