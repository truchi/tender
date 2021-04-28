use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg = Fg> {
    pub char:   char,
    pub styles: Styles<Fg, Bg>,
}

// color OVER cell
impl<C: Over<Fg, NewFg> + Over<Bg, NewBg> + Clone, Fg, Bg, NewFg, NewBg>
    Over<Cell<Fg, Bg>, Cell<NewFg, NewBg>> for ColorWrapper<C>
{
    fn over(self, cell: Cell<Fg, Bg>) -> Cell<NewFg, NewBg> {
        Cell {
            char:   cell.char,
            styles: Styles {
                foreground: self.0.clone().over(cell.styles.foreground),
                background: self.0.over(cell.styles.background),
                attributes: cell.styles.attributes,
            },
        }
    }
}

// cell OVER color
impl<C: Clone, Fg: Over<C, NewFg>, Bg: Over<C, NewBg>, NewFg, NewBg> Over<C, Cell<NewFg, NewBg>>
    for Cell<Fg, Bg>
{
    fn over(self, color: C) -> Cell<NewFg, NewBg> {
        Cell {
            char:   self.char,
            styles: Styles {
                foreground: self.styles.foreground.over(color.clone()),
                background: self.styles.background.over(color),
                attributes: self.styles.attributes,
            },
        }
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
