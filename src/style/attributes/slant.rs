pub use Slant::*;

/// [`Slant`](crate::Slant) (`Italic`, `NoSlant`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Slant {
    Italic,
    NoSlant,
}

impl Default for Slant {
    fn default() -> Self {
        NoSlant
    }
}
