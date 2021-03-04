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

impl Styles<PreRgba, Rgb> {
    pub fn flatten(self) -> Styles<Rgb> {
        let foreground = self.foreground.over(self.background.0);

        Styles {
            foreground: Foreground(foreground),
            background: self.background,
            attributes: self.attributes,
        }
    }
}

/*
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
*/
