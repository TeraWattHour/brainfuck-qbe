use crate::operation::Operation;
use std::str::Bytes;

pub(crate) struct Cursor<'a> {
    bytes: Bytes<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new<'b: 'a>(content: &'a str) -> Self {
        Self {
            bytes: content.bytes(),
        }
    }

    fn operator(b: u8) -> Option<Operation> {
        match b {
            b'+' => Some(Operation::IncrementByte),
            b'-' => Some(Operation::DecrementByte),
            b'>' => Some(Operation::IncrementPointer),
            b'<' => Some(Operation::DecrementPointer),
            b'.' => Some(Operation::Output),
            b'[' => Some(Operation::JumpIfZero),
            b']' => Some(Operation::JumpIfNonZero),
            _ => None,
        }
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Operation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.len() == 0 {
            return None;
        }

        while let Some(b) = self.bytes.next() {
            match Self::operator(b) {
                Some(operator) => return Some(operator),
                None => continue,
            }
        }

        None
    }
}
