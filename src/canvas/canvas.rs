use crate::canvas::*;
use std::{
    fmt::{self, Display, Formatter},
    io::Write,
};

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

    pub fn render<T: Write>(&mut self, w: &mut T) {
        if self.first == true {
            self.first = false;
            self.render_initial(w);
        } else {
            self.render_damage(w);
        }
    }

    pub fn render_initial<T: Write>(&mut self, w: &mut T) {
        let mut items = unsafe { (&mut self.grid).items_unchecked(..) };

        if let Some(cell) = items.next() {
            write!(w, "\x1B[1;1;H{}", cell.new).unwrap();

            self.styles = DedupStyles::new(cell.new.styles);
            cell.old = cell.new;

            for cell in items {
                Self::render_cell(w, cell, &mut self.styles);
            }
        }
    }

    pub fn render_damage<T: Write>(&mut self, w: &mut T) {
        let mut move_to = MoveTo::new(self.grid.size());
        let mut items = unsafe { (&mut self.grid).items_unchecked(..) };
        let mut rendered = false;

        while let Some(cell) = items.next() {
            if cell.new == cell.old {
                rendered = false;
            } else {
                if !rendered {
                    // dbg!(move_to.point);
                    write!(w, "{}", move_to).unwrap();
                }

                Self::render_cell(w, cell, &mut self.styles);
                rendered = true;
            }

            move_to.next();
        }
    }

    fn render_cell<T: Write>(w: &mut T, cell: &mut DamageCell, styles: &mut DedupStyles) {
        styles.update(cell.new.styles);
        write!(w, "{}{}", styles, cell.new.char).unwrap();
        cell.old = cell.new;
    }
}

#[derive(Debug)]
struct MoveTo {
    size:  Size,
    point: Point,
}

impl MoveTo {
    fn new(size: Size) -> Self {
        Self {
            size,
            point: (0, 0).into(),
        }
    }

    fn next(&mut self) {
        if self.point.x == self.size.x - 1 {
            self.point.x = 0;
            self.point.y += 1;
        } else {
            self.point.x += 1;
        }
    }
}

impl Display for MoveTo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.point.y + 1, self.point.x + 1)
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
