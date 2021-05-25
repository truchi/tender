mod layer;
mod move_to;
// mod canvas;
// mod cell;

pub use layer::*;
pub use move_to::*;
// pub use canvas::*;
// pub use cell::*;

use crate::{geometry::*, grid::*, style::*};
use std::{
    io::{self, Stdout, Write},
    ops::Deref,
};

pub trait WithPosition {
    fn position(&self) -> Point;
}

impl<T: Deref<Target = U>, U: WithPosition> WithPosition for T {
    fn position(&self) -> Size {
        self.deref().position()
    }
}

#[derive(Debug)]
pub struct Screen<Canvas> {
    position: Point,
    canvas:   Canvas,
    stdout:   Stdout,
}

impl<Canvas> Screen<Canvas> {
    pub fn new(position: impl Index0D, canvas: Canvas, stdout: Stdout) -> Self {
        Self {
            position: position.unchecked(),
            canvas,
            stdout,
        }
    }

    pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame<Canvas>>
    where
        Canvas: WithSize,
    {
        let rect = rect.checked(self.canvas.size())?;

        Some(Frame { rect, screen: self })
    }

    pub unsafe fn frame_unchecked(&mut self, rect: impl Index2D) -> Frame<Canvas>
    where
        Canvas: WithSize,
    {
        let rect = rect.unchecked(self.canvas.size());

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
}

impl<Canvas> AsRef<Screen<Canvas>> for Screen<Canvas> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Canvas> AsMut<Screen<Canvas>> for Screen<Canvas> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[derive(Debug)]
pub struct Frame<'a, Canvas> {
    rect:   Rect,
    screen: &'a mut Screen<Canvas>,
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

pub fn render<T>(position: Point, grid: T, mut w: impl Write) -> io::Result<()>
where
    T: GridRows,
    T::Item: AsRef<Cell>,
{
    fn render_row<C: AsRef<Cell>>(
        mut w: impl Write,
        row: impl IntoIterator<Item = C>,
        previous: &mut Cell,
        move_to: &mut MoveTo,
    ) -> io::Result<()> {
        for icell in row {
            let cell = *icell.as_ref();
            write!(w, "{}", Dedup(*previous, cell))?;
            *previous = cell;
        }
        move_to.next_row();

        Ok(())
    }

    let mut rows = unsafe { grid.rows_unchecked(..) }.into_iter();
    let mut move_to = MoveTo::new(position);

    if let Some(row) = rows.next() {
        let mut row = row.into_iter();

        // Render first cell as is
        if let Some(cell) = row.next() {
            let mut previous = *cell.as_ref();
            write!(w, "{}{}", move_to, previous)?;

            // Finish rendering this row, deduping
            render_row(&mut w, row, &mut previous, &mut move_to)?;

            // Render remaining rows, deduping
            for row in rows {
                write!(w, "{}", move_to)?;
                render_row(&mut w, row, &mut previous, &mut move_to)?;
            }

            // Done
            return Ok(());
        }
    }

    // Was empty
    Ok(())
}

pub fn render_damage<T>(position: Point, grid: T, mut w: impl Write) -> io::Result<()>
where
    T: GridRows,
    T::Item: AsMut<Damaged>,
{
    fn render_row_damage<C: AsMut<Damaged>>(
        mut w: impl Write,
        row: impl IntoIterator<Item = C>,
        previous: &mut Cell,
        move_to: &mut MoveTo,
        mut rendered: bool,
    ) -> io::Result<()> {
        for mut damaged in row {
            if let Some(cell) = damaged.as_mut().damage() {
                if !rendered {
                    write!(w, "{}", move_to)?;
                }
                write!(w, "{}", Dedup(*previous, cell))?;
                *previous = cell;
                rendered = true;
            } else {
                rendered = false;
            }
            move_to.next_col();
        }
        move_to.next_row();

        Ok(())
    }

    let mut rows = unsafe { grid.rows_unchecked(..) }.into_iter();
    let mut move_to = MoveTo::new(position);

    // We start looking for a cell that has damage
    while let Some(row) = rows.next() {
        move_to.first_col();

        let mut row = row.into_iter();
        while let Some(mut damaged) = row.next() {
            // Render first cell with damage as is
            if let Some(cell) = damaged.as_mut().damage() {
                let mut previous = cell;

                write!(w, "{}{}", move_to, previous)?;

                // Finish rendering this row, deduping
                move_to.next_col();
                render_row_damage(&mut w, row, &mut previous, &mut move_to, true)?;

                // Render remaining rows, deduping
                while let Some(row) = rows.next() {
                    move_to.first_col();
                    render_row_damage(&mut w, row, &mut previous, &mut move_to, false)?;
                }

                // Done
                return Ok(());
            }
            move_to.next_col();
        }
        move_to.next_row();
    }

    // Was empty or undamaged
    Ok(())
}
