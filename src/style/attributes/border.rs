pub use Border::*;

/// [`Border`](crate::Border) (`Circle`, `Frame`, `NoBorder`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Border {
    Circle,
    Frame,
    NoBorder,
}

impl Default for Border {
    fn default() -> Self {
        NoBorder
    }
}
