use crate::style::*;

/// A [`Background`](crate::Background) [`Color`](crate::Color).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.
///
/// `Default`s to `Background(Color::ResetColor)`, user's default terminal's
/// background color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Background(pub Rgba);
