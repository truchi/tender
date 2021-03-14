use crate::canvas::*;
use std::fmt::{self, Display, Formatter};

pub struct Canvas {
    grid: RowGrid1D<Cell<Rgb>, Vec<Cell<Rgb>>>,
}

impl Deref for Canvas {
    type Target = RowGrid1D<Cell<Rgb>, Vec<Cell<Rgb>>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl Canvas {
    pub fn new(size: Size, background: Rgb) -> Self {
        let len = size.x * size.y;

        let mut cells = Vec::with_capacity(len);
        cells.resize(len, Cell {
            char:   ' ',
            styles: Styles::<Rgb>::default()
                // .set_foreground(Rgb(100, 0, 0))
                .set_background(background),
        });
        // cells.resize(len, Cell::<Rgb>::default().set_background(background));

        debug_assert!(cells.len() == len);
        let grid = RowGrid1D::new_unchecked(size, cells);

        Self { grid }
    }

    pub fn over<T: GridRows<Item = Cell<PreRgba>> + WithPosition>(&mut self, layer: T) {
        let crop = crop(layer.position(), layer.size(), self.size());

        if let Some((canvas_rect, layer_rect)) = crop {
            // SAFETY: crop guaranties we are in bounds
            let canvas = unsafe { (&mut self.grid).crop_unchecked(canvas_rect) };
            let layer = unsafe { layer.crop_unchecked(layer_rect) };
            let zip = canvas.zip(layer);

            // SAFETY: RangeFull are safe for grids
            unsafe { zip.rows_unchecked(..) }
                .flatten()
                .for_each(|(canvas_cell, layer_cell)| *canvas_cell = layer_cell.over(*canvas_cell));
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[1;1;H")?;
        let mut items = unsafe { self.grid.items_unchecked(..) };

        if let Some(cell) = items.next() {
            let Styles {
                foreground: mut prev_foreground,
                background: mut prev_background,
                attributes:
                    Attributes {
                        weight: mut prev_weight,
                        slant: mut prev_slant,
                        underline: mut prev_underline,
                        strike: mut prev_strike,
                        overline: mut prev_overline,
                        invert: mut prev_invert,
                        blink: mut prev_blink,
                        border: mut prev_border,
                    },
            } = cell.styles;

            write!(f, "{}", cell)?;

            for cell in items {
                let Cell { char, styles } = cell;
                let Styles {
                    foreground,
                    background,
                    attributes:
                        Attributes {
                            weight,
                            slant,
                            underline,
                            strike,
                            overline,
                            invert,
                            blink,
                            border,
                        },
                } = styles;

                macro_rules! attr {
                    ($($attr:ident $prev:ident)*) => { $(
                        if *$attr != $prev {
                            write!(f, "{}", $attr)?;
                            $prev = *$attr;
                        }
                    )* };
                }

                attr!(
                    foreground prev_foreground
                    background prev_background
                    weight     prev_weight
                    slant      prev_slant
                    underline  prev_underline
                    strike     prev_strike
                    overline   prev_overline
                    invert     prev_invert
                    blink      prev_blink
                    border     prev_border
                );
                write!(f, "{}", char)?;
            }
        }

        Ok(())
    }
}

fn crop(point: Point, size: Size, at: Size) -> Option<(Rect, Rect)> {
    if point.x >= at.x || point.y >= at.y {
        return None;
    }

    let x = point.x.saturating_add(size.x).min(at.x);
    let y = point.y.saturating_add(size.y).min(at.y);

    debug_assert!(point.x <= x);
    debug_assert!(point.y <= y);

    Some((
        Coord {
            x: point.x..x,
            y: point.y..y,
        },
        Coord {
            x: 0..x - point.x,
            y: 0..y - point.y,
        },
    ))
}
