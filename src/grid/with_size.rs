use crate::grid::*;
use std::ops::Deref;

/// [`Size`](Size) getter.
///
/// Anything that [`Deref`](std::ops::Deref)s into a `WithSize` is `WithSize`.
pub trait WithSize {
    /// Returns the [`Size`](Size).
    fn size(&self) -> Size;
}

impl<T: Deref<Target = U>, U: WithSize> WithSize for T {
    fn size(&self) -> Size {
        self.deref().size()
    }
}
