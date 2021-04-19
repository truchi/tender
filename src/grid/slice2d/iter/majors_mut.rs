use super::*;
use std::{marker::PhantomData, ops::Range, slice::from_raw_parts_mut};

/// A mutable 2D iterator along the major axis of a [`Slice2D`].
#[derive(Debug)]
pub struct MajorsMut<'a, M, I, T> {
    items:    &'a mut [I],
    range:    Range<usize>,
    major:    usize,
    count:    usize,
    _phantom: PhantomData<(M, T)>,
}

impl<'a, M: Major, I, T: AsMut<[I]>> MajorsMut<'a, M, I, T> {
    pub(crate) unsafe fn new_unchecked(
        grid: &'a mut Slice2D<M, I, T>,
        index: impl Index2D,
    ) -> Self {
        let index = index.unchecked(grid.size);
        let (range, minor) = major_index2d::<M>(index);

        // Splitting to the first col/row of interest
        let major = grid.size.major();
        let first = minor.start * major;
        let items = grid.as_mut();
        debug_assert!(first <= items.len(), "Index out of bounds");
        let items = items.get_unchecked_mut(first..);

        Self {
            items,
            range,
            major,
            count: minor.end - minor.start,
            _phantom: PhantomData,
        }
    }
}

impl<'a, M, I, T> Iterator for MajorsMut<'a, M, I, T> {
    type Item = &'a mut [I];

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let len = self.items.len();
            let ptr = self.items.as_mut_ptr();

            // SAFETY: users guaranty index is in bounds at construction
            let (slice, items) = unsafe {
                debug_assert!(self.major <= self.items.len());
                let (slice, items) = (
                    from_raw_parts_mut(ptr, self.major),
                    from_raw_parts_mut(ptr.add(self.major), len - self.major),
                );

                debug_assert!(self.range.end <= slice.len());
                (slice.get_unchecked_mut(self.range.clone()), items)
            };

            self.items = items;
            self.count -= 1;

            Some(slice)
        }
    }
}
