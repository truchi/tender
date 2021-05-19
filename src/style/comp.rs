use super::*;

/// A terminal `Cell`, composited.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Comp {
    pub(super) char:       char,
    pub(super) foreground: PreRgba,
    pub(super) background: PreRgba,
    pub(super) attributes: AttributesU8,
}

impl Comp {
    pub fn drop_alpha(self) -> Cell {
        debug_assert!(self.foreground.is_opaque());
        debug_assert!(self.background.is_opaque());

        Cell {
            char:       self.char,
            foreground: self.foreground.drop_alpha(),
            background: self.background.drop_alpha(),
            attributes: self.attributes,
        }
    }
}

impl<Fg: Color, Bg: Color> From<Cell<Fg, Bg>> for Comp {
    fn from(cell: Cell<Fg, Bg>) -> Self {
        let foreground: PreRgba = cell.foreground.into();
        let background: PreRgba = cell.background.into();

        Self {
            char: cell.char,
            foreground: foreground.over(background),
            background,
            attributes: cell.attributes,
        }
    }
}

impl Over<Comp> for PreRgba {
    type Output = Comp;

    fn over(self, comp: Comp) -> Self::Output {
        Comp {
            char:       comp.char,
            foreground: self.over(comp.foreground).into(),
            background: self.over(comp.background).into(),
            attributes: comp.attributes,
        }
    }
}

impl Over<PreRgba> for Comp {
    type Output = Comp;

    fn over(self, color: PreRgba) -> Self::Output {
        Comp {
            char:       self.char,
            foreground: self.foreground.over(color).into(),
            background: self.background.over(color).into(),
            attributes: self.attributes,
        }
    }
}

impl Over<Comp> for Comp {
    type Output = Comp;

    fn over(self, bottom: Comp) -> Comp {
        if self.background.is_opaque() {
            debug_assert!(self.foreground.is_opaque());
            self
        } else if self.foreground == self.background {
            self.background.over(bottom)
        } else {
            self.over(bottom.background)
        }
    }
}

impl<Fg: Color, Bg: Color> Over<Cell<Fg, Bg>> for Comp {
    type Output = Comp;

    fn over(self, bottom: Cell<Fg, Bg>) -> Comp {
        self.over(Comp::from(bottom))
    }
}

impl Over<&mut Cell> for &Comp {
    type Output = ();

    fn over(self, bottom: &mut Cell) {
        *bottom = (*self).over(*bottom).drop_alpha();
    }
}

impl Over<Damaged> for Comp {
    type Output = Damaged;

    fn over(self, mut bottom: Damaged) -> Damaged {
        (&self).over(&mut bottom.current);
        bottom
    }
}
