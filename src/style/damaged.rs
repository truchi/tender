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

impl AsRef<Cell> for Damaged {
    fn as_ref(&self) -> &Cell {
        &self.current
    }
}

impl AsMut<Cell> for Damaged {
    fn as_mut(&mut self) -> &mut Cell {
        &mut self.current
    }
}

impl AsRef<Damaged> for Damaged {
    fn as_ref(&self) -> &Damaged {
        self
    }
}

impl AsMut<Damaged> for Damaged {
    fn as_mut(&mut self) -> &mut Damaged {
        self
    }
}
