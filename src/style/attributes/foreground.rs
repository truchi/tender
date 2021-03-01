use crate::style::*;

/// A [`Foreground`](crate::Foreground) [`Color`](crate::Color).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.
///
/// `Default`s to `Foreground(Color::ResetColor)`, user's default terminal's
/// foreground color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Foreground(pub PreRgba);
