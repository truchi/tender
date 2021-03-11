use crate::canvas::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg = Fg> {
    pub char: char,
    pub styles: Styles<Fg, Bg>,
}

impl<Fg, Bg> Cell<Fg, Bg> {
    pub fn cast<T, U>(self) -> Cell<T, U>
    where
        Fg: Into<T>,
        Bg: Into<U>,
    {
        Cell {
            char: self.char,
            styles: self.styles.cast::<T, U>(),
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
            char: self.char,
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
                // Replace self's background with other's
                Cell {
                    char: self.char,
                    styles: Styles {
                        foreground: self.styles.foreground,
                        background: other.styles.background,
                        attributes: self.styles.attributes,
                    }
                    .resolve(),
                }
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
    fn get_foreground(self) -> Foreground<Fg> {
        self.styles.get_foreground()
    }

    fn get_background(self) -> Background<Bg> {
        self.styles.get_background()
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

    styler!(
        get_attributes set_attributes attributes: Attributes
        get_weight     set_weight     weight:     Weight
        get_slant      set_slant      slant:      Slant
        get_underline  set_underline  underline:  Underline
        get_strike     set_strike     strike:     Strike
        get_overline   set_overline   overline:   Overline
        get_invert     set_invert     invert:     Invert
        get_blink      set_blink      blink:      Blink
        get_border     set_border     border:     Border
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct DamageCell {
    pub new: Cell<Rgb>,
    pub old: Cell<Rgb>,
}

impl DamageCell {
    pub fn new(self, new: Cell<Rgb>) -> Self {
        Self { new, old: self.old }
    }

    pub fn old(self, old: Cell<Rgb>) -> Self {
        Self { new: self.new, old }
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
