use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Damaged {
    pub current:  Cell<Rgb, Rgb>,
    pub previous: Cell<Rgb, Rgb>,
}

impl Damaged {
    pub fn new(cell: Cell<Rgb, Rgb>) -> Self {
        Self {
            current:  cell,
            previous: cell,
        }
    }
}

impl ICell for Damaged {
    fn cell(&self) -> Cell<Rgb, Rgb> {
        self.current
    }

    // fn cell_mut(&mut self) -> &mut Cell<Rgb, Rgb> {
    // &mut self.current
    // }

    fn damage(&self) -> Option<Cell<Rgb, Rgb>> {
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
    fn cell(&self) -> Cell<Rgb, Rgb> {
        self.current
    }

    // fn cell_mut(&mut self) -> &mut Cell<Rgb, Rgb> {
    // &mut self.current
    // }

    fn damage(&self) -> Option<Cell<Rgb, Rgb>> {
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

impl Over<&mut Cell<Rgb, Rgb>> for &Damaged {
    type Output = ();

    fn over(self, cell: &mut Cell<Rgb, Rgb>) {
        (&self.cell()).over(cell)
    }
}
