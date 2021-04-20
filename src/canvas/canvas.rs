use crate::canvas::*;
use std::{
    fmt::{self, Display, Formatter},
    io::Write,
};

pub struct Canvas {
    first:  bool,
    styles: Styles<Rgb>,
    grid:   RowVec1D<DamageCell>,
}

impl Deref for Canvas {
    type Target = RowVec1D<DamageCell>;

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

        // SAFETY: len == x * y
        let grid = unsafe { RowVec1D::new_unchecked(size, cells) };

        Self {
            first: true.into(),
            styles: Default::default(),
            grid,
        }
    }

    pub fn over<T: GridRows<Item = Cell<PreRgba>> + WithPosition>(&mut self, layer: T) {
        (&mut self.grid)
            .zip_at(layer.position(), layer)
            .flatten_rows()
            .for_each(DamageCell::over);
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
            Self::render_first_cell(w, cell, &mut self.styles);

            for cell in items {
                Self::render_dedup_cell(w, cell, &mut self.styles);
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
                    write!(w, "{}", move_to).unwrap();
                }

                Self::render_dedup_cell(w, cell, &mut self.styles);
                rendered = true;
            }

            move_to.next();
        }
    }

    fn render_first_cell<T: Write>(w: &mut T, cell: &mut DamageCell, styles: &mut Styles<Rgb>) {
        write!(w, "\x1B[1;1;H{}", cell.new).unwrap();

        *styles = cell.new.styles;
        cell.old = cell.new;
    }

    fn render_dedup_cell<T: Write>(w: &mut T, cell: &mut DamageCell, styles: &mut Styles<Rgb>) {
        cell.new.styles.render_dedup(w, styles);
        write!(w, "{}", cell.new.char).unwrap();

        *styles = cell.new.styles;
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
