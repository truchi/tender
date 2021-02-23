use crate::grid::*;
use std::ops::Deref;

/// [`Size`](Size) as [`Major`](Major) getter.
///
/// Anything that [`Deref`](std::ops::Deref)s into a `WithMSize` is `WithMSize`.
pub trait WithMSize<M: Major>: WithSize {
    /// Returns the [`Size`](Size) as [`Major`](Major).
    fn msize(&self) -> M {
        self.size().into()
    }
}

impl<M: Major, T: Deref<Target = U>, U: WithMSize<M>> WithMSize<M> for T {
    fn msize(&self) -> M {
        self.deref().msize()
    }
}
