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
        <&'a T as Grid>::Item: AsRef<Cell>,
    {
        fn render_row<C: AsRef<Cell>>(
            mut w: impl Write,
            row: impl IntoIterator<Item = C>,
            previous: &mut Cell,
            move_to: &mut MoveTo,
        ) -> io::Result<()> {
            for icell in row {
                let cell = *icell.as_ref();
                write!(w, "{}", Dedup(*previous, cell))?;
                *previous = cell;
            }
            move_to.next_row();

            Ok(())
        }

        let mut rows = unsafe { self.grid.rows_unchecked(..) }.into_iter();
        let mut move_to = MoveTo::new(self.position);

        if let Some(row) = rows.next() {
            let mut row = row.into_iter();

            // Render first cell as is
            if let Some(cell) = row.next() {
                let mut previous = *cell.as_ref();
                write!(w, "{}{}", move_to, previous)?;

                // Finish rendering this row, deduping
                render_row(&mut w, row, &mut previous, &mut move_to)?;

                // Render remaining rows, deduping
                for row in rows {
                    write!(w, "{}", move_to)?;
                    render_row(&mut w, row, &mut previous, &mut move_to)?;
                }

                // Done
                return Ok(());
            }
        }

        // Was empty
        Ok(())
    }

    pub fn render_damage<'a>(&'a mut self, mut w: impl Write) -> io::Result<()>
    where
        &'a mut T: GridRows,
        <&'a mut T as Grid>::Item: AsMut<Damaged>,
    {
        fn render_row_damage<C: AsMut<Damaged>>(
            mut w: impl Write,
            row: impl IntoIterator<Item = C>,
            previous: &mut Cell,
            move_to: &mut MoveTo,
            mut rendered: bool,
        ) -> io::Result<()> {
            for mut damaged in row {
                if let Some(cell) = damaged.as_mut().damage() {
                    if !rendered {
                        write!(w, "{}", move_to)?;
                    }
                    write!(w, "{}", Dedup(*previous, cell))?;
                    *previous = cell;
                    rendered = true;
                } else {
                    rendered = false;
                }
                move_to.next_col();
            }
            move_to.next_row();

            Ok(())
        }

        let mut rows = unsafe { self.grid.rows_unchecked(..) }.into_iter();
        let mut move_to = MoveTo::new(self.position);

        // We start looking for a cell that has damage
        while let Some(row) = rows.next() {
            move_to.first_col();

            let mut row = row.into_iter();
            while let Some(mut damaged) = row.next() {
                // Render first cell with damage as is
                if let Some(cell) = damaged.as_mut().damage() {
                    let mut previous = cell;

                    write!(w, "{}{}", move_to, previous)?;

                    // Finish rendering this row, deduping
                    move_to.next_col();
                    render_row_damage(&mut w, row, &mut previous, &mut move_to, true)?;

                    // Render remaining rows, deduping
                    while let Some(row) = rows.next() {
                        move_to.first_col();
                        render_row_damage(&mut w, row, &mut previous, &mut move_to, false)?;
                    }

                    // Done
                    return Ok(());
                }
                move_to.next_col();
            }
            move_to.next_row();
        }

        // Was empty or undamaged
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
struct MoveTo {
    initial: Point,
    current: Point,
}

impl MoveTo {
    fn new(initial: Point) -> Self {
        Self {
            initial,
            current: initial,
        }
    }

    fn first_col(&mut self) {
        self.current.x = self.initial.x;
    }

    fn first_row(&mut self) {
        self.current.y = self.initial.y;
    }

    fn next_col(&mut self) {
        self.current.x += 1;
    }

    fn next_row(&mut self) {
        self.current.y += 1;
    }
}

impl Display for MoveTo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.current.y + 1, self.current.x + 1)
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

    // let grid3 = repeat_with((4, 4), |_| {
    // Cell::new('3', Rgb(255, 0, 255), Rgb(0, 255, 0), ())
    // });
    // let layer3 = Layer::new((10, 10), grid3);
    //
    // layer3.render(stdout()).unwrap();
    // stdout().flush().unwrap();

    canvas.render(stdout()).unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    (&layer1).over(&mut canvas);
    canvas.render_damage(stdout()).unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    // (&layer2).over(&mut canvas);
    // canvas.render(stdout()).unwrap();
    // stdout().flush().unwrap();
    // sleep(Duration::from_millis(500));
}
