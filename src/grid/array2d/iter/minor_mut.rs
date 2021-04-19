use super::*;
use std::{marker::PhantomData, ops::Range, slice::from_raw_parts_mut};

/// A 1D iterator along the minor axis of an [`Array2D`].
#[derive(Debug)]
pub struct MinorMut<'a, M, I, T, U> {
    items:   &'a mut [U],
    index:   usize,
    phantom: PhantomData<(M, I, T)>,
}

impl<'a, M: Major, I, T: AsMut<[U]>, U> MinorMut<'a, M, I, T, U> {
    pub(crate) unsafe fn new_unchecked(
        grid: &'a mut Array2D<M, I, T, U>,
        index: impl Index1D,
    ) -> Self {
        let size = grid.size;
        let (index, Range { start, end }) = index.unchecked(size.minor());

        let items = grid.as_mut();
        let len = items.len();

        debug_assert!(start <= end);
        debug_assert!(end <= len);
        let items = items.get_unchecked_mut(start..end);

        Self {
            items,
            index,
            phantom: PhantomData,
        }
    }
}

impl<'a, M, I: 'a, T, U: AsMut<[I]>> Iterator for MinorMut<'a, M, I, T, U> {
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.items.len();

        if len == 0 {
            None
        } else {
            // SAFETY: constructor guaranty index is in bounds
            Some(unsafe {
                let ptr = self.items.as_mut_ptr();

                debug_assert!(1 <= len);
                let slice = from_raw_parts_mut(ptr, 1);
                self.items = from_raw_parts_mut(ptr.add(1), len - 1);

                debug_assert!(slice.len() == 1);
                let slice = slice.get_unchecked_mut(0).as_mut();

                debug_assert!(self.index < slice.len());
                slice.get_unchecked_mut(self.index)
            })
        }
    }
}
