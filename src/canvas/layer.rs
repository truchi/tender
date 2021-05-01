use super::*;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Layer<T> {
    pub position: Point,
    pub grid:     T,
}

impl<T> Layer<T> {
    pub fn new(position: impl Into<Point>, grid: T) -> Self {
        Self {
            position: position.into(),
            grid,
        }
    }
}

impl<T: WithSize> WithSize for Layer<T> {
    fn size(&self) -> Size {
        self.grid.size()
    }
}

impl<T> WithPosition for Layer<T> {
    fn position(&self) -> Point {
        self.position
    }
}

macro_rules! grid {
    ($($Trait:ident $Assoc:ident $fn:ident $Index:ident)*) => { $(
        grid!(impl $Trait $Assoc $fn $Index);
        grid!(impl $Trait $Assoc $fn $Index mut);
    )* };
    (impl $Trait:ident $Assoc:ident $fn:ident $Index:ident $($mut:ident)?) => {
        impl<'a, T> $Trait for &'a $($mut)? Layer<T>
        where
            Self: WithSize,
            &'a $($mut)? T: $Trait,
        {
            type $Assoc = <&'a $($mut)? T as $Trait>::$Assoc;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                self.grid.$fn(index)
            }
        }
    };
}

grid!(
    Grid     Item item_unchecked Index0D
    GridRow  Row  row_unchecked  Index1D
    GridRows Rows rows_unchecked Index2D
);

impl<'t, Top, Bottom> Over<Bottom, ()> for &'t Layer<Top>
where
    &'t Layer<Top>: GridRows,
    Bottom: GridRows,
    <&'t Layer<Top> as Grid>::Item: Over<<Bottom as Grid>::Item, ()>,
{
    fn over(self, bottom: Bottom) {
        bottom
            .zip_at(self.position(), self)
            .flatten_rows()
            .for_each(|(bottom, top)| top.over(bottom));
    }
}

impl<T> Display for Layer<T>
where
    for<'a> &'a T: GridRows<Item = &'a Cell<Rgb>>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rows = unsafe { self.grid.rows_unchecked(..) };
        let mut move_to = MoveTo(self.position);

        for row in rows {
            write!(f, "{}", move_to)?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
            move_to.0.y += 1;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct MoveTo(Point);

impl Display for MoveTo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.0.y + 1, self.0.x + 1)
    }
}
