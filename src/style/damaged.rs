use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Damaged {
    pub current:  Cell,
    pub previous: Cell,
}

impl Damaged {
    pub fn new(cell: Cell) -> Self {
        Self {
            current:  cell,
            previous: cell,
        }
    }
}

impl ICell for Damaged {
    fn cell(&self) -> Cell {
        self.current
    }

    // fn cell_mut(&mut self) -> &mut Cell {
    // &mut self.current
    // }

    fn damage(&self) -> Option<Cell> {
        if self.current == self.previous {
            None
        } else {
            Some(self.current)
        }
    }

    fn update(&mut self) {
        self.previous = self.current;
    }
}

impl ICell for &Damaged {
    fn cell(&self) -> Cell {
        self.current
    }

    // fn cell_mut(&mut self) -> &mut Cell {
    // &mut self.current
    // }

    fn damage(&self) -> Option<Cell> {
        if self.current == self.previous {
            None
        } else {
            Some(self.current)
        }
    }

    fn update(&mut self) {
        // self.previous = self.current;
    }
}
