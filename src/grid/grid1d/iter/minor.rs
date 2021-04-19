use super::*;
use std::{marker::PhantomData, ops::Range};

/// A 1D iterator along the minor axis of a [`Grid1D`].
#[derive(Debug)]
pub struct Minor<'a, M, I, T> {
    items:    &'a [I],
    current:  usize,
    by:       usize,
    count:    usize,
    _phantom: PhantomData<(M, T)>,
}

impl<'a, M: Major, I, T: AsRef<[I]>> Minor<'a, M, I, T> {
    pub(crate) unsafe fn new_unchecked(grid: &'a Grid1D<M, I, T>, index: impl Index1D) -> Self {
        let size = grid.size;
        let (i, Range { start, end }) = index.unchecked(size.minor());

        Self {
            items:    grid.as_ref(),
            current:  index0d(M::new(i, start).into(), size),
            count:    end - start,
            by:       size.major(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, M, I, T> Iterator for Minor<'a, M, I, T> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let current = self.current;
            self.current += self.by;
            self.count -= 1;

            // SAFETY: users guaranty index is in bounds at construction
            debug_assert!(current < self.items.len(), "Index out of bounds");
            Some(unsafe { self.items.get_unchecked(current) })
        }
    }
}
