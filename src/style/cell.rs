use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg = Fg> {
    pub char:   char,
    pub styles: Styles<Fg, Bg>,
}

impl<
        TopFg: PartialEq<TopBg> + Into<BottomFg> + Over<BottomBg>,
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
