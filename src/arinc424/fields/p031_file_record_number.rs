use crate::num_parseable;

num_parseable! {
    /**
    The "File Record Number" is a reference number assigned to the record for
    housekeeping purposes. Records are numbered consecutively, the first record
    on the file being assigned the number 00001, the second the number 00002,
    and so on through the final record on the file. File record numbers are
    subject to change at each file update.

    File record numbers are assigned to records during the assembly of the data
    file. If the file reaches 99999, the next record number will start over with
    00000.
    */
    pub FileRecordNumber u32: 5;
}
