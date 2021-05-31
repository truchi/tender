// pub mod examples;

// mod frame;
mod layer;
// mod screen;

// pub use frame::*;
pub use layer::*;
// pub use screen::*;

use crate::{geometry::*, grid::*, style::*};
use std::io::{self, stdout, BufWriter, Stdout, Write};

pub trait Render: Sized {
    fn render(self) -> io::Result<()> {
        Ok(())
    }
}

pub trait First: Copy {
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

pub fn render<T>(position: Point, grid: T, mut w: impl Write) -> io::Result<()>
where
    T: GridRows,
    T::Item: ICell,
{
    fn render_row<C: ICell>(
        mut w: impl Write,
        row: impl IntoIterator<Item = C>,
        previous: &mut Cell,
        move_to: &mut MoveTo,
    ) -> io::Result<()> {
        for cell in row {
            let cell = cell.update();
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
            let mut previous = cell.update();
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
