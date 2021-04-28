use super::*;
use std::fmt::{self, Display, Formatter};

/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg = Fg> {
    char:       char,
    foreground: Color<Fg>,
    background: Color<Bg>,
    attributes: Attributes,
}

impl<Fg, Bg> Cell<Fg, Bg> {
    pub fn new<T>(char: char, foreground: T, background: Bg, attributes: Attributes) -> Self
    where
        T: Over<Bg, Fg>,
        Bg: Clone,
    {
        Self {
            char,
            foreground: Color(foreground.over(background.clone())),
            background: Color(background),
            attributes,
        }
    }

    pub fn cast<T, U>(self) -> Cell<T, U>
    where
        Fg: Into<T>,
        Bg: Into<U>,
    {
        Cell {
            char:       self.char,
            foreground: Color(self.foreground.0.into()),
            background: Color(self.background.0.into()),
            attributes: self.attributes,
        }
    }
}

// color OVER cell
impl<C, Fg, Bg, NewFg, NewBg> Over<Cell<Fg, Bg>, Cell<NewFg, NewBg>> for Color<C>
where
    C: Over<Fg, NewFg> + Over<Bg, NewBg> + Clone,
{
    fn over(self, cell: Cell<Fg, Bg>) -> Cell<NewFg, NewBg> {
        Cell {
            char:       cell.char,
            foreground: self.clone().over(cell.foreground),
            background: self.over(cell.background),
            attributes: cell.attributes,
        }
    }
}

// cell OVER color
impl<C, Fg, Bg, NewFg, NewBg> Over<Color<C>, Cell<NewFg, NewBg>> for Cell<Fg, Bg>
where
    C: Clone,
    Fg: Over<C, NewFg>,
    Bg: Over<C, NewBg>,
{
    fn over(self, color: Color<C>) -> Cell<NewFg, NewBg> {
        Cell {
            char:       self.char,
            foreground: self.foreground.over(color.clone()),
            background: self.background.over(color),
            attributes: self.attributes,
        }
    }
}

// cell OVER cell
impl<TopFg, TopBg, BottomFg, BottomBg, NewFg, NewBg>
    Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>> for Cell<TopFg, TopBg>
where
    TopFg: Into<NewFg> + PartialEq<TopBg>,
    TopBg: Into<NewBg> + WithAlpha,
    BottomBg: Clone,
    TopFg: Over<BottomBg, NewFg>,
    TopBg: Over<BottomFg, NewFg> + Over<BottomBg, NewBg>,
{
    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Cell<NewFg, NewBg> {
        if self.background.is_opaque() {
            self.cast()
        } else if self.foreground == self.background {
            self.background.over(bottom)
        } else {
            self.over(bottom.background)
        }
    }
}

impl Display for Cell<Rgb> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Foreground(self.foreground.0),
            Background(self.background.0),
            self.attributes,
            self.char,
        )
    }
}
