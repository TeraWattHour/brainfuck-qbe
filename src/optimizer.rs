use std::{cell::RefCell, iter::Peekable, rc::Rc};

use crate::{operation::Operation, Cursor};

pub struct Optimizer<'a> {
    cursor: Peekable<&'a mut Cursor<'a>>,
    label: Rc<RefCell<usize>>,
    open: RefCell<Vec<usize>>,
}

impl<'a> Optimizer<'a> {
    pub fn new(cursor: &'a mut Cursor<'a>) -> Self {
        Self {
            cursor: cursor.peekable(),
            label: Rc::new(RefCell::new(0)),
            open: RefCell::new(Vec::new()),
        }
    }
}

impl<'a> Iterator for Optimizer<'a> {
    type Item = Result<(Operation, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        use Operation::*;

        let operation = match self.cursor.next() {
            Some(op) => op,
            None => {
                if self.open.borrow().len() > 0 {
                    return Some(Err("unclosed bracket pair".into()));
                }
                return None;
            }
        };
        match operation {
            Output => return Some(Ok((operation, 1))),
            JumpIfZero => {
                let mut label = self.label.borrow_mut();
                *label += 1;
                self.open.borrow_mut().push(*label);
                return Some(Ok((operation, *label)));
            }
            JumpIfNonZero => {
                return Some(self.open.borrow_mut().pop().map_or_else(
                    || Err("unbalanced bracket pair".into()),
                    |label| Ok((operation, label)),
                ));
            }
            _ => (),
        }

        if matches!(operation, Output | JumpIfZero | JumpIfNonZero) {
            return Some(Ok((operation, 1)));
        }

        let mut repetitions = 1;
        while self.cursor.peek().is_some_and(|b| b == &operation) {
            repetitions += 1;
            self.cursor.next();
        }

        Some(Ok((operation, repetitions)))
    }
}
