//! A grid that iterate in a [`Rect`] of itself.

use crate::grid::*;

/// A grid that only iterates in a [`Rect`].
///
/// See [`Grid::crop()`].
#[derive(Clone, Default, Debug)]
pub struct Crop<T> {
    rect: Rect,
    grid: T,
}

impl<T: WithSize> Crop<T> {
    pub(crate) fn new(rect: impl Index2D, grid: T) -> Option<Self> {
        let rect = rect.checked(grid.size())?;

        Some(Self { rect, grid })
    }

    pub(crate) unsafe fn new_unchecked(rect: impl Index2D, grid: T) -> Self {
        let rect = rect.unchecked(grid.size());

        Self { rect, grid }
    }
}

impl<T> WithSize for Crop<T> {
    fn size(&self) -> Size {
        self.rect.size()
    }
}

macro_rules! grid0d {
    () => {
        grid0d!(impl owned);
        grid0d!(impl ref);
        grid0d!(impl ref mut);
    };
    (impl owned) => {
        impl<T> Grid for Crop<T>
        where
            T: Grid
        {
            type Item = T::Item;

            grid0d!(impl fn);
        }
    };
    (impl ref $($mut:ident)?) => {
        impl<'a, T> Grid for &'a $($mut)? Crop<&$($mut)? T>
        where
            &'a $($mut)? T: Grid
        {
            type Item = <&'a $($mut)? T as Grid>::Item;

            grid0d!(impl fn);
        }
    };
    (impl fn) => {
        unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
            let index = index.unchecked() + self.rect.start();

            self.grid.item_unchecked(index)
        }
    };
}

macro_rules! grid1d {
    ($($Trait:ident $Assoc:ident $fn:ident $i:ident $range:ident)*) => { $(
        grid1d!(impl owned  : $Trait $Assoc $fn $i $range);
        grid1d!(impl ref    : $Trait $Assoc $fn $i $range);
        grid1d!(impl ref mut: $Trait $Assoc $fn $i $range);
    )* };
    (impl owned: $Trait:ident $Assoc:ident $fn:ident $i:ident $range:ident) => {
        impl<T> $Trait for Crop<T>
        where
            T: $Trait
        {
            type $Assoc = T::$Assoc;

            grid1d!(impl fn $fn $Assoc $i $range);
        }
    };
    (impl ref $($mut:ident)?: $Trait:ident $Assoc:ident $fn:ident $i:ident $range:ident) => {
        impl<'a, T> $Trait for &'a $($mut)? Crop<&$($mut)? T>
        where
            &'a $($mut)? T: $Trait
        {
            type $Assoc = <&'a $($mut)? T as $Trait>::$Assoc;

            grid1d!(impl fn $fn $Assoc $i $range);
        }
    };
    (impl fn $fn:ident $Assoc:ident $i:ident $range:ident) => {
        unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
            let mut index = index.$fn(self.size());
            index.0 += self.rect.$i.start;
            index.1.start += self.rect.$range.start;
            index.1.end += self.rect.$range.start;

            self.grid.$fn(index)
        }
    };
}

macro_rules! grid2d {
    ($($Trait:ident $Assoc:ident $fn:ident)*) => { $(
        grid2d!(impl owned  : $Trait $Assoc $fn);
        grid2d!(impl ref    : $Trait $Assoc $fn);
        grid2d!(impl ref mut: $Trait $Assoc $fn);
    )* };
    (impl owned: $Trait:ident $Assoc:ident $fn:ident) => {
        impl<T> $Trait for Crop<T>
        where
            T: $Trait
        {
            type $Assoc = T::$Assoc;

            grid2d!(impl fn $fn $Assoc);
        }
    };
    (impl ref $($mut:ident)?: $Trait:ident $Assoc:ident $fn:ident) => {
        impl<'a, T> $Trait for &'a $($mut)? Crop<&$($mut)? T>
        where
            &'a $($mut)? T: $Trait
        {
            type $Assoc = <&'a $($mut)? T as $Trait>::$Assoc;

            grid2d!(impl fn $fn $Assoc);
        }
    };
    (impl fn $fn:ident $Assoc:ident) => {
        unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
            let index = index.unchecked(self.size()).translate(self.rect.start());

            self.grid.$fn(index)
        }
    };
}

grid0d!();
grid1d!(
    GridCol Col col_unchecked x y
    GridRow Row row_unchecked y x
);
grid2d!(
    GridCols  Cols  cols_unchecked
    GridRows  Rows  rows_unchecked
    GridItems Items items_unchecked
);
