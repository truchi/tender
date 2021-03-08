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

macro_rules! from {
    ($($From:ident for $For:ident)*) => { $(
        impl From<Styles<$From>> for Styles<$For> {
            fn from(style: Styles<$From>) -> Self {
                Self {
                    foreground: style.foreground.into(),
                    background: style.background.into(),
                    attributes: style.attributes,
                }
            }
        }
    )* };
}

from!(
    Rgba    for Rgb
    PreRgba for Rgb
    Rgb     for Rgba
    PreRgba for Rgba
    Rgb     for PreRgba
    Rgba    for PreRgba
);
