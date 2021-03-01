pub use Overline::*;

/// [`Overline`](crate::Overline) (`Overlined`, `NoOverline`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Overline {
    Overlined,
    NoOverline,
}

impl Default for Overline {
    fn default() -> Self {
        NoOverline
    }
}
