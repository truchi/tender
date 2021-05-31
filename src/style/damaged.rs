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

    pub fn damage(&mut self) -> Option<Cell> {
        let damage = if self.current == self.previous {
            None
        } else {
            Some(self.current)
        };

        self.previous = self.current;
        damage
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
