use crate::canvas::*;
use std::io::Write;

pub struct Canvas {
    first:  bool,
    styles: DedupStyles,
    grid:   RowGrid1D<DamageCell, Vec<DamageCell>>,
}

impl Deref for Canvas {
    type Target = RowGrid1D<DamageCell, Vec<DamageCell>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl Canvas {
    pub fn new(size: Size, background: Rgb) -> Self {
        let len = size.x * size.y;
        let styles = Styles::<Rgb>::default().set_background(background);

        let mut cells = Vec::with_capacity(len);
        cells.resize(len, DamageCell::from(Cell { char: ' ', styles }).into());

        Self {
            first:  true.into(),
            styles: Default::default(),
            grid:   RowGrid1D::new_unchecked(size, cells),
        }
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
                .for_each(DamageCell::over)
        }
    }

    pub fn render(&mut self, w: &mut impl Write) {
        let mut items = unsafe { (&mut self.grid).items_unchecked(..) };

        if let Some(cell) = items.next() {
            write!(w, "\x1B[1;1;H{}", cell.new).unwrap();
            self.styles = DedupStyles::new(cell.new.styles);
            cell.old = cell.new;

            for cell in items {
                self.styles.update(cell.new.styles);
                write!(w, "{}{}", self.styles, cell.new.char).unwrap();
                cell.old = cell.new;
            }
        }
    }

    pub fn render_damage(&mut self, w: &mut impl Write) {
        let mut items = unsafe { (&mut self.grid).items_unchecked(..) };

        if let Some(cell) = items.next() {
            write!(w, "\x1B[1;1;H{}", cell.new).unwrap();
            self.styles = DedupStyles::new(cell.new.styles);
            cell.old = cell.new;

            for cell in items {
                self.styles.update(cell.new.styles);
                write!(w, "{}{}", self.styles, cell.new.char).unwrap();
                cell.old = cell.new;
            }
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
