use super::*;

pub struct Screen<G, O: Options = Cell, W = Stdout> {
    pub out: W,
    layer:   Layer<G, O>,
}

impl<G, O: Options, W> Screen<G, O, W> {
    pub fn new(layer: Layer<G, O>, out: W) -> Self {
        Self { layer, out }
    }

    pub fn size(&self) -> Size
    where
        G: WithSize,
    {
        self.layer.size()
    }

    pub fn frame<'a>(&'a self, rect: impl Index2D) -> Option<Screen<Crop<&'a G>, O, &'a W>>
    where
        G: WithSize,
        &'a G: Grid,
    {
        Some(unsafe { Self::frame_unchecked(self, rect.checked(self.size())?) })
    }

    pub fn frame_mut<'a>(
        &'a mut self,
        rect: impl Index2D,
    ) -> Option<Screen<Crop<&'a mut G>, O, &'a mut W>>
    where
        G: WithSize,
        &'a mut G: Grid,
    {
        Some(unsafe { Self::frame_mut_unchecked(self, rect.checked(self.size())?) })
    }

    pub unsafe fn frame_unchecked<'a>(&'a self, rect: impl Index2D) -> Screen<Crop<&'a G>, O, &'a W>
    where
        G: WithSize,
        &'a G: Grid,
    {
        Screen::new(self.layer.frame_unchecked(rect), &self.out)
    }

    pub unsafe fn frame_mut_unchecked<'a>(
        &'a mut self,
        rect: impl Index2D,
    ) -> Screen<Crop<&'a mut G>, O, &'a mut W>
    where
        G: WithSize,
        &'a mut G: Grid,
    {
        Screen::new(self.layer.frame_mut_unchecked(rect), &mut self.out)
    }

    pub fn flush(&mut self) -> io::Result<()>
    where
        W: Write,
    {
        self.out.flush()
    }
}

impl<'t, 'b, Top, Bottom, T, B, W> Over<&'b mut Screen<Bottom, B, W>> for &'t Layer<Top, T>
where
    T: Options,
    B: Options,
    &'t Top: GridRows,
    &'b mut Bottom: GridRows,
    <&'t Top as Grid>::Item: Over<<&'b mut Bottom as Grid>::Item>,
{
    type Output = ();

    fn over(self, bottom: &'b mut Screen<Bottom, B, W>) {
        <&'t Layer<Top, T> as Over<&'b mut Layer<Bottom, B>>>::over(self, &mut bottom.layer);
    }
}

impl<'a, G, O, W> Paint for &'a mut Screen<G, O, W>
where
    O: Options,
    &'a mut G: GridRows,
    <&'a mut G as Grid>::Item: AsMut<Cell>,
{
    type Output = ();

    fn paint(self, painter: impl Painter) {
        self.layer.paint(painter);
    }
}

impl<'a, G, W: Write> Render for &'a mut Screen<G, Cell, W>
where
    &'a G: GridRows<Item = &'a Cell>,
{
    fn render(self) -> io::Result<()> {
        (&self.layer, &mut self.out).render()
    }
}

impl<'a, G: 'a, W: Write> Render for &'a mut Screen<G, Damaged, W>
where
    &'a mut G: GridRows<Item = &'a mut Damaged>,
{
    fn render(self) -> io::Result<()> {
        (&mut self.layer, &mut self.out).render()
    }
}

impl<G, O: Options, W> AsRef<Layer<G, O>> for Screen<G, O, W> {
    fn as_ref(&self) -> &Layer<G, O> {
        &self.layer
    }
}

impl<G, O: Options, W> AsMut<Layer<G, O>> for Screen<G, O, W> {
    fn as_mut(&mut self) -> &mut Layer<G, O> {
        &mut self.layer
    }
}

impl<G: Debug, W> Debug for Screen<G, Cell, W> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Screen")
            .field("position", &self.layer)
            .finish()
    }
}

impl<G: Debug, W> Debug for Screen<G, Damaged, W> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Screen")
            .field("layer", &self.layer)
            .finish()
    }
}
