use super::*;

pub trait Grid<I> {
    unsafe fn item_unchecked(self, point: impl Into<Point>) -> I;
    fn item(self, point: impl Into<Point>) -> Option<I>;
}

pub trait GridRow<I>: Grid<I> + Sized {
    type Row: IntoIterator<Item = I>;

    unsafe fn row_unchecked(self, row: impl Into<Row>) -> Self::Row;
    fn row(self, row: impl Into<Row>) -> Option<Self::Row>;
}

pub trait GridCol<I>: Grid<I> + Sized {
    type Col: IntoIterator<Item = I>;

    unsafe fn col_unchecked(self, col: impl Into<Col>) -> Self::Col;
    fn col(self, col: impl Into<Col>) -> Option<Self::Col>;
}

pub trait GridItems<I>: GridCol<I> + Sized {
    type Items: IntoIterator<Item = I>;

    unsafe fn items_unchecked(self, rect: impl Into<Rect>) -> Self::Items;
    fn items(self, rect: impl Into<Rect>) -> Option<Self::Items>;
}

pub trait GridRows<I>: GridRow<I> + Sized {
    type Rows: IntoIterator<Item = Self::Row>;

    unsafe fn rows_unchecked(self, rect: impl Into<Rect>) -> Self::Rows;
    fn rows(self, rect: impl Into<Rect>) -> Option<Self::Rows>;
}

pub trait GridCols<I>: GridCol<I> + Sized {
    type Cols: IntoIterator<Item = Self::Cols>;

    unsafe fn cols_unchecked(self, rect: impl Into<Rect>) -> Self::Cols;
    fn cols(self, rect: impl Into<Rect>) -> Option<Self::Cols>;
}
