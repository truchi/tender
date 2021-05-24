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

    pub fn render<'a>(&'a self, w: impl Write) -> io::Result<()>
    where
        &'a T: GridRows,
        <&'a T as Grid>::Item: AsRef<Cell>,
    {
        render(self.position, &self.grid, w)
    }

    pub fn render_damage<'a>(&'a mut self, w: impl Write) -> io::Result<()>
    where
        &'a mut T: GridRows,
        <&'a mut T as Grid>::Item: AsMut<Damaged>,
    {
        render_damage(self.position, &mut self.grid, w)
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
pub struct MoveTo {
    initial: Point,
    current: Point,
}

impl MoveTo {
    pub fn new(initial: Point) -> Self {
        Self {
            initial,
            current: initial,
        }
    }

    pub fn first_col(&mut self) {
        self.current.x = self.initial.x;
    }

    pub fn first_row(&mut self) {
        self.current.y = self.initial.y;
    }

    pub fn next_col(&mut self) {
        self.current.x += 1;
    }

    pub fn next_row(&mut self) {
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
    let grid = RowVec1D::new((w, h), vec![canvas_cell; w * h]).unwrap();
    let mut canvas = Layer::new((0, 0), grid.clone());
    let mut screen = Screen::new((0, 0), grid, stdout());

    let cell1 = Cell::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), ());
    let layer1 = Layer::new((1, 1), repeat((10, 10), cell1));

    let cell2 = Cell::new('2', Rgb(0, 0, 255), Rgba(0, 255, 0, 127), ());
    let layer2 = Layer::new((2, 2), repeat((10, 10), cell2));

    screen.render().unwrap();
    screen.render_damage().unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    (&layer1).over(&mut canvas);
    canvas.render(stdout()).unwrap();
    // canvas.render_damage(stdout()).unwrap();
    // stdout().flush().unwrap();
    // sleep(Duration::from_millis(500));

    // (&layer2).over(&mut canvas);
    // canvas.render(stdout()).unwrap();
    // stdout().flush().unwrap();
    // sleep(Duration::from_millis(500));
}
