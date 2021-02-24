pub use Border::*;

/// [`Border`](crate::Border) (`Circle`, `Frame`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Border {
    Circle,
    Frame,
}
