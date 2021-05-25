mod layer;
// mod canvas;
// mod cell;

pub use layer::*;
// pub use canvas::*;
// pub use cell::*;

use crate::{geometry::*, grid::*, style::*};
use std::{
    io::{self, Stdout, Write},
    ops::{Deref, DerefMut},
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

    // pub fn as_layer_ref(&self) -> Layer<&Canvas> {
    // Layer::new(Point::ZERO, &self.canvas)
    // }
    //
    // pub fn as_layer_mut(&mut self) -> Layer<&mut Canvas> {
    // Layer::new(Point::ZERO, &mut self.canvas)
    // }

    pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame<Self>>
    where
        Canvas: WithSize,
    {
        let rect = rect.checked(self.canvas.size())?;

        Some(Frame { rect, frame: self })
    }

    pub unsafe fn frame_unchecked(&mut self, rect: impl Index2D) -> Frame<Self>
    where
        Canvas: WithSize,
    {
        let rect = rect.unchecked(self.canvas.size());

        Frame { rect, frame: self }
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
pub struct Frame<'a, T> {
    rect:  Rect,
    frame: &'a mut T,
}

impl<'a, T> Frame<'a, T> {
    // pub fn as_layer_ref<'b, Canvas: 'b>(&'b self) -> Layer<Crop<&Canvas>>
    // where
    // T: AsRef<Screen<Canvas>>,
    // &'b Canvas: Grid,
    // {
    // Layer::new(Point::ZERO, unsafe {
    // self.frame.as_ref().canvas.crop_unchecked(self.rect.clone())
    // })
    // }
    //
    // pub fn as_layer_mut<'b, Canvas: 'b>(&'b mut self) -> Layer<Crop<&mut Canvas>>
    // where
    // T: AsMut<Screen<Canvas>>,
    // &'b mut Canvas: Grid,
    // {
    // Layer::new(Point::ZERO, unsafe {
    // self.frame.as_mut().canvas.crop_unchecked(self.rect.clone())
    // })
    // }

    pub fn frame2(&mut self, rect: impl Index2D) -> Option<Frame<T>> {
        let rect = rect.checked(self.rect.size())?;
        let rect = rect.translate(self.rect.start());

        Some(Frame {
            rect,
            frame: self.frame,
        })
    }

    pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame<Self>> {
        let rect = rect.checked(self.rect.size())?;
        let rect = rect.translate(self.rect.start());

        Some(Frame { rect, frame: self })
    }

    pub unsafe fn frame_unchecked(&mut self, rect: impl Index2D) -> Frame<Self> {
        let rect = rect.unchecked(self.rect.size());
        let rect = rect.translate(self.rect.start());

        Frame { rect, frame: self }
    }

    pub fn render<'b, Canvas: 'b>(&'b mut self) -> io::Result<()>
    where
        T: AsMut<Screen<Canvas>>,
        &'b Canvas: GridRows,
        <&'b Canvas as Grid>::Item: AsRef<Cell>,
    {
        let screen = self.frame.as_mut();
        let rect = self.rect.clone();

        // SAFETY: rect is checked at creation
        debug_assert!(rect.clone().checked((&screen.canvas).size()).is_some());
        render(
            screen.position + rect.start(),
            unsafe { screen.canvas.crop_unchecked(rect) },
            &mut screen.stdout,
        )
    }

    pub fn render_damage<'b, Canvas: 'b>(&'b mut self) -> io::Result<()>
    where
        T: AsMut<Screen<Canvas>>,
        &'b mut Canvas: GridRows,
        <&'b mut Canvas as Grid>::Item: AsMut<Damaged>,
    {
        let screen = self.frame.as_mut();

        // SAFETY: rect is checked at creation
        render_damage(
            screen.position,
            unsafe { screen.canvas.crop_unchecked(self.rect.clone()) },
            &mut screen.stdout,
        )
    }
}

impl<'a, T: AsRef<Screen<Canvas>>, Canvas> AsRef<Screen<Canvas>> for Frame<'a, T> {
    fn as_ref(&self) -> &Screen<Canvas> {
        self.frame.as_ref()
    }
}

impl<'a, T: AsMut<Screen<Canvas>>, Canvas> AsMut<Screen<Canvas>> for Frame<'a, T> {
    fn as_mut(&mut self) -> &mut Screen<Canvas> {
        self.frame.as_mut()
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
