use crate::canvas::*;
use std::{
    cell::Cell as StdCell,
    fmt::{self, Display, Formatter},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg = Fg> {
    pub char:   char,
    pub styles: Styles<Fg, Bg>,
}

impl<Fg, Bg> Cell<Fg, Bg> {
    pub fn cast<T, U>(self) -> Cell<T, U>
    where
        Fg: Into<T>,
        Bg: Into<U>,
    {
        Cell {
            char:   self.char,
            styles: self.styles.cast::<T, U>(),
        }
    }

    pub fn set_foreground<NewFg>(self, foreground: NewFg) -> Cell<NewFg, Bg> {
        Cell {
            char:   self.char,
            styles: self.styles.set_foreground(foreground),
        }
    }

    pub fn set_background<NewBg>(self, background: NewBg) -> Cell<Fg, NewBg> {
        Cell {
            char:   self.char,
            styles: self.styles.set_background(background),
        }
    }
}

impl Cell<Rgb> {
    /// Applies `color` over `Foreground` and `Background`.
    pub fn color(self, color: PreRgba) -> Self {
        Self {
            styles: self.styles.color(color),
            ..self
        }
    }
}

impl Cell<PreRgba, Rgb> {
    /// Resolves `Foreground` to `Rgb` from `Background`.
    pub fn resolve(self) -> Cell<Rgb> {
        Cell {
            char:   self.char,
            styles: self.styles.resolve(),
        }
    }
}

impl Cell<PreRgba> {
    /// Places `self` over `other`.
    pub fn over(self, other: Cell<Rgb>) -> Cell<Rgb> {
        let foreground = self.styles.foreground;
        let background = self.styles.background;

        // When self has opaque background, other is invisible
        if background.is_opaque() {
            self.cast::<PreRgba, Rgb>().resolve()
        }
        // Otherwise, we see through self's background
        else {
            // If self's char is invisible
            if foreground.is_invisible() {
                // Apply self's background over other
                other.color(background.0)
            }
            // If self's char is visible
            else {
                // Merge backgrounds in self
                let background = self.get_background().over(other.get_background().0);
                self.set_background(background).resolve()
            }
        }
    }
}

macro_rules! styler {
    ($($get:ident $set:ident $attr:ident: $Attr:ident)*) => { $(
        fn $get(self) -> $Attr {
            self.styles.$get()
        }

        fn $set(self, $attr: $Attr) -> Self {
            Self {
                styles: self.styles.$set($attr),
                ..self
            }
        }
    )* };
}

impl<Fg, Bg> Styler<Fg, Bg> for Cell<Fg, Bg> {
    styler!(
        get_weight     set_weight     weight:     Weight
        get_slant      set_slant      slant:      Slant
        get_underline  set_underline  underline:  Underline
        get_strike     set_strike     strike:     Strike
        get_overline   set_overline   overline:   Overline
        get_invert     set_invert     invert:     Invert
        get_blink      set_blink      blink:      Blink
        get_border     set_border     border:     Border
    );

    fn get_foreground(self) -> Foreground<Fg> {
        self.styles.get_foreground()
    }

    fn get_background(self) -> Background<Bg> {
        self.styles.get_background()
    }

    fn get_attributes(self) -> Attributes {
        self.styles.get_attributes()
    }

    fn set_foreground(self, color: Fg) -> Self {
        Self {
            styles: self.styles.set_foreground(color),
            ..self
        }
    }

    fn set_background(self, color: Bg) -> Self {
        Self {
            styles: self.styles.set_background(color),
            ..self
        }
    }

    fn set_attributes(self, attributes: Attributes) -> Self {
        Self {
            styles: self.styles.set_attributes(attributes),
            ..self
        }
    }
}

impl Display for Cell<Rgb> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { char, styles } = self;

        write!(f, "{}{}", styles, char)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct DamageCell {
    pub new: Cell<Rgb>,
    pub old: Cell<Rgb>,
}

impl DamageCell {
    pub fn over((below, above): (&mut Self, Cell<PreRgba>)) {
        below.new = above.over(below.new);
    }
}

impl From<Cell<Rgb>> for DamageCell {
    fn from(cell: Cell<Rgb>) -> Self {
        DamageCell {
            new: cell,
            old: cell,
        }
    }
}
