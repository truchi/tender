use crate::style::*;

/// [`Style`](crate::Style)s.
///
/// A straightforward implementation of [`Styler`](crate::Styler).
///
/// `Display`s the corresponding CSIs to the terminal.
///
/// `Default`s as an empty [`Style`](crate::Style) (all fields set to `None`).
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Style {
    pub foreground: Option<Foreground>,
    pub background: Option<Background>,
    pub weight:     Option<Weight>,
    pub slant:      Option<Slant>,
    pub underline:  Option<Underline>,
    pub strike:     Option<Strike>,
    pub overline:   Option<Overline>,
    pub invert:     Option<Invert>,
    pub blink:      Option<Blink>,
    pub border:     Option<Border>,
}
