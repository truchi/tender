pub use Invert::*;

/// [`Invert`](crate::Invert) (`Inverted`, `NoInvert`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Invert {
    Inverted,
    NoInvert,
}

impl Default for Invert {
    fn default() -> Self {
        NoInvert
    }
}
