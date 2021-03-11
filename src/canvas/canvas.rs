use crate::canvas::*;

pub struct Canvas {
    grid: RowGrid1D<DamageCell, Vec<DamageCell>>,
}

impl Deref for Canvas {
    type Target = RowGrid1D<DamageCell, Vec<DamageCell>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

// impl<'a> Grid for &'a mut Canvas {
// type Item = &'a mut DamageCell;
//
// unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
// (&mut self.grid).item_unchecked(index)
// }
// }
//
// impl<'a> GridRow for &'a mut Canvas {
// type Row = <&'a mut RowGrid1D<DamageCell, Vec<DamageCell>> as GridRow>::Row;
//
// unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
// (&mut self.grid).row_unchecked(index)
// }
// }
//
// impl<'a> GridRows for &'a mut Canvas {
// type Rows = <&'a mut RowGrid1D<DamageCell, Vec<DamageCell>> as
// GridRows>::Rows;
//
// unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
// (&mut self.grid).rows_unchecked(index)
// }
// }

impl Canvas {
    pub fn new(size: Size, cell: Cell<Rgb>) -> Self {
        let len = size.x * size.y;

        let mut cells = Vec::with_capacity(len);
        cells.resize(len, cell.into());

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
                .for_each(|(canvas_cell, layer_cell)| {
                    canvas_cell.new = layer_cell.over(canvas_cell.new);
                });
        }
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
