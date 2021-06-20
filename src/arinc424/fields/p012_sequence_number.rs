use crate::num_parseable;

num_parseable! {
    /**
    For Route Type Records - A route of flight is defined by a series of records taken in
    order. The "Sequence Number" field defines the location of the record in the sequence
    defining the route of flight identified in the route identifier field.

    For Boundary Type Records - A boundary is defined by a series of records taken in order.
    The "Sequence Number" field defines the location of the record in the sequence defining
    a boundary. For Record Types requiring more than one primary record to define the
    complete content – In a series of records used to define a complete condition, the
    "Sequence Number" is used to define each primary record in the sequence.

    For Airport and Heliport TAA Records – Sequence Number 1 will always be assigned to the
    record based on the Center Fix upon which the Straight-In Area is predicated, Sequence
    Number 2 will always be assigned to the record based on the Center Fix upon which the
    Left Base Area is predicated, and Sequence Number 3 will always be assigned to the record
    based on the Center Fix upon which the Right Base Area is predicated. Therefore, if a TAA
    Record has a Straight-In Area and a Right Base Area, but no Left Base Area, only Sequence
    Numbers 1 and 3 will be used. If a TAA Record has a Straight-In Area and a Left Base Area
    but no Right Base Area, only Sequence Numbers 1 and 2 will be used.

    Sequence numbers are assigned during the route, boundary or sequence definition phase of
    the data file assembly. Sequence numbers are assigned so as not be duplicated within the
    route, boundary or sequence assigned a unique identification/designation.

    For three or four digit Sequence Numbers, initially, an increment of ten should be maintained
    between the sequence numbers assigned to consecutive records.

    In route or boundary records, should subsequent maintenance of the file necessitate the
    addition of a record or records, the new record(s) should be located in the correct position
    in the sequence and assigned a sequence number whose most significant characters are identical
    to those in the sequence number of the preceding record in sequence. The unit character
    should be assigned a value midway between the units character values of the preceding and
    following record sequence numbers. For example, if it is desired to add one record to the
    sequence and the units characters of both the preceding and following records at the desired
    location are zeros (indicating no previous modification at this point), the units character or
    the inserted record's sequence number should be five (5).

    When an enroute airway crosses the boundary separating two geographical areas (Section 5.3), the
    airway fix lying on or closest to the boundary shall be coded twice, once for each geographical
    area, and should be assigned the same sequence number in each case. Record uniqueness in such cases
    is maintained through the "Boundary Code" (Section 5.18). Enroute airway record sequence numbers
    should be assigned in a manner which permits them to be arranged into continuous airway routes in
    flight sequence order when sorted according to the Route Identifier and Sequence Number only,
    without regard to their applicable Geographical Area Code.

    *This is a split field:*

    - *`TripleSequenceNumber` contains 3-length numbers (SID/STAR/Approach and Company Routes)*
    - *`DualSequenceNumber` contains 2-length numbers (VHF Navaid Limitation Continuation Records)*
    - *`SingleSequenceNumber` contains 1-length numbers (MSA Table, TAA Table, Cruise Table)*
    */
    pub QuadSequenceNumber u16: 4;
}

