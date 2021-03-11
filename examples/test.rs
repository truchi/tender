use tender::{grid::GridRows, *};

fn main() {
    let mut canvas = canvas::Canvas::new((20, 20).into(), canvas::Cell::<style::Rgb> {
        char:   'X',
        styles: Default::default(),
    });

    let repeat = grid::repeat((5, 5).into(), canvas::Cell::<style::PreRgba> {
        char:   'a',
        styles: style::Styles {
            foreground: style::Foreground(style::PreRgba {
                red:   0,
                green: 0,
                blue:  0,
                alpha: 10,
            }),
            ..Default::default()
        },
    });
    let layer = canvas::GridLayer {
        position: (10, 10).into(),
        grid:     repeat,
    };

    canvas.over(layer);

    for row in unsafe { canvas.rows_unchecked(..) } {
        let row = row
            .into_iter()
            .map(|cell| cell.new.char)
            .collect::<String>();
        println!("{}", row);

        // for cell in row {
        // print!("{:?}", cell.new.char);
        // }
        // println!();
    }
}
