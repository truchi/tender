use super::*;
use std::fmt::{self, Display, Formatter};

/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg = Fg> {
    char:       char,
    foreground: Color<Fg>,
    background: Color<Bg>,
    attributes: Attrs,
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
            attributes: attributes.into(),
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

// Color OVER Cell
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

// Cell OVER Color
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

// Cell OVER Cell
impl<TopFg, TopBg, BottomFg, BottomBg, NewFg, NewBg>
    Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>> for Cell<TopFg, TopBg>
where
    TopFg: Into<NewFg> + PartialEq<TopBg>,
    TopBg: Into<NewBg> + WithAlpha,
    Color<TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>>,
    Self: Over<Color<BottomBg>, Cell<NewFg, NewBg>>,
{
    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Cell<NewFg, NewBg> {
        if self.background.is_opaque() {
            self.cast()
        } else if self.foreground == self.background {
            self.background.over(bottom)
        } else {
            // full syntax or internal compilator error...
            <_ as Over<_, Cell<NewFg, NewBg>>>::over(self, bottom.background)
        }
    }
}

// Cell OVER &mut Cell
impl<'b, TopFg, TopBg, BottomFg, BottomBg> Over<&'b mut Cell<BottomFg, BottomBg>, ()>
    for Cell<TopFg, TopBg>
where
    Cell<BottomFg, BottomBg>: Clone,
    Cell<TopFg, TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<BottomFg, BottomBg>>,
{
    fn over(self, bottom: &'b mut Cell<BottomFg, BottomBg>) {
        *bottom = self.over(bottom.clone());
    }
}

// &Cell OVER &mut Cell
impl<'t, 'b, TopFg, TopBg, BottomFg, BottomBg> Over<&'b mut Cell<BottomFg, BottomBg>, ()>
    for &'t Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Clone,
    Cell<BottomFg, BottomBg>: Clone,
    Cell<TopFg, TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<BottomFg, BottomBg>>,
{
    fn over(self, bottom: &'b mut Cell<BottomFg, BottomBg>) {
        *bottom = self.clone().over(bottom.clone());
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