num_parseable! {
    /**
    For Route Type Records - A route of flight is defined by a series of records taken in
    order. The "Sequence Number" field defines the location of the record in the sequence
    defining the route of flight identified in the route identifier field.

    For Boundary Type Records - A boundary is defined by a series of records taken in order.
    The "Sequence Number" field defines the location of the record in the sequence defining
    a boundary. For Record Types requiring more than one primary record to define the
    complete content – In a series of records used to define a complete condition, the
    "Sequence Number" is used to define each primary record in the sequence.

    For Airport and Heliport TAA Records – Sequence Number 1 will always be assigned to the
    record based on the Center Fix upon which the Straight-In Area is predicated, Sequence
    Number 2 will always be assigned to the record based on the Center Fix upon which the
    Left Base Area is predicated, and Sequence Number 3 will always be assigned to the record
    based on the Center Fix upon which the Right Base Area is predicated. Therefore, if a TAA
    Record has a Straight-In Area and a Right Base Area, but no Left Base Area, only Sequence
    Numbers 1 and 3 will be used. If a TAA Record has a Straight-In Area and a Left Base Area
    but no Right Base Area, only Sequence Numbers 1 and 2 will be used.

    Sequence numbers are assigned during the route, boundary or sequence definition phase of
    the data file assembly. Sequence numbers are assigned so as not be duplicated within the
    route, boundary or sequence assigned a unique identification/designation.

    For three or four digit Sequence Numbers, initially, an increment of ten should be maintained
    between the sequence numbers assigned to consecutive records.

    In route or boundary records, should subsequent maintenance of the file necessitate the
    addition of a record or records, the new record(s) should be located in the correct position
    in the sequence and assigned a sequence number whose most significant characters are identical
    to those in the sequence number of the preceding record in sequence. The unit character
    should be assigned a value midway between the units character values of the preceding and
    following record sequence numbers. For example, if it is desired to add one record to the
    sequence and the units characters of both the preceding and following records at the desired
    location are zeros (indicating no previous modification at this point), the units character or
    the inserted record's sequence number should be five (5).

    When an enroute airway crosses the boundary separating two geographical areas (Section 5.3), the
    airway fix lying on or closest to the boundary shall be coded twice, once for each geographical
    area, and should be assigned the same sequence number in each case. Record uniqueness in such cases
    is maintained through the "Boundary Code" (Section 5.18). Enroute airway record sequence numbers
    should be assigned in a manner which permits them to be arranged into continuous airway routes in
    flight sequence order when sorted according to the Route Identifier and Sequence Number only,
    without regard to their applicable Geographical Area Code.

    *This is a split field:*

    - *`QuadSequenceNumber` contains 4-length numbers (Enroute Airways, Preferred Routes, FIR/UIR and
      Restrictive Airspace)*
    - *`DualSequenceNumber` contains 2-length numbers (VHF Navaid Limitation Continuation Records)*
    - *`SingleSequenceNumber` contains 1-length numbers (MSA Table, TAA Table, Cruise Table)*
    */
    pub TripleSequenceNumber u16: 3;
}

num_parseable! {
    /**
    For Route Type Records - A route of flight is defined by a series of records taken in
    order. The "Sequence Number" field defines the location of the record in the sequence
    defining the route of flight identified in the route identifier field.

    For Boundary Type Records - A boundary is defined by a series of records taken in order.
    The "Sequence Number" field defines the location of the record in the sequence defining
    a boundary. For Record Types requiring more than one primary record to define the
    complete content – In a series of records used to define a complete condition, the
    "Sequence Number" is used to define each primary record in the sequence.

    For Airport and Heliport TAA Records – Sequence Number 1 will always be assigned to the
    record based on the Center Fix upon which the Straight-In Area is predicated, Sequence
    Number 2 will always be assigned to the record based on the Center Fix upon which the
    Left Base Area is predicated, and Sequence Number 3 will always be assigned to the record
    based on the Center Fix upon which the Right Base Area is predicated. Therefore, if a TAA
    Record has a Straight-In Area and a Right Base Area, but no Left Base Area, only Sequence
    Numbers 1 and 3 will be used. If a TAA Record has a Straight-In Area and a Left Base Area
    but no Right Base Area, only Sequence Numbers 1 and 2 will be used.

    Sequence numbers are assigned during the route, boundary or sequence definition phase of
    the data file assembly. Sequence numbers are assigned so as not be duplicated within the
    route, boundary or sequence assigned a unique identification/designation.

    For one or two digit Sequence Numbers, initially, an increment of one should be maintained
    between the sequence numbers assigned to consecutive records.

    In route or boundary records, should subsequent maintenance of the file necessitate the
    addition of a record or records, the new record(s) should be located in the correct position
    in the sequence and assigned a sequence number whose most significant characters are identical
    to those in the sequence number of the preceding record in sequence. The unit character
    should be assigned a value midway between the units character values of the preceding and
    following record sequence numbers. For example, if it is desired to add one record to the
    sequence and the units characters of both the preceding and following records at the desired
    location are zeros (indicating no previous modification at this point), the units character or
    the inserted record's sequence number should be five (5).

    For records taken in sequence with one or two digit sequence numbers, additional data must be
    entered in the proper sequence and all subsequent records will be up numbered accordingly.

    When an enroute airway crosses the boundary separating two geographical areas (Section 5.3), the
    airway fix lying on or closest to the boundary shall be coded twice, once for each geographical
    area, and should be assigned the same sequence number in each case. Record uniqueness in such cases
    is maintained through the "Boundary Code" (Section 5.18). Enroute airway record sequence numbers
    should be assigned in a manner which permits them to be arranged into continuous airway routes in
    flight sequence order when sorted according to the Route Identifier and Sequence Number only,
    without regard to their applicable Geographical Area Code.

    *This is a split field:*

    - *`QuadSequenceNumber` contains 4-length numbers (Enroute Airways, Preferred Routes, FIR/UIR and
      Restrictive Airspace)*
    - *`TripleSequenceNumber` contains 3-length numbers (SID/STAR/Approach and Company Routes)*
    - *`SingleSequenceNumber` contains 1-length numbers (MSA Table, TAA Table, Cruise Table)*
    */
    pub DualSequenceNumber u16: 2;
}

