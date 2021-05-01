use tender::{canvas::*, grid::*, style::*};

fn main() {
    let canvas_cell = Cell::<Rgb, _>::new(' ', Rgb(0, 0, 0), Rgb(255, 0, 0), Default::default());
    let mut canvas = Layer::new((0, 0), RowVec1D::new((1, 1), vec![canvas_cell; 1]).unwrap());

    let cell1 = Cell::<Rgb, _>::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), Default::default());
    let layer1 = Layer::new((0, 0), repeat((1, 1), cell1));

    let cell2 = Cell::<Rgb, _>::new(
        '2',
        Rgb(0, 0, 255),
        Rgba(0, 255, 0, 127),
        Default::default(),
    );
    // let layer2 = Layer::new((0, 0), repeat((1, 1), cell2));

    dbg!(&canvas);
    dbg!(layer1);
    // dbg!(layer2);

    (&layer1).over(&mut canvas);
    dbg!(&canvas);
}
