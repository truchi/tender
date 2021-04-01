use crate::grid::*;

/// Base trait for dealing with grids.
///
/// Grids can yield their [`Item`](Grid::Item)s given a [`Index0D`]
/// (a [`Point`]).
pub trait Grid: WithSize + Sized {
    /// The type of the grid's items.
    type Item;

    /// Returns the item at `index`, without bounds checking.
    ///
    /// ### Safety
    ///
    /// Calling this method with an out-of-bounds `index` is *undefined
    /// behavior*.
    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item;

    /// Returns the item at `index`, or [`None`] if out of bounds.
    fn item(self, index: impl Index0D) -> Option<Self::Item> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }

    /// Creates a grid which copies all of its elements.
    ///
    /// This is useful when you have an iterator over `&T`, but you need an
    /// iterator over `T`.
    fn copied<'a, T>(self) -> Copied<Self>
    where
        Self: Grid<Item = &'a T>,
        T: 'a + Copy,
    {
        Copied(self)
    }

    /// Creates a grid which clones all of its elements.
    ///
    /// This is useful when you have an iterator over `&T`, but you need an
    /// iterator over `T`.
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Grid<Item = &'a T>,
        T: 'a + Clone,
    {
        Cloned(self)
    }

    /// Creates a new grid by cropping with `rect`, without bounds
    /// checking.
    ///
    /// ### Safety
    ///
    /// Calling this method with an out-of-bounds `rect` is *undefined
    /// behavior*.
    unsafe fn crop_unchecked(self, rect: impl Index2D) -> Crop<Self> {
        Crop::new_unchecked(rect, self)
    }

    /// Creates a new grid by cropping with `rect`, or [`None`] if out of
    /// bounds.
    fn crop(self, rect: impl Index2D) -> Option<Crop<Self>> {
        Crop::new(rect, self)
    }

    /// ‘Zips up’ two grids into a single grid of pairs.
    ///
    /// Yields elements from the overlapping area of both grids.
    fn zip<U: Grid>(self, other: U) -> Zip<Self, U> {
        Zip::new(self, other)
    }
}

macro_rules! grid1d {
    ($(
        $(#[$meta:meta])*
        $Trait:ident
        $(#[$assoc_meta:meta])*
        $Assoc:ident
        $(#[$unchecked_meta:meta])*
        $unchecked:ident
        $(#[$checked_meta:meta])*
        $checked:ident
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait: Grid {
            $(#[$assoc_meta])*
            type $Assoc: IntoIterator<Item = Self::Item>;

            $(#[$unchecked_meta])*
            unsafe fn $unchecked(self, index: impl Index1D) -> Self::$Assoc;

            $(#[$checked_meta])*
            fn $checked(self, index: impl Index1D) -> Option<Self::$Assoc> {
                let index = index.$checked(self.size())?;

                // SAFETY: index is checked
                Some(unsafe { self.$unchecked(index) })
            }
        }
    )* };
}

macro_rules! grid2d {
    ($(
        $(#[$meta:meta])*
        $Trait:ident ($Parent:ident $Item:ident)
        $(#[$assoc_meta:meta])*
        $Assoc:ident
        $(#[$unchecked_meta:meta])*
        $unchecked:ident
        $(#[$checked_meta:meta])*
        $checked:ident
        $(#[$full_meta:meta])*
        $full:ident
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait: $Parent {
            $(#[$assoc_meta])*
            type $Assoc: IntoIterator<Item = Self::$Item>;

            $(#[$unchecked_meta])*
            unsafe fn $unchecked(self, index: impl Index2D) -> Self::$Assoc;

            $(#[$checked_meta])*
            fn $checked(self, index: impl Index2D) -> Option<Self::$Assoc> {
                let index = index.checked(self.size())?;

                // SAFETY: index is checked
                Some(unsafe { self.$unchecked(index) })
            }

            $(#[$full_meta])*
            fn $full(self) -> Self::$Assoc {
                // SAFETY: RangeFull must be safe
                unsafe { self.$unchecked(..) }
            }
        }
    )* };
}

grid1d!(
    /// Provides a [`Col`](GridCol::Col) 1D [`IntoIterator`].
    GridCol
        /// The type of a column.
        ///
        /// An iterator over [`Item`](Grid::Item)s.
        Col
        /// Returns the column at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index`
        /// is *undefined behavior*.
        col_unchecked
        /// Returns the column at `index`, or `None` if out of bounds.
        col
    /// Provides a [`Row`](GridRow::Row) 1D [`IntoIterator`].
    GridRow
        /// The type of a row.
        ///
        /// An iterator over [`Item`](Grid::Item)s.
        Row
        /// Returns the row at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index`
        /// is *undefined behavior*.
        row_unchecked
        /// Returns the row at `index`, or `None` if out of bounds.
        row
);

grid2d!(
    /// Provides a [`Cols`](GridCols::Cols) 2D [`IntoIterator`].
    GridCols (GridCol Col)
        /// The type of columns.
        ///
        /// An iterator over [`Col`](GridCol::Col)s.
        Cols
        /// Returns the columns at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index`
        /// is *undefined behavior*.
        cropped_cols_unchecked
        /// Returns the columns at `index`, or `None` if out of bounds.
        cropped_cols
        /// Returns the columns.
        cols
    /// Provides a [`Rows`](GridRows::Rows) 2D [`IntoIterator`].
    GridRows (GridRow Row)
        /// The type of rows.
        ///
        /// An iterator over [`Row`](GridRow::Row)s.
        Rows
        /// Returns the rows at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index`
        /// is *undefined behavior*.
        cropped_rows_unchecked
        /// Returns the rows at `index`, or `None` if out of bounds.
        cropped_rows
        /// Returns the rows.
        rows
    /// Provides an [`Items`](GridItems::Items) 2D [`IntoIterator`].
    GridItems (Grid Item)
        /// The type of items.
        ///
        /// An iterator over [`Item`](Grid::Item)s.
        Items
        /// Returns the items at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index`
        /// is *undefined behavior*.
        cropped_items_unchecked
        /// Returns the items at `index`, or `None` if out of bounds.
        cropped_items
        /// Returns the items.
        items
);
