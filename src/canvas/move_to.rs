use super::*;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct MoveTo {
    initial: Point,
    current: Point,
}

impl MoveTo {
    pub fn new(initial: Point) -> Self {
        Self {
            initial,
            current: initial,
        }
    }

    pub fn first_col(&mut self) {
        self.current.x = self.initial.x;
    }

    pub fn first_row(&mut self) {
        self.current.y = self.initial.y;
    }

    pub fn next_col(&mut self) {
        self.current.x += 1;
    }

    pub fn next_row(&mut self) {
        self.current.y += 1;
    }
}

impl Display for MoveTo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.current.y + 1, self.current.x + 1)
    }
}
