use super::*;
use std::marker::PhantomData;

// ========================================================================== //
//                                                                            //
// ********************************* Aliases ******************************** //
//                                                                            //
// ========================================================================== //

pub type ColSlice2D<I, T> = Slice2D<ColMajor, I, T>;
pub type RowSlice2D<I, T> = Slice2D<RowMajor, I, T>;

pub type Vec2D<M, I> = Slice2D<M, I, Vec<I>>;
pub type ColVec2D<I> = Slice2D<ColMajor, I, Vec<I>>;
pub type RowVec2D<I> = Slice2D<RowMajor, I, Vec<I>>;

// ========================================================================== //
//                                                                            //
// ********************************* Slice2D ******************************** //
//                                                                            //
// ========================================================================== //

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Slice2D<M, I, T> {
    size:    Size,
    items:   T,
    phantom: PhantomData<(M, I)>,
}

impl<M: Major, I, T> Slice2D<M, I, T> {
    pub unsafe fn new_unchecked(size: impl Into<Size>, items: T) -> Self {
        Self {
            size: size.into(),
            items,
            phantom: PhantomData,
        }
    }

    pub fn new(size: impl Into<Size>, items: T) -> Option<Self>
    where
        T: AsRef<[I]>,
    {
        let size = size.into();

        if items.as_ref().len() == size.x * size.y {
            // SAFETY: len == x * y
            Some(unsafe { Self::new_unchecked(size, items) })
        } else {
            None
        }
    }

    pub fn new_mut(size: impl Into<Size>, mut items: T) -> Option<Self>
    where
        T: AsMut<[I]>,
    {
        let size = size.into();

        if items.as_mut().len() == size.x * size.y {
            // SAFETY: len == x * y
            Some(unsafe { Self::new_unchecked(size, items) })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> T {
        self.items
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl<M, I, T: AsRef<[I]>> AsRef<[I]> for Slice2D<M, I, T> {
    fn as_ref(&self) -> &[I] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[I]>> AsMut<[I]> for Slice2D<M, I, T> {
    fn as_mut(&mut self) -> &mut [I] {
        self.items.as_mut()
    }
}

// ========================================================================== //
//                                                                            //
// ****************************** Grid* traits ****************************** //
//                                                                            //
// ========================================================================== //

// ---------- //
// Grid (ref) //
// ---------- //

impl<'a, I, T: AsRef<I>> Grid<&'a I> for &'a ColSlice2D<I, T> {
    unsafe fn item_unchecked(self, point: impl Into<Point>) -> &'a I {
        todo!()
    }

    fn item(self, point: impl Into<Point>) -> Option<&'a I> {
        todo!()
    }
}

impl<'a, I, T: AsRef<I>> Grid<&'a I> for &'a RowSlice2D<I, T> {
    unsafe fn item_unchecked(self, point: impl Into<Point>) -> &'a I {
        todo!()
    }

    fn item(self, point: impl Into<Point>) -> Option<&'a I> {
        todo!()
    }
}

// ---------- //
// Grid (mut) //
// ---------- //

impl<'a, I, T: AsRef<I>> Grid<&'a mut I> for &'a mut ColSlice2D<I, T> {
    unsafe fn item_unchecked(self, point: impl Into<Point>) -> &'a mut I {
        todo!()
    }

    fn item(self, point: impl Into<Point>) -> Option<&'a mut I> {
        todo!()
    }
}

impl<'a, I, T: AsRef<I>> Grid<&'a mut I> for &'a mut RowSlice2D<I, T> {
    unsafe fn item_unchecked(self, point: impl Into<Point>) -> &'a mut I {
        todo!()
    }

    fn item(self, point: impl Into<Point>) -> Option<&'a mut I> {
        todo!()
    }
}
