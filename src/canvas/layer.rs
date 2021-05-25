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

    pub fn size(&self) -> Size
    where
        T: WithSize,
    {
        self.grid.size()
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
            .zip_at(self.position, self.grid)
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
        self.over(screen.as_layer_mut());
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
        self.over(frame.as_layer_mut());
    }
}

impl<T> Paint for Layer<T>
where
    T: GridRows,
    T::Item: AsMut<Cell>,
{
    type Output = ();

    fn paint(self, painter: impl Painter) {
        self.grid
            .flatten_rows()
            .for_each(|mut cell| cell.as_mut().paint(painter));
    }
}

impl<'a, T> Paint for &'a mut Layer<T>
where
    &'a mut T: GridRows,
    <&'a mut T as Grid>::Item: AsMut<Cell>,
{
    type Output = ();

    fn paint(self, painter: impl Painter) {
        self.as_mut().paint(painter);
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

    screen.render().unwrap();
    screen.flush().unwrap();
    sleep(Duration::from_millis(500));

    screen.as_mut().under(layer1.as_ref());
    screen.paint(Italic);
    screen.render_damage().unwrap();
    screen.flush().unwrap();
    sleep(Duration::from_millis(500));

    let frame = &mut screen.frame((10..30, 10..30)).unwrap();
    frame.under(layer2.as_ref());
    frame.bold();
    frame.render_damage().unwrap();
    frame.flush().unwrap();
    sleep(Duration::from_millis(500));

    let frame = &mut frame.frame((4..10, 4..10)).unwrap();
    frame.foreground(PURPLE);
    frame.striked();
    frame.render_damage().unwrap();
    frame.flush().unwrap();
}
