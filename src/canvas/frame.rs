use super::*;

#[derive(Debug)]
pub struct Frame<'a, Canvas> {
    pub(super) rect:   Rect,
    pub(super) screen: &'a mut Screen<Canvas>,
}

impl<'a, Canvas> Frame<'a, Canvas> {
    pub fn position(&self) -> Point {
        self.screen.position + self.rect.start()
    }

    pub fn size(&self) -> Size {
        self.rect.size()
    }

    pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame<Canvas>> {
        let rect = rect.checked(self.size())?;
        let rect = rect.translate(self.rect.start());

        Some(Frame {
            rect,
            screen: self.screen,
        })
    }

    pub unsafe fn frame_unchecked(&mut self, rect: impl Index2D) -> Frame<Canvas> {
        let rect = rect.unchecked(self.rect.size());
        let rect = rect.translate(self.rect.start());

        Frame {
            rect,
            screen: self.screen,
        }
    }

    pub fn render<'b>(&'b mut self) -> io::Result<()>
    where
        &'b Canvas: GridRows,
        <&'b Canvas as Grid>::Item: AsRef<Cell>,
    {
        let position = self.position();
        let rect = self.rect.clone();
        let Screen { canvas, stdout, .. } = &self.screen;

        // SAFETY: rect is checked at creation
        debug_assert!(rect.clone().checked((canvas).size()).is_some());
        render(position, unsafe { canvas.crop_unchecked(rect) }, stdout)
    }

    pub fn render_damage<'b>(&'b mut self) -> io::Result<()>
    where
        &'b mut Canvas: GridRows,
        <&'b mut Canvas as Grid>::Item: AsMut<Damaged>,
    {
        let position = self.position();
        let rect = self.rect.clone();
        let Screen { canvas, stdout, .. } = &mut self.screen;

        // SAFETY: rect is checked at creation
        render_damage(position, unsafe { canvas.crop_unchecked(rect) }, stdout)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.screen.flush()
    }
}

impl<Canvas> AsRef<Screen<Canvas>> for Frame<'_, Canvas> {
    fn as_ref(&self) -> &Screen<Canvas> {
        self.screen.as_ref()
    }
}

impl<Canvas> AsMut<Screen<Canvas>> for Frame<'_, Canvas> {
    fn as_mut(&mut self) -> &mut Screen<Canvas> {
        self.screen.as_mut()
    }
}