num_parseable! {
    /**
    For Route Type Records - A route of flight is defined by a series of records taken in
    order. The "Sequence Number" field defines the location of the record in the sequence
    defining the route of flight identified in the route identifier field.

    For Boundary Type Records - A boundary is defined by a series of records taken in order.
    The "Sequence Number" field defines the location of the record in the sequence defining
    a boundary. For Record Types requiring more than one primary record to define the
    complete content – In a series of records used to define a complete condition, the
    "Sequence Number" is used to define each primary record in the sequence.

    For Airport and Heliport TAA Records – Sequence Number 1 will always be assigned to the
    record based on the Center Fix upon which the Straight-In Area is predicated, Sequence
    Number 2 will always be assigned to the record based on the Center Fix upon which the
    Left Base Area is predicated, and Sequence Number 3 will always be assigned to the record
    based on the Center Fix upon which the Right Base Area is predicated. Therefore, if a TAA
    Record has a Straight-In Area and a Right Base Area, but no Left Base Area, only Sequence
    Numbers 1 and 3 will be used. If a TAA Record has a Straight-In Area and a Left Base Area
    but no Right Base Area, only Sequence Numbers 1 and 2 will be used.

    Sequence numbers are assigned during the route, boundary or sequence definition phase of
    the data file assembly. Sequence numbers are assigned so as not be duplicated within the
    route, boundary or sequence assigned a unique identification/designation.

    For one or two digit Sequence Numbers, initially, an increment of one should be maintained
    between the sequence numbers assigned to consecutive records.

    In route or boundary records, should subsequent maintenance of the file necessitate the
    addition of a record or records, the new record(s) should be located in the correct position
    in the sequence and assigned a sequence number whose most significant characters are identical
    to those in the sequence number of the preceding record in sequence. The unit character
    should be assigned a value midway between the units character values of the preceding and
    following record sequence numbers. For example, if it is desired to add one record to the
    sequence and the units characters of both the preceding and following records at the desired
    location are zeros (indicating no previous modification at this point), the units character or
    the inserted record's sequence number should be five (5).

    For records taken in sequence with one or two digit sequence numbers, additional data must be
    entered in the proper sequence and all subsequent records will be up numbered accordingly.

    When an enroute airway crosses the boundary separating two geographical areas (Section 5.3), the
    airway fix lying on or closest to the boundary shall be coded twice, once for each geographical
    area, and should be assigned the same sequence number in each case. Record uniqueness in such cases
    is maintained through the "Boundary Code" (Section 5.18). Enroute airway record sequence numbers
    should be assigned in a manner which permits them to be arranged into continuous airway routes in
    flight sequence order when sorted according to the Route Identifier and Sequence Number only,
    without regard to their applicable Geographical Area Code.

    *This is a split field:*

    - *`QuadSequenceNumber` contains 4-length numbers (Enroute Airways, Preferred Routes, FIR/UIR and
      Restrictive Airspace)*
    - *`TripleSequenceNumber` contains 3-length numbers (SID/STAR/Approach and Company Routes)*
    - *`DualSequenceNumber` contains 2-length numbers (VHF Navaid Limitation Continuation Records)*
    */
    pub SingleSequenceNumber u16: 1;
}
