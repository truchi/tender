use tender::{grid::GridRows, *};

fn main() {
    let mut canvas = canvas::Canvas::new((20, 20).into(), canvas::Cell::<style::Rgb> {
        char:   'X',
        styles: Default::default(),
    });

    let layer1 = canvas::GridLayer {
        position: (10, 10).into(),
        grid:     grid::repeat((5, 5).into(), canvas::Cell::<style::PreRgba> {
            char:   'a',
            styles: style::Styles {
                foreground: style::Foreground(style::PreRgba(0, 0, 0, 10)),
                ..Default::default()
            },
        }),
    };
    let layer2 = canvas::GridLayer {
        position: (8, 8).into(),
        grid:     grid::repeat((3, 3).into(), canvas::Cell::<style::PreRgba> {
            char:   'b',
            styles: style::Styles {
                foreground: style::Foreground(style::PreRgba(0, 0, 0, 10)),
                ..Default::default()
            },
        }),
    };

    canvas.over(layer1);
    canvas.over(layer2);

    for row in unsafe { canvas.rows_unchecked(..) } {
        let row = row
            .into_iter()
            .map(|cell| cell.new.char)
            .collect::<String>();
        println!("{}", row);
    }
}
