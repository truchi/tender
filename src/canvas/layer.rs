use super::*;

pub trait First {
    fn new() -> Self;
    fn is_first(&self) -> bool;
    fn unset(&mut self);
}

impl First for () {
    fn new() -> Self {
        ()
    }

    fn is_first(&self) -> bool {
        // There is no reason to be here
        debug_assert!(
            false,
            "Calling `is_first()` on `()` (Layer is made of Cells)"
        );
        true
    }

    fn unset(&mut self) {}
}
impl First for bool {
    fn new() -> Self {
        true
    }

    fn is_first(&self) -> bool {
        *self
    }

    fn unset(&mut self) {
        *self = false;
    }
}

pub trait Options {
    type First: First;
}

impl Options for Cell {
    type First = ();
}
impl Options for Damaged {
    type First = bool;
}

pub struct Layer2<T, C: Options> {
    pub position: Point,
    grid:         T,
    first:        C::First,
}

impl<T, C: Options> Layer2<T, C> {
    pub fn new(position: impl Into<Point>, grid: T) -> Self {
        Self {
            position: position.into(),
            grid,
            first: C::First::new(),
        }
    }
}

impl<Top, Bottom> Over<Layer2<Bottom, Bottom::Item>> for Layer2<Top, Top::Item>
where
    Top: GridRows,
    Bottom: GridRows,
    Top::Item: Options,
    Bottom::Item: Options,
    Top::Item: Over<Bottom::Item>,
{
    type Output = ();

    fn over(self, bottom: Layer2<Bottom, Bottom::Item>) {
        bottom
            .grid
            .zip_at(self.position, self.grid)
            .flatten_rows()
            .for_each(|(bottom, top)| {
                top.over(bottom);
            });
    }
}

impl<'t, 'b, Top, Bottom, T, B> Over<&'b mut Layer2<Bottom, B>> for &'t Layer2<Top, T>
where
    T: Options,
    B: Options,
    &'t Top: GridRows,
    &'b mut Bottom: GridRows,
    <&'t Top as Grid>::Item: Over<<&'b mut Bottom as Grid>::Item>,
{
    type Output = ();

    fn over(self, bottom: &'b mut Layer2<Bottom, B>) {
        bottom
            .grid
            .zip_at(self.position, &self.grid)
            .flatten_rows()
            .for_each(|(bottom, top)| {
                top.over(bottom);
            });
    }
}

impl<'a, T, O> Paint for &'a mut Layer2<T, O>
where
    O: Options,
    &'a mut T: GridRows,
    <&'a mut T as Grid>::Item: AsMut<Cell>,
{
    type Output = ();

    fn paint(self, painter: impl Painter) {
        self.grid.flatten_rows().for_each(|mut cell| {
            cell.as_mut().paint(painter);
        });
    }
}

pub trait Render: Sized {
    fn render(self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a, T, W: Write> Render for (&'a Layer2<T, Cell>, W)
where
    &'a T: GridRows<Item = &'a Cell>,
{
    fn render(self) -> io::Result<()> {
        render2(self.0.position, &self.0.grid, self.1)
    }
}

impl<'a, T: 'a, W: Write> Render for (&'a mut Layer2<T, Damaged>, W)
where
    &'a mut T: GridRows<Item = &'a mut Damaged>,
{
    fn render(self) -> io::Result<()> {
        let (layer, out) = self;

        if layer.first.is_first() {
            layer.first.unset();
            render2(layer.position, &mut layer.grid, out)
        } else {
            render_damage(layer.position, &mut layer.grid, out)
        }
    }
}

pub fn test2() -> String {
    use crate::*;

    let mut vec = vec![];
    let mut out = stdout();
    let (w, h) = (151, 40);

    let mut layer = Layer2::<_, Damaged>::new(
        Point::ZERO,
        RowVec1D::new((w, h), vec![Damaged::default(); w * h]).unwrap(),
    );
    let mut repeated = Layer2::<_, Cell>::new(
        (10, 10),
        repeat(
            (10, 10),
            'a'.foreground(RED)
                .background(BLUE)
                .bold()
                .italic()
                .underlined()
                .striked(),
        ),
    );
    // dbg!(&layer);
    // dbg!(&repeated);

    (&mut layer).under(&repeated);

    // let mut v = vec![];
    (&mut layer, &mut out).render().unwrap();
    out.flush().unwrap();

    // dbg!(layer.first);

    repeated.position = repeated.position + Point::from((11, 11));

    (&mut layer).background(GREEN);
    (&mut layer).under(&repeated);
    (&mut layer, &mut out).render().unwrap();
    out.flush().unwrap();

    String::from_utf8(vec).unwrap()

    // let frame = layer.frame_ref(..).unwrap();
    // frame.render(&mut vec![]).unwrap();
    /*
     */
}

// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================

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

    pub fn frame_ref<'a>(&'a self, rect: impl Index2D) -> Option<Layer<Crop<&T>>>
    where
        T: WithSize,
        &'a T: Grid,
    {
        let rect = rect.checked(self.size())?;
        let position = self.position + rect.start();
        let grid = unsafe { self.grid.crop_unchecked(rect) };

        Some(Layer::new(position, grid))
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

pub fn test() -> String {
    use crate::*;

    let mut out = stdout();
    // let (w, h) = (2, 2);
    let (w, h) = (151, 40);
    let mut layer = Layer::new(
        Point::ZERO,
        RowVec1D::new((w, h), vec![Damaged::default(); w * h]).unwrap(),
    );
    let repeated = Layer::new(
        (10, 10),
        repeat(
            (10, 10),
            'a'.foreground(RED)
                .background(BLUE)
                .bold()
                .italic()
                .underlined()
                .striked(),
        ),
    );
    // dbg!(&layer);
    // dbg!(&repeated);

    let mut vec = vec![];

    layer.as_mut().under(repeated);
    layer.render(&mut out).unwrap();
    // layer.render(&mut vec).unwrap();
    out.flush().unwrap();

    String::from_utf8(vec).unwrap()

    // let frame = layer.frame_ref(..).unwrap();
    // frame.render(&mut vec![]).unwrap();
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

// impl<'screen, Top, Canvas: 'screen> Over<&'screen mut Screen<Canvas>> for
// Layer<Top> where
// Top: GridRows,
// &'screen mut Canvas: GridRows,
// Top::Item: Over<<&'screen mut Canvas as Grid>::Item>,
// {
// type Output = ();
//
// fn over(self, screen: &'screen mut Screen<Canvas>) {
// self.over(screen.as_layer_mut());
// }
// }

// impl<'frame, Top, Canvas> Over<&'frame mut Frame<'_, Canvas>> for Layer<Top>
// where
// Top: GridRows,
// &'frame mut Canvas: GridRows,
// Top::Item: Over<<&'frame mut Canvas as Grid>::Item>,
// {
// type Output = ();
//
// fn over(self, frame: &'frame mut Frame<'_, Canvas>) {
// self.over(frame.as_layer_mut());
// }
// }

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
