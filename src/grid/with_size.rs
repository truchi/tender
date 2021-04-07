use crate::grid::*;
use std::ops::Deref;

/// [`Size`] getter.
///
/// Anything that derefs into a [`WithSize`] is [`WithSize`].
pub trait WithSize {
    /// Returns the [`Size`].
    fn size(&self) -> Size;
}

impl<T: Deref<Target = U>, U: WithSize> WithSize for T {
    fn size(&self) -> Size {
        self.deref().size()
    }
}
