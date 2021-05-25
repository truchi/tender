use super::*;

#[derive(Debug)]
pub struct Frame<'a, Canvas> {
    pub(super) rect:   Rect,
    pub(super) screen: &'a mut Screen<Canvas>,
}

impl<'a, Canvas> Frame<'a, Canvas> {
    pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame<Canvas>> {
        let rect = rect.checked(self.rect.size())?;
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
        let screen = &mut self.screen;
        let rect = self.rect.clone();

        // SAFETY: rect is checked at creation
        debug_assert!(rect.clone().checked((&screen.canvas).size()).is_some());
        render(
            screen.position + rect.start(),
            unsafe { screen.canvas.crop_unchecked(rect) },
            &mut screen.stdout,
        )
    }

    pub fn render_damage<'b>(&'b mut self) -> io::Result<()>
    where
        &'b mut Canvas: GridRows,
        <&'b mut Canvas as Grid>::Item: AsMut<Damaged>,
    {
        let screen = &mut self.screen;
        let rect = self.rect.clone();

        // SAFETY: rect is checked at creation
        render_damage(
            screen.position + rect.start(),
            unsafe { screen.canvas.crop_unchecked(rect) },
            &mut screen.stdout,
        )
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
