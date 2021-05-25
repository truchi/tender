use super::*;

pub trait Paint {
    fn paint(&mut self, painter: impl Painter);

    fn char(&mut self, char: char) {
        self.paint(char);
    }

    fn foreground<C: Color>(&mut self, foreground: C) {
        self.paint(Foreground(foreground));
    }

    fn background<C: Color>(&mut self, background: C) {
        self.paint(Background(background));
    }

    fn attributes(&mut self, attributes: Attributes) {
        self.paint(attributes);
    }

    fn weight(&mut self, weight: Weight) {
        self.paint(weight);
    }

    fn bold(&mut self) {
        self.paint(Bold);
    }

    fn light(&mut self) {
        self.paint(Light);
    }

    fn no_weight(&mut self) {
        self.paint(NoWeight);
    }

    fn slant(&mut self, slant: Slant) {
        self.paint(slant);
    }

    fn italic(&mut self) {
        self.paint(Italic);
    }

    fn no_slant(&mut self) {
        self.paint(NoSlant);
    }

    fn underline(&mut self, underline: Underline) {
        self.paint(underline);
    }

    fn underlined(&mut self) {
        self.paint(Underlined);
    }

    fn no_underline(&mut self) {
        self.paint(NoUnderline);
    }

    fn strike(&mut self, strike: Strike) {
        self.paint(strike);
    }

    fn striked(&mut self) {
        self.paint(Striked);
    }

    fn no_strike(&mut self) {
        self.paint(NoStrike);
    }
}

impl Paint for Cell {
    fn paint(&mut self, painter: impl Painter) {
        painter.paint(self);
    }
}

pub trait Painter {
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
