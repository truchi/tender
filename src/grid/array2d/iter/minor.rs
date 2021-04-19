use super::*;
use std::{marker::PhantomData, ops::Range};

/// A 1D iterator along the minor axis of an [`Grid2D`].
#[derive(Debug)]
pub struct Minor<'a, M, I, T, U> {
    items:   &'a [U],
    index:   usize,
    range:   Range<usize>,
    phantom: PhantomData<(M, I, T)>,
}

impl<'a, M: Major, I, T: AsRef<[U]>, U> Minor<'a, M, I, T, U> {
    pub(crate) unsafe fn new_unchecked(grid: &'a Grid2D<M, I, T, U>, index: impl Index1D) -> Self {
        let size = grid.size;
        let (index, range) = index.unchecked(size.minor());

        Self {
            items: grid.as_ref(),
            index,
            range,
            phantom: PhantomData,
        }
    }
}

impl<'a, M, I: 'a, T, U: AsRef<[I]>> Iterator for Minor<'a, M, I, T, U> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.range.next()?;

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe {
            debug_assert!(start < self.items.len());
            let items = self.items.get_unchecked(start).as_ref();

            debug_assert!(self.index < items.len());
            items.get_unchecked(self.index)
        })
    }
}
