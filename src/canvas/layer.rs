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

    pub fn rows<'a>(&'a self) -> <<&'a T as GridRows>::Rows as IntoIterator>::IntoIter
    where
        &'a T: GridRows,
    {
        unsafe { self.grid.rows_unchecked(..) }.into_iter()
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

// grid!(
// Grid     Item item_unchecked Index0D
// GridRow  Row  row_unchecked  Index1D
// GridRows Rows rows_unchecked Index2D
// );

impl<Top, Bottom> Over<Bottom> for &Layer<Top>
where
    Self: GridRows,
    Bottom: GridRows,
    <Self as Grid>::Item: Over<<Bottom as Grid>::Item>,
{
    type Output = ();

    fn over(self, bottom: Bottom) {
        bottom
            .zip_at(self.position(), self)
            .flatten_rows()
            .for_each(|(bottom, top)| {
                top.over(bottom);
            });
    }
}

/*
impl<T> Display for Layer<T>
where
    for<'a> &'a T: GridRows<Item = &'a Cell<Rgb, Rgb>>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut rows = unsafe { self.grid.rows_unchecked(..) }.into_iter();
        let mut move_to = MoveTo(self.position);

        if let Some(row) = rows.next() {
            let mut row = row.into_iter();

            if let Some(mut previous) = row.next() {
                write!(f, "{}{}", move_to, previous)?;

                for cell in row {
                    write!(f, "{}", Dedup(*previous, *cell))?;
                    previous = cell;
                }
                move_to.next_row();

                for row in rows {
                    write!(f, "{}", move_to)?;
                    for cell in row {
                        write!(f, "{}", Dedup(*previous, *cell))?;
                        previous = cell;
                    }
                    move_to.next_row();
                }
            }
        }

        Ok(())
    }
}
*/

/*
 */
impl<T, C> Display for Layer<T>
where
    for<'a> &'a T: GridRows<Item = &'a C>,
    for<'a> &'a C: ICell,
{
    // for<'a> <&'a T as Grid>::Item: ICell,
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut rows = self.rows();
        // let mut rows = unsafe { self.grid.rows_unchecked(..) }.into_iter();
        let mut move_to = MoveTo(self.position);
        let mut previous: Cell<Rgb, Rgb>;

        fn render_row<T: ICell>(
            f: &mut Formatter,
            row: impl IntoIterator<Item = T>,
            previous: &mut Cell<Rgb, Rgb>,
            move_to: &mut MoveTo,
        ) -> fmt::Result {
            for icell in row {
                let cell = icell.cell();
                write!(f, "{}", Dedup(*previous, cell))?;
                *previous = cell;
            }
            move_to.next_row();

            Ok(())
        }

        if let Some(row) = rows.next() {
            let mut row = row.into_iter();

            if let Some(prev) = row.next() {
                previous = prev.cell();
                write!(f, "{}{}", move_to, previous)?;

                render_row(f, row, &mut previous, &mut move_to)?;

                for row in rows {
                    write!(f, "{}", move_to)?;
                    render_row(f, row, &mut previous, &mut move_to)?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct MoveTo(Point);

impl MoveTo {
    fn next_row(&mut self) {
        self.0.y += 1;
    }
}

impl Display for MoveTo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.0.y + 1, self.0.x + 1)
    }
}
// ===================================================================

pub fn example() {
    use std::{
        io::{stdout, Write},
        thread::sleep,
        time::Duration,
    };

    let (w, h) = (151, 40);

    let canvas_cell = Cell::<Rgb, _>::new(' ', Rgb(0, 0, 0), Rgb(255, 0, 0), ());
    let canvas_cell = Damaged::new(canvas_cell);
    let vec = vec![canvas_cell; w * h];
    let mut canvas = Layer::new((0, 0), RowVec1D::new((w, h), vec).unwrap());

    let cell1 = Cell::<Rgb, _>::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), ());
    let layer1 = Layer::new((1, 1), repeat((10, 10), cell1));

    let cell2 = Cell::<Rgb, _>::new('2', Rgb(0, 0, 255), Rgba(0, 255, 0, 127), ());
    let layer2 = Layer::new((2, 2), repeat((10, 10), cell2));

    print!("{}", canvas);
    /*
    // println!("{:?}", canvas.to_string());
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    (&layer1).over(&mut canvas);

    print!("{}", canvas);
    // println!("{:?}", canvas.to_string());
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    (&layer2).over(&mut canvas);

    print!("{}", canvas);
    // println!("{:?}", canvas.to_string());
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    */
}
