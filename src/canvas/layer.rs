use super::*;

pub struct Layer<T, C: Options = Cell> {
    pub position: Point,
    grid:         T,
    first:        C::First,
}

impl<T, C: Options> Layer<T, C> {
    pub fn new(position: impl Into<Point>, grid: T) -> Self {
        Self {
            position: position.into(),
            grid,
            first: C::First::new(),
        }
    }

    pub fn frame_ref<'a>(&'a self, rect: impl Index2D) -> Option<Layer<Crop<&'a T>, C>>
    where
        T: WithSize,
        &'a T: Grid,
    {
        let rect = rect.checked(self.grid.size())?;
        let position = self.position + rect.start();
        let grid = unsafe { self.grid.crop_unchecked(rect) };

        Some(Layer {
            position,
            grid,
            first: self.first,
        })
    }

    pub fn frame_mut<'a>(&'a mut self, rect: impl Index2D) -> Option<Layer<Crop<&'a mut T>, C>>
    where
        T: WithSize,
        &'a mut T: Grid,
    {
        let rect = rect.checked(self.grid.size())?;
        let position = self.position + rect.start();
        let grid = unsafe { (&mut self.grid).crop_unchecked(rect) };

        Some(Layer {
            position,
            grid,
            first: self.first,
        })
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

impl<'a, T, O> Paint for &'a mut Layer<T, O>
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

impl<'a, T, W: Write> Render for (&'a Layer<T, Cell>, W)
where
    &'a T: GridRows<Item = &'a Cell>,
{
    fn render(self) -> io::Result<()> {
        render(self.0.position, &self.0.grid, self.1)
    }
}

impl<'a, T: 'a, W: Write> Render for (&'a mut Layer<T, Damaged>, W)
where
    &'a mut T: GridRows<Item = &'a mut Damaged>,
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
