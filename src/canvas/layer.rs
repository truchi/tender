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

    pub fn render<'a>(&'a self, mut w: impl Write) -> io::Result<()>
    where
        &'a T: GridRows,
        <&'a T as Grid>::Item: ICell,
    {
        let mut rows = unsafe { self.grid.rows_unchecked(..) }.into_iter();
        let mut move_to = MoveTo(self.position);
        let mut previous: Cell;

        if let Some(row) = rows.next() {
            let mut row = row.into_iter();

            if let Some(cell) = row.next() {
                previous = cell.cell();
                write!(w, "{}{}", move_to, previous)?;

                Self::render_row(&mut w, row, &mut previous, &mut move_to)?;

                for row in rows {
                    write!(w, "{}", move_to)?;
                    Self::render_row(&mut w, row, &mut previous, &mut move_to)?;
                }
            }
        }

        Ok(())
    }

    fn render_row<C: ICell>(
        mut w: impl Write,
        row: impl IntoIterator<Item = C>,
        previous: &mut Cell,
        move_to: &mut MoveTo,
    ) -> io::Result<()> {
        for icell in row {
            let cell = icell.cell();
            write!(w, "{}", Dedup(*previous, cell))?;
            *previous = cell;
        }
        move_to.next_row();

        Ok(())
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

impl<'t, 'b, Top, Bottom> Over<&'b mut Layer<Bottom>> for &'t Layer<Top>
where
    &'t Top: GridRows,
    &'b mut Bottom: GridRows,
    <&'t Top as Grid>::Item: Over<<&'b mut Bottom as Grid>::Item>,
{
    type Output = ();

    fn over(self, bottom: &'b mut Layer<Bottom>) {
        bottom
            .grid
            .zip_at(self.position(), &self.grid)
            .flatten_rows()
            .for_each(|(bottom, top)| {
                top.over(bottom);
            });
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
    use std::{io::stdout, thread::sleep, time::Duration};

    let (w, h) = (151, 40);

    let canvas_cell = Cell::new(' ', Rgb(0, 0, 0), Rgb(255, 0, 0), ());
    let canvas_cell = Damaged::new(canvas_cell);
    let vec = vec![canvas_cell; w * h];
    let mut canvas = Layer::new((0, 0), RowVec1D::new((w, h), vec).unwrap());

    let cell1 = Cell::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), ());
    let layer1 = Layer::new((1, 1), repeat((10, 10), cell1));

    let cell2 = Cell::new('2', Rgb(0, 0, 255), Rgba(0, 255, 0, 127), ());
    let layer2 = Layer::new((2, 2), repeat((10, 10), cell2));

    canvas.render(stdout()).unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    (&layer1).over(&mut canvas);
    canvas.render(stdout()).unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    (&layer2).over(&mut canvas);
    canvas.render(stdout()).unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
}
