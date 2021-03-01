pub use Strike::*;

/// [`Strike`](crate::Strike) (`Striked`, `NoStrike`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Strike {
    Striked,
    NoStrike,
}

impl Default for Strike {
    fn default() -> Self {
        NoStrike
    }
}
