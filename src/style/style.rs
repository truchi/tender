use crate::style::*;

///
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Style {
    pub foreground: Foreground,
    pub background: Background,
    pub weight:     Weight,
    pub slant:      Slant,
    pub underline:  Underline,
    pub strike:     Strike,
    pub overline:   Overline,
    pub invert:     Invert,
    pub blink:      Blink,
    pub border:     Border,
}
