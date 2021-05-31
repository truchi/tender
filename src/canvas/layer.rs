use super::*;

pub struct Layer<G, O: Options = Cell> {
    pub position: Point,
    grid:         G,
    first:        O::First,
}

impl<G: Debug> Debug for Layer<G, Cell> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Layer")
            .field("position", &self.position)
            .field("grid", &self.grid)
            .finish()
    }
}

impl<G: Debug> Debug for Layer<G, Damaged> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Layer")
            .field("position", &self.position)
            .field("first", &self.first)
            .field("grid", &self.grid)
            .finish()
    }
}

impl<G, O: Options> Layer<G, O> {
    pub fn new(position: impl Into<Point>, grid: G) -> Self {
        Self {
            position: position.into(),
            grid,
            first: O::First::new(),
        }
    }

    pub fn frame<'a>(&'a self, rect: impl Index2D) -> Option<Layer<Crop<&'a G>, O>>
    where
        G: WithSize,
        &'a G: Grid,
    {
        Some(unsafe { Self::frame_unchecked(self, rect.checked(self.grid.size())?) })
    }

    pub fn frame_mut<'a>(&'a mut self, rect: impl Index2D) -> Option<Layer<Crop<&'a mut G>, O>>
    where
        G: WithSize,
        &'a mut G: Grid,
    {
        Some(unsafe { Self::frame_mut_unchecked(self, rect.checked(self.grid.size())?) })
    }

    pub unsafe fn frame_unchecked<'a>(&'a self, rect: impl Index2D) -> Layer<Crop<&'a G>, O>
    where
        G: WithSize,
        &'a G: Grid,
    {
        let rect = rect.unchecked(self.grid.size());
        let position = self.position + rect.start();
        let grid = self.grid.crop_unchecked(rect);

        Layer {
            position,
            grid,
            first: self.first,
        }
    }

    pub unsafe fn frame_mut_unchecked<'a>(
        &'a mut self,
        rect: impl Index2D,
    ) -> Layer<Crop<&'a mut G>, O>
    where
        G: WithSize,
        &'a mut G: Grid,
    {
        let rect = rect.unchecked(self.grid.size());
        let position = self.position + rect.start();
        let grid = (&mut self.grid).crop_unchecked(rect);

        Layer {
            position,
            grid,
            first: self.first,
        }
    }
}

impl<'t, 'b, Top, Bottom, T, B> Over<&'b mut Layer<Bottom, B>> for &'t Layer<Top, T>
where
    T: Options,
    B: Options,
    &'t Top: GridRows,
    &'b mut Bottom: GridRows,
    <&'t Top as Grid>::Item: Over<<&'b mut Bottom as Grid>::Item>,
{
    type Output = ();

    fn over(self, bottom: &'b mut Layer<Bottom, B>) {
        bottom
            .grid
            .zip_at(self.position, &self.grid)
            .flatten_rows()
            .for_each(|(bottom, top)| {
                top.over(bottom);
            });
    }
}

impl<'a, G, O> Paint for &'a mut Layer<G, O>
where
    O: Options,
    &'a mut G: GridRows,
    <&'a mut G as Grid>::Item: AsMut<Cell>,
{
    type Output = ();

    fn paint(self, painter: impl Painter) {
        self.grid.flatten_rows().for_each(|mut cell| {
            cell.as_mut().paint(painter);
        });
    }
}

impl<'a, G, W: Write> Render for (&'a Layer<G, Cell>, W)
where
    &'a G: GridRows<Item = &'a Cell>,
{
    fn render(self) -> io::Result<()> {
        render(self.0.position, &self.0.grid, self.1)
    }
}

impl<'a, G: 'a, W: Write> Render for (&'a mut Layer<G, Damaged>, W)
where
    &'a mut G: GridRows<Item = &'a mut Damaged>,
{
    fn render(self) -> io::Result<()> {
        let (layer, out) = self;

        if layer.first.is_first() {
            layer.first.unset();
            render(layer.position, &mut layer.grid, out)
        } else {
            render_damage(layer.position, &mut layer.grid, out)
        }
    }
}

pub fn test2() {
    use super::*;
    let layer = Layer::<_, Damaged>::new((0, 0), repeat((10, 10), Damaged::new('a'.italic())));
    dbg!(layer);
}
