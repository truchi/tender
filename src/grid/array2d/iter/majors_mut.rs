pub use super::*;
use std::{marker::PhantomData, ops::Range, slice::from_raw_parts_mut};

/// A mutable 2D iterator along the major axis of an [`Array2D`].
#[derive(Debug)]
pub struct MajorsMut<'a, M, I, T, U> {
    items:   &'a mut [U],
    range:   Range<usize>,
    phantom: PhantomData<(M, I, T)>,
}

impl<'a, M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>> MajorsMut<'a, M, I, T, U> {
    pub(crate) unsafe fn rows_unchecked(
        grid: &'a mut Array2D<M, I, T, U>,
        index: impl Index2D,
    ) -> Self {
        let index = index.unchecked(grid.size);
        let items = grid.as_mut().get_unchecked_mut(index.y);

        Self {
            items,
            range: index.x,
            phantom: PhantomData,
        }
    }

    pub(crate) unsafe fn cols_unchecked(
        grid: &'a mut Array2D<M, I, T, U>,
        index: impl Index2D,
    ) -> Self {
        let index = index.unchecked(grid.size);
        let items = grid.as_mut().get_unchecked_mut(index.x);

        Self {
            items,
            range: index.y,
            phantom: PhantomData,
        }
    }
}

impl<'a, M, I: 'a, T, U: AsMut<[I]>> Iterator for MajorsMut<'a, M, I, T, U> {
    type Item = &'a mut [I];

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.items.len();

        if len == 0 {
            None
        } else {
            // SAFETY: users guaranty index is in bounds at construction
            Some(unsafe {
                let ptr = (&mut self.items).as_mut_ptr();
                let range = self.range.clone();

                debug_assert!(1 <= len);
                let slice = from_raw_parts_mut(ptr, 1);
                self.items = from_raw_parts_mut(ptr.add(1), len - 1);

                debug_assert!(slice.len() == 1);
                let slice = slice.get_unchecked_mut(0).as_mut();

                debug_assert!(range.start <= range.end);
                debug_assert!(range.end <= slice.len());
                let slice = slice.get_unchecked_mut(range);

                slice
            })
        }
    }
}
