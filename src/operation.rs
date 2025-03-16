#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    IncrementPointer, // >
    DecrementPointer, // <
    IncrementByte,    // +
    DecrementByte,    // -
    Output,           // .
    JumpIfZero,       // [
    JumpIfNonZero,    // ]
}
