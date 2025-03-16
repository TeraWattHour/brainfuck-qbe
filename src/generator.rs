use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};

use crate::{operation::Operation, optimizer::Optimizer};

pub struct Generator<'a, 'b> {
    optimizer: &'a mut Optimizer<'a>,
    file: &'b mut File,
}

impl<'a, 'b> Generator<'a, 'b> {
    pub fn new(optimizer: &'a mut Optimizer<'a>, file: &'b mut File) -> Self {
        Self { optimizer, file }
    }

    const PREAMBLE: &'static [u8] = br#"
export function w $main() {
@start
    %ip =l copy $data
    %val =w copy 0
"#;

    const EPILOGUE: &'static [u8] = br#"
    ret 0
}

data $data = { z 10000000 }
"#;

    pub fn generate(&mut self) -> Result<(), Box<dyn Error>> {
        use Operation::*;

        self.file.write(&Self::PREAMBLE[1..])?;

        while let Some(Ok((op, repetitions))) = self.optimizer.next() {
            match op {
                IncrementPointer => self.increment_pointer(repetitions)?,
                DecrementPointer => self.decrement_pointer(repetitions)?,
                IncrementByte => self.increment_byte(repetitions)?,
                DecrementByte => self.decrement_byte(repetitions)?,
                Output => self.output()?,
                JumpIfZero => self.jump_if_zero(repetitions)?,
                JumpIfNonZero => self.jump_if_non_zero(repetitions)?,
            };
        }

        self.file.write(&Self::EPILOGUE[1..])?;

        Ok(())
    }

    fn increment_byte(&mut self, by: usize) -> io::Result<usize> {
        let instructions = format!("    %val =w add %val, {by}\n");
        self.file.write(instructions.as_bytes())
    }

    fn decrement_byte(&mut self, by: usize) -> io::Result<usize> {
        let instructions = format!("    %val =w sub %val, {by}\n");
        self.file.write(instructions.as_bytes())
    }

    fn increment_pointer(&mut self, by: usize) -> io::Result<usize> {
        self.file.write(
            format!("    storeb %val, %ip\n    %ip =l add %ip, {by}\n    %val =w loadsb %ip\n")
                .as_bytes(),
        )
    }

    fn decrement_pointer(&mut self, by: usize) -> io::Result<usize> {
        self.file.write(
            format!("    storeb %val, %ip\n    %ip =l sub %ip, {by}\n    %val =w loadsb %ip\n")
                .as_bytes(),
        )
    }

    // if %val == 0 (second argument) jump to after close
    fn jump_if_zero(&mut self, label: usize) -> io::Result<usize> {
        self.file.write(
            format!(
                "@open_{label}
    jnz %val, @open_after_{label}, @close_after_{label}
@open_after_{label}
"
            )
            .as_bytes(),
        )
    }

    // if %val != 0 (first argument) jump to after open
    fn jump_if_non_zero(&mut self, label: usize) -> io::Result<usize> {
        self.file.write(
            format!(
                "@close_{label}
    jnz %val, @open_after_{label}, @close_after_{label}
@close_after_{label}
"
            )
            .as_bytes(),
        )
    }

    fn output(&mut self) -> io::Result<usize> {
        self.file.write(
            &b"
    call $putchar(w %val)
"[1..],
        )
    }
}
