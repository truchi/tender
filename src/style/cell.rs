use super::*;
use std::fmt::{self, Display, Formatter};

/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg = Rgb, Bg = Rgb> {
    pub char:       char,
    pub foreground: Fg,
    pub background: Bg,
    pub attributes: AttributesU8,
}

impl<Fg, Bg> Cell<Fg, Bg> {
    pub fn new(
        char: char,
        foreground: Fg,
        background: Bg,
        attributes: impl Into<Attributes>,
    ) -> Self {
        Self {
            char,
            foreground,
            background,
            attributes: attributes.into().into(),
        }
    }
}

impl<Fg: Color> Display for Cell<Fg, Rgb> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Foreground(self.foreground.over(self.background)),
            Background(self.background),
            self.attributes,
            self.char,
        )
    }
}

impl Display for Dedup<Cell> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Dedup(Foreground(self.0.foreground), Foreground(self.1.foreground)),
            Dedup(Background(self.0.background), Background(self.1.background)),
            Dedup(self.0.attributes, self.1.attributes),
            self.1.char,
        )
    }
}

impl<TopFg, TopBg, BottomFg, BottomBg> Over<Cell<BottomFg, BottomBg>> for Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Into<Comp>,
    Cell<BottomFg, BottomBg>: Into<Comp>,
{
    type Output = Comp;

    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Comp {
        self.into().over(bottom.into())
    }
}

impl<Fg, Bg> Over<&mut Cell> for &Cell<Fg, Bg>
where
    Fg: Color,
    Bg: Color,
    Cell<Fg, Bg>: Into<Comp>,
{
    type Output = ();

    fn over(self, bottom: &mut Cell) {
        (&(*self).into()).over(bottom);
    }
}

impl<Fg, Bg> Over<Damaged> for Cell<Fg, Bg>
where
    Fg: Color,
    Bg: Color,
    Cell<Fg, Bg>: Into<Comp>,
{
    type Output = Damaged;

    fn over(self, mut damaged: Damaged) -> Damaged {
        (&self).over(&mut damaged.current);
        damaged
    }
}

impl ICell for &Cell {
    fn cell(&self) -> Cell {
        **self
    }

    // fn cell_mut(&mut self) -> &mut Cell {
    // self
    // }

    fn damage(&self) -> Option<Cell> {
        Some(**self)
    }

    fn update(&mut self) {}
}
