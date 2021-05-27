use super::*;
use std::fmt::{self, Display, Formatter};

/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

    fn styles(
        &self,
    ) -> (
        Foreground<Fg>,
        Background<Bg>,
        Weight,
        Slant,
        Underline,
        Strike,
    )
    where
        Fg: Copy,
        Bg: Copy,
    {
        (
            Foreground(self.foreground),
            Background(self.background),
            self.attributes.get_weight(),
            self.attributes.get_slant(),
            self.attributes.get_underline(),
            self.attributes.get_strike(),
        )
    }
}

impl<Fg: Default, Bg: Default> Default for Cell<Fg, Bg> {
    fn default() -> Self {
        Self {
            char:       ' ',
            foreground: Default::default(),
            background: Default::default(),
            attributes: Default::default(),
        }
    }
}

impl<Fg: Color> Display for CS<Cell<Fg, Rgb>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Cell {
            foreground,
            background,
            attributes,
            ..
        } = self.0;

        write!(
            f,
            "{}",
            CS((
                (Foreground(foreground.over(background))),
                (Background(background)),
                attributes,
            ))
        )
    }
}

impl Display for CS<Dedup<Cell>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Dedup(previous, current) = self.0;

        CS(Dedup(previous.styles(), current.styles())).fmt(f)
    }
}

impl<Fg: Color> Display for Cell<Fg, Rgb> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", CSI(*self), self.char)
    }
}

impl Display for Dedup<Cell> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", CSI(*self), self.1.char)
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

impl<Fg, Bg> Over<&mut Damaged> for Cell<Fg, Bg>
where
    Fg: Color,
    Bg: Color,
    Cell<Fg, Bg>: Into<Comp>,
{
    type Output = ();

    fn over(self, damaged: &mut Damaged) {
        *damaged = self.over(*damaged);
    }
}

impl<Fg, Bg> AsRef<Cell<Fg, Bg>> for Cell<Fg, Bg> {
    fn as_ref(&self) -> &Cell<Fg, Bg> {
        self
    }
}
