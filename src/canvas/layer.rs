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

    pub fn as_ref(&self) -> Layer<&T> {
        Layer::new(self.position, &self.grid)
    }

    pub fn as_mut(&mut self) -> Layer<&mut T> {
        Layer::new(self.position, &mut self.grid)
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

impl<Top, Bottom> Over<Layer<Bottom>> for Layer<Top>
where
    Top: GridRows,
    Bottom: GridRows,
    Top::Item: Over<Bottom::Item>,
{
    type Output = ();

    fn over(self, bottom: Layer<Bottom>) {
        bottom
            .grid
            .zip_at(self.position(), self.grid)
            .flatten_rows()
            .for_each(|(bottom, top)| {
                top.over(bottom);
            });
    }
}

impl<'screen, Top, Canvas: 'screen> Over<&'screen mut Screen<Canvas>> for Layer<Top>
where
    Top: GridRows,
    &'screen mut Canvas: GridRows,
    Top::Item: Over<<&'screen mut Canvas as Grid>::Item>,
{
    type Output = ();

    fn over(self, screen: &'screen mut Screen<Canvas>) {
        self.over(Layer::new(Point::ZERO, &mut screen.canvas));
    }
}

impl<'frame, T, Top, Canvas: 'frame> Over<&'frame mut Frame<'_, T>> for Layer<Top>
where
    T: DerefMut<Target = Screen<Canvas>>,
    Top: GridRows,
    &'frame mut Canvas: GridRows,
    Top::Item: Over<<&'frame mut Canvas as Grid>::Item>,
{
    type Output = ();

    fn over(self, screen: &'frame mut Frame<'_, T>) {
        // self.over(Layer::new(Point::ZERO, &mut screen.canvas));
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
    // let mut canvas = Layer::new((0, 0), grid.clone());
    let mut screen = Screen::new((0, 0), grid, stdout());

    let cell1 = Cell::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), ());
    let layer1 = Layer::new((0, 0), repeat((12, 12), cell1));

    let cell2 = Cell::new('2', Rgb(0, 0, 255), Rgba(0, 255, 0, 127), ());
    let layer2 = Layer::new((2, 2), repeat((10, 10), cell2));

    screen.render().unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    (&mut screen).under(layer1.as_ref());
    screen.render_damage().unwrap();
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    let frame = &mut screen.frame((10..15, 10..15)).unwrap();
    frame.under(layer2.as_ref());
    // screen.render_damage().unwrap();
    // stdout().flush().unwrap();
}
