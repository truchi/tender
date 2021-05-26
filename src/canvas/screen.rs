use super::*;

#[derive(Debug)]
pub struct Screen<'lock, Canvas> {
    pub position:      Point,
    pub(super) canvas: Canvas,
    pub(super) stdout: BufWriter<StdoutLock<'lock>>,
}

impl<'lock, Canvas> Screen<'lock, Canvas> {
    pub fn new(position: impl Index0D, canvas: Canvas, stdout: &'lock Stdout) -> Self {
        Self {
            position: position.unchecked(),
            canvas,
            stdout: BufWriter::new(stdout.lock()),
        }
    }

    pub fn size(&self) -> Size
    where
        Canvas: WithSize,
    {
        self.canvas.size()
    }

    pub fn frame(&'lock mut self, rect: impl Index2D) -> Option<Frame<Canvas>>
    where
        Canvas: WithSize,
    {
        let rect = rect.checked(self.size())?;

        Some(Frame { rect, screen: self })
    }

    pub unsafe fn frame_unchecked(&'lock mut self, rect: impl Index2D) -> Frame<Canvas>
    where
        Canvas: WithSize,
    {
        let rect = rect.unchecked(self.size());

        Frame { rect, screen: self }
    }

    pub fn render<'a>(&'a mut self) -> io::Result<()>
    where
        &'a Canvas: GridRows,
        <&'a Canvas as Grid>::Item: AsRef<Cell>,
    {
        render(self.position, &self.canvas, &mut self.stdout)
    }

    pub fn render_damage<'a>(&'a mut self) -> io::Result<()>
    where
        &'a mut Canvas: GridRows,
        <&'a mut Canvas as Grid>::Item: AsMut<Damaged>,
    {
        render_damage(self.position, &mut self.canvas, &mut self.stdout)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    pub fn as_layer_ref(&self) -> Layer<&Canvas> {
        Layer::new(Point::ZERO, &self.canvas)
    }

    pub fn as_layer_mut(&mut self) -> Layer<&mut Canvas> {
        Layer::new(Point::ZERO, &mut self.canvas)
    }
}

impl<'lock, Canvas> AsRef<Screen<'lock, Canvas>> for Screen<'lock, Canvas> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'lock, Canvas> AsMut<Screen<'lock, Canvas>> for Screen<'lock, Canvas> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'a, Canvas> Paint for &'a mut Screen<'_, Canvas>
where
    &'a mut Canvas: GridRows,
    <&'a mut Canvas as Grid>::Item: AsMut<Cell>,
{
    type Output = ();

    fn paint(self, painter: impl Painter) {
        self.as_layer_mut().paint(painter);
    }
}
