//! Grids that `Clone`/`Copy` elements.

use crate::grid::*;
use std::iter::Map;

macro_rules! grid1d {
    ($Type:ident: $Clone:ident ($Cloned:ident $cloned:ident) $($Trait:ident $Assoc:ident $fn:ident)*) => { $(
        impl<'a, I: 'a + $Clone, T: $Trait<Item = &'a I>> $Trait for $Type<T> {
            type $Assoc = std::iter::$Cloned<<T::$Assoc as IntoIterator>::IntoIter>;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                self.0.$fn(index).into_iter().$cloned()
            }
        }
    )* };
}

macro_rules! grid2d {
    ($Type:ident: $Clone:ident ($cloned:ident) $($Trait:ident $Assoc:ident $Item:ident $fn:ident)*) => { $(
        impl<'a, I: 'a + $Clone, T: $Trait<Item = &'a I>> $Trait for $Type<T> {
            type $Assoc = Map<<T::$Assoc as IntoIterator>::IntoIter, fn(T::$Item) -> Self::$Item>;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                self.0
                    .$fn(index)
                    .into_iter()
                    .map(|item| item.into_iter().$cloned())
            }
        }
    )* };
}

macro_rules! cloned {
    ($(
        $(#[$meta:meta])*
        $Type:ident: $Clone:ident ($Cloned:ident $cloned:ident)
    )*) => { $(
        $(#[$meta])*
        pub struct $Type<T>(pub(crate) T);

        impl<T: WithSize> WithSize for $Type<T> {
            fn size(&self) -> Size {
                self.0.size()
            }
        }

        impl<'a, I: 'a + Clone, T: Grid<Item = &'a I>> Grid for $Type<T> {
            type Item = I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                self.0.item_unchecked(index).clone()
            }
        }

        grid1d!($Type: $Clone ($Cloned $cloned)
            GridRow Row row_unchecked
            GridCol Col col_unchecked
        );

        grid2d!($Type: $Clone ($cloned)
            GridRows Rows Row rows_unchecked
            GridCols Cols Col cols_unchecked
        );

        impl<'a, I: 'a + $Clone, T: GridItems<Item = &'a I>> GridItems for $Type<T> {
            type Items = std::iter::$Cloned<<T::Items as IntoIterator>::IntoIter>;

            unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
                self.0.items_unchecked(index).into_iter().$cloned()
            }
        }
    )* };
}

cloned!(
    /// A grid yielding `Clone`d items.
    ///
    /// See [`Grid::cloned()`].
    Cloned: Clone (Cloned cloned)
    /// A grid yielding `Copy`ed items.
    ///
    /// See [`Grid::copied()`].
    Copied: Copy (Copied copied)
);
