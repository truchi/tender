use super::*;

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
impl<C: Over<Fg, NewFg> + Over<Bg, NewBg> + Clone, Fg, Bg, NewFg, NewBg>
    Over<Cell<Fg, Bg>, Cell<NewFg, NewBg>> for Color<C>
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
impl<C: Clone, Fg: Over<C, NewFg>, Bg: Over<C, NewBg>, NewFg, NewBg>
    Over<Color<C>, Cell<NewFg, NewBg>> for Cell<Fg, Bg>
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
{
    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Cell<NewFg, NewBg> {
        todo!()
    }
}

/*
impl<
        TopFg: Over<BottomBg> + Into<BottomFg> + PartialEq<TopBg>,
        TopBg: Color + Over<BottomFg> + Over<BottomBg> + Into<BottomBg>,
        BottomFg,
        BottomBg: Copy,
    > Over<Cell<BottomFg, BottomBg>> for Cell<TopFg, TopBg>
where
    <TopFg as Over<BottomBg>>::Output: Into<BottomFg>,
    <TopBg as Over<BottomFg>>::Output: Into<BottomFg>,
    <TopBg as Over<BottomBg>>::Output: Into<BottomBg>,
{
    type Output = Cell<BottomFg, BottomBg>;

    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Cell<BottomFg, BottomBg> {
        if self.styles.background.is_opaque() {
            Cell {
                char:   self.char,
                styles: Styles {
                    foreground: self.styles.foreground.into(),
                    background: self.styles.background.into(),
                    attributes: self.styles.attributes,
                },
            }
        } else if self.styles.foreground == self.styles.background {
            Cell {
                char:   bottom.char,
                styles: Styles {
                    foreground: self.styles.background.over(bottom.styles.foreground).into(),
                    background: self.styles.background.over(bottom.styles.background).into(),
                    attributes: bottom.styles.attributes,
                },
            }
        } else {
            Cell {
                char:   self.char,
                styles: Styles {
                    foreground: self.styles.foreground.over(bottom.styles.background).into(),
                    background: self.styles.background.over(bottom.styles.background).into(),
                    attributes: self.styles.attributes,
                },
            }
        }
    }
}
*/
