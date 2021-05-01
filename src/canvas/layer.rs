use super::*;

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
            T: WithSize,
            &'a $($mut)? T: $Trait,
        {
            type $Assoc = <&'a $($mut)? T as $Trait>::$Assoc;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                self.grid.$fn(index)
            }
        }
    };
}

// FIXME
grid!(
    Grid     Item item_unchecked Index0D
    GridRow  Row  row_unchecked  Index1D
    GridRows Rows rows_unchecked Index2D
);

impl<'t, 'b, Top, Bottom> Over<&'b mut Layer<Bottom>, ()> for &'t Layer<Top>
where
    &'t Layer<Top>: GridRows,
    &'b mut Layer<Bottom>: GridRows,
    <&'t Layer<Top> as Grid>::Item: Over<<&'b mut Layer<Bottom> as Grid>::Item, ()>,
{
    fn over(self, bottom: &'b mut Layer<Bottom>) {
        self.zip_at(bottom.position(), bottom)
            .flatten_rows()
            .for_each(|(top, bottom)| top.over(bottom));
    }
}

fn _main() {
    let canvas_cell = Cell::<Rgb, _>::new(' ', Rgb(0, 0, 0), Rgb(255, 0, 0), Default::default());
    let mut canvas = Layer::new((0, 0), RowVec1D::new((1, 1), vec![canvas_cell; 1]).unwrap());
    dbg!(&canvas);

    let cell1 = Cell::<Rgb, _>::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), Default::default());
    let layer1 = Layer::new((0, 0), repeat((1, 1), cell1));
    dbg!(layer1);

    // let cell2 = Cell::<Rgb, _>::new(
    // '2',
    // Rgb(0, 0, 255),
    // Rgba(0, 255, 0, 127),
    // Default::default(),
    // );
    // let layer2 = Layer::new((0, 0), repeat((1, 1), cell2));
    // dbg!(layer2);

    // (&layer1).over(&mut canvas);
    dbg!(&canvas);
}
