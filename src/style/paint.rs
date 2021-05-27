use super::*;

pub trait Paint: Sized {
    type Output;

    fn paint(self, painter: impl Painter) -> Self::Output;

    fn char(self, char: char) -> Self::Output {
        self.paint(char)
    }

    fn foreground<C: Color>(self, foreground: C) -> Self::Output {
        self.paint(Foreground(foreground))
    }

    fn background<C: Color>(self, background: C) -> Self::Output {
        self.paint(Background(background))
    }

    fn attributes(self, attributes: Attributes) -> Self::Output {
        self.paint(attributes)
    }

    fn weight(self, weight: Weight) -> Self::Output {
        self.paint(weight)
    }

    fn bold(self) -> Self::Output {
        self.paint(Bold)
    }

    fn light(self) -> Self::Output {
        self.paint(Light)
    }

    fn no_weight(self) -> Self::Output {
        self.paint(NoWeight)
    }

    fn slant(self, slant: Slant) -> Self::Output {
        self.paint(slant)
    }

    fn italic(self) -> Self::Output {
        self.paint(Italic)
    }

    fn no_slant(self) -> Self::Output {
        self.paint(NoSlant)
    }

    fn underline(self, underline: Underline) -> Self::Output {
        self.paint(underline)
    }

    fn underlined(self) -> Self::Output {
        self.paint(Underlined)
    }

    fn no_underline(self) -> Self::Output {
        self.paint(NoUnderline)
    }

    fn strike(self, strike: Strike) -> Self::Output {
        self.paint(strike)
    }

    fn striked(self) -> Self::Output {
        self.paint(Striked)
    }

    fn no_strike(self) -> Self::Output {
        self.paint(NoStrike)
    }
}

impl Paint for Cell {
    type Output = Self;

    fn paint(mut self, painter: impl Painter) -> Self {
        painter.paint(&mut self);
        self
    }
}

impl Paint for &mut Cell {
    type Output = ();

    fn paint(self, painter: impl Painter) {
        painter.paint(self);
    }
}

impl Paint for char {
    type Output = Cell;

    fn paint(self, painter: impl Painter) -> Cell {
        Cell::default().char(self).paint(painter)
    }
}

pub trait Painter: Copy {
    fn paint(self, cell: &mut Cell);
}

impl Painter for char {
    fn paint(self, cell: &mut Cell) {
        cell.char = self;
    }
}

impl<C: Color> Painter for Foreground<C> {
    fn paint(self, cell: &mut Cell) {
        cell.foreground = self.0.over(cell.background);
    }
}

impl<C: Color> Painter for Background<C> {
    fn paint(self, cell: &mut Cell) {
        cell.background = self.0.over(cell.background);
    }
}

impl Painter for AttributesU8 {
    fn paint(self, cell: &mut Cell) {
        cell.attributes = self;
    }
}

impl Painter for Attributes {
    fn paint(self, cell: &mut Cell) {
        cell.attributes = self.into();
    }
}

impl Painter for Weight {
    fn paint(self, cell: &mut Cell) {
        cell.attributes.set_weight(self);
    }
}

impl Painter for Slant {
    fn paint(self, cell: &mut Cell) {
        cell.attributes.set_slant(self);
    }
}

impl Painter for Underline {
    fn paint(self, cell: &mut Cell) {
        cell.attributes.set_underline(self);
    }
}

impl Painter for Strike {
    fn paint(self, cell: &mut Cell) {
        cell.attributes.set_strike(self);
    }
}

macro_rules! tuples {
    ($([$($field:tt $P:ident)*])*) => { $(
        impl<$($P,)*> Painter for ($($P,)*)
        where
            $($P: Painter,)*
        {
            fn paint(self, cell: &mut Cell) {
                $(self.$field.paint(cell);)*
            }
        }
    )* };
}

tuples!(
    [0 P1 1 P2]
    [0 P1 1 P2 2 P3]
    [0 P1 1 P2 2 P3 3 P4]
    [0 P1 1 P2 2 P3 3 P4 4 P5]
    [0 P1 1 P2 2 P3 3 P4 4 P5 5 P6]
    [0 P1 1 P2 2 P3 3 P4 4 P5 5 P6 6 P7]
);
