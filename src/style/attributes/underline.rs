pub use Underline::*;

/// [`Underline`](crate::Underline) (`Underlined`, `NoUnderline`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Underline {
    Underlined,
    NoUnderline,
}

impl Default for Underline {
    fn default() -> Self {
        NoUnderline
    }
}
