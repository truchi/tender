use crate::style::*;
use std::{
    fmt::{self, Display, Formatter},
    io::Write,
};

/// `Styles`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Styles<Fg, Bg = Fg> {
    pub foreground: Fg,
    pub background: Bg,
    pub attributes: Attributes,
}

impl Styles<PreRgba> {
    pub fn new(foreground: PreRgba, background: PreRgba, attributes: Attributes) -> Self {
        Self {
            foreground: foreground.over(background),
            background,
            attributes,
        }
    }
}

// impl Over for Styles

// =====================================================================================

impl<Fg, Bg> Styles<Fg, Bg> {
    pub fn cast<NewFg, NewBg>(self) -> Styles<NewFg, NewBg>
    where
        Fg: Into<NewFg>,
        Bg: Into<NewBg>,
    {
        Styles {
            foreground: self.foreground.into(),
            background: self.background.into(),
            attributes: self.attributes,
        }
    }

    pub fn set_foreground<NewFg>(self, foreground: NewFg) -> Styles<NewFg, Bg> {
        Styles {
            foreground,
            background: self.background,
            attributes: self.attributes,
        }
    }

    pub fn set_background<NewBg>(self, background: NewBg) -> Styles<Fg, NewBg> {
        Styles {
            foreground: self.foreground,
            background,
            attributes: self.attributes,
        }
    }
}

impl Styles<Rgb> {
    /// Applies `color` over `Foreground` and `Background`.
    pub fn color(self, color: PreRgba) -> Self {
        self.set_foreground(color.over(self.foreground))
            .set_background(color.over(self.background))
    }

    pub fn render_dedup<T: Write>(self, w: &mut T, prev: &Self) {
        macro_rules! dedup {
            (
                colors $($color:ident)*,
                attributes $($attr:ident)*,
            ) => {
                $(if self.$color != prev.$color {
                    write!(w, "{}", self.$color).unwrap();
                })*
                $(if self.attributes.$attr != prev.attributes.$attr {
                    write!(w, "{}", self.attributes.$attr).unwrap();
                })*
            };
        }

        dedup!(
            colors
                foreground background,
            attributes
                weight slant underline strike overline invert blink border,
        );
    }
}

impl Styles<PreRgba, Rgb> {
    /// Resolves `Foreground` to `Rgb` from `Background`.
    pub fn resolve(self) -> Styles<Rgb> {
        self.set_foreground(self.foreground.over(self.background))
    }
}

macro_rules! styler {
    ($($get:ident $set:ident $attr:ident: $Attr:ident)*) => { $(
        fn $get(self) -> $Attr {
            self.attributes.$attr
        }

        fn $set(self, $attr: $Attr) -> Self {
            Self {
                attributes: Attributes {
                    $attr,
                    ..self.attributes
                },
                ..self
            }
        }
    )* };
}

impl<Fg, Bg> Styler<Fg, Bg> for Styles<Fg, Bg> {
    styler!(
        get_weight    set_weight    weight:    Weight
        get_slant     set_slant     slant:     Slant
        get_underline set_underline underline: Underline
        get_strike    set_strike    strike:    Strike
        get_overline  set_overline  overline:  Overline
        get_invert    set_invert    invert:    Invert
        get_blink     set_blink     blink:     Blink
        get_border    set_border    border:    Border
    );

    fn get_foreground(self) -> Fg {
        self.foreground
    }

    fn get_background(self) -> Bg {
        self.background
    }

    fn get_attributes(self) -> Attributes {
        self.attributes
    }

    fn set_foreground(self, foreground: Fg) -> Self {
        Self { foreground, ..self }
    }

    fn set_background(self, background: Bg) -> Self {
        Self { background, ..self }
    }

    fn set_attributes(self, attributes: Attributes) -> Self {
        Self { attributes, ..self }
    }
}

impl Display for Styles<Rgb> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self {
            foreground,
            background,
            attributes,
        } = self;

        write!(f, "{}{}{}", foreground, background, attributes)
    }
}
