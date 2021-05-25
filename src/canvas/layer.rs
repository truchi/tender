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

impl<'frame, Top, Canvas> Over<&'frame mut Frame<'_, Canvas>> for Layer<Top>
where
    Top: GridRows,
    &'frame mut Canvas: GridRows,
    Top::Item: Over<<&'frame mut Canvas as Grid>::Item>,
{
    type Output = ();

    fn over(self, frame: &'frame mut Frame<'_, Canvas>) {
        self.over(Layer::new(Point::ZERO, unsafe {
            frame.screen.canvas.crop_unchecked(frame.rect.clone())
        }));
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
    let mut screen = Screen::new(
        (0, 0),
        RowVec1D::new((w, h), vec![
            Damaged::new(Cell::new(' ', BLACK, RED, ()));
            w * h
        ])
        .unwrap(),
        stdout(),
    );
    let layer1 = Layer::new(
        (0, 0),
        repeat((12, 12), Cell::new('1', LIME, Rgba(0, 0, 0, 127), ())),
    );
    let layer2 = Layer::new(
        (2, 2),
        repeat((10, 10), Cell::new('2', BLUE, Rgba(0, 255, 0, 127), ())),
    );

    screen.render();
    screen.flush();
    sleep(Duration::from_millis(500));

    screen.as_mut().under(layer1.as_ref());
    screen.render_damage();
    screen.flush();
    sleep(Duration::from_millis(500));

    let frame = &mut screen.frame((10..30, 10..30)).unwrap();
    frame.under(layer2.as_ref());
    frame.render_damage();
    frame.flush();
    sleep(Duration::from_millis(500));

    let frame = &mut frame.frame((2..10, 2..10)).unwrap();
    frame.under(layer2.as_ref());
    frame.render_damage();
    frame.flush();
}
