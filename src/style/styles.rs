use crate::style::*;

/// `Styles` ([`Foreground`](crate::style::Foreground),
/// [`Background`](crate::style::Background),
/// [`Attributes`](crate::style::Attributes)).
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Styles<Fg, Bg = Fg> {
    pub foreground: Foreground<Fg>,
    pub background: Background<Bg>,
    pub attributes: Attributes,
}

impl<Fg, Bg> Styles<Fg, Bg> {
    pub fn cast<NewFg, NewBg>(self) -> Styles<NewFg, NewBg>
    where
        Fg: Into<NewFg>,
        Bg: Into<NewBg>,
    {
        Styles {
            foreground: Foreground(self.foreground.0.into()),
            background: Background(self.background.0.into()),
            attributes: self.attributes,
        }
    }

    pub fn set_foreground<NewFg>(self, foreground: NewFg) -> Styles<NewFg, Bg> {
        Styles {
            foreground: Foreground(foreground),
            background: self.background,
            attributes: self.attributes,
        }
    }

    pub fn set_background<NewBg>(self, background: NewBg) -> Styles<Fg, NewBg> {
        Styles {
            foreground: self.foreground,
            background: Background(background),
            attributes: self.attributes,
        }
    }
}

impl Styles<Rgb> {
    /// Applies `color` over `Foreground` and `Background`.
    pub fn color(self, color: PreRgba) -> Self {
        let foreground = color.over(self.foreground.0);
        let background = color.over(self.background.0);

        Styles {
            foreground: Foreground(foreground),
            background: Background(background),
            attributes: self.attributes,
        }
    }
}

impl Styles<PreRgba, Rgb> {
    /// Resolves `Foreground` to `Rgb` from `Background`.
    pub fn resolve(self) -> Styles<Rgb> {
        let foreground = self.foreground.over(self.background.0);

        Styles {
            foreground: Foreground(foreground),
            background: self.background,
            attributes: self.attributes,
        }
    }
}

/*
impl Styles<PreRgba, PreRgba> {
    /// Resolves `Foreground` to `Rgb` from `Background`
    /// (its alpha being discarded).
    pub fn resolve(self) -> Styles<Rgb> {
        Styles::<_, Rgb> {
            foreground: self.foreground,
            background: self.background.into(),
            attributes: self.attributes,
        }
        .resolve()
    }
}
*/

macro_rules! styler {
    ($($get:ident $set:ident $attr:ident: $Attr:ident)*) => { $(
        fn $get(self) -> Option<$Attr> {
            self.attributes.$attr
        }

        fn $set(self, $attr: Option<$Attr>) -> Self {
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

    fn get_foreground(self) -> Foreground<Fg> {
        self.foreground
    }

    fn get_background(self) -> Background<Bg> {
        self.background
    }

    fn get_attributes(self) -> Attributes {
        self.attributes
    }

    fn set_foreground(self, color: Fg) -> Self {
        Self {
            foreground: Foreground(color),
            ..self
        }
    }

    fn set_background(self, color: Bg) -> Self {
        Self {
            background: Background(color),
            ..self
        }
    }

    fn set_attributes(self, attributes: Attributes) -> Self {
        Self { attributes, ..self }
    }
}
