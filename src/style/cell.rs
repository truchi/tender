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

impl<Fg: Color, Bg: Color> Over<Comp> for Cell<Fg, Bg> {
    type Output = Comp;

    fn over(self, bottom: Comp) -> Comp {
        Comp::from(self).over(bottom)
    }
}

impl<TopFg, TopBg, BotFg, BotBg> Over<Cell<BotFg, BotBg>> for Cell<TopFg, TopBg>
where
    TopFg: Color,
    TopBg: Color,
    BotFg: Color,
    BotBg: Color,
{
    type Output = Comp;

    fn over(self, bottom: Cell<BotFg, BotBg>) -> Comp {
        Comp::from(self).over(Comp::from(bottom))
    }
}

impl<Fg, Bg> Over<&mut Cell> for &Cell<Fg, Bg>
where
    Fg: Color,
    Bg: Color,
{
    type Output = ();

    fn over(self, bottom: &mut Cell) {
        (&Comp::from(*self)).over(bottom)
    }
}

impl<Fg, Bg> Over<Damaged> for Cell<Fg, Bg>
where
    Fg: Color,
    Bg: Color,
    Cell<Fg, Bg>: Into<Comp>,
{
    type Output = Damaged;

    fn over(self, damaged: Damaged) -> Damaged {
        Comp::from(self).over(damaged)
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
