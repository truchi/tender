use tender::{canvas::*, grid::*, style::*};

fn main() {
    let canvas_cell = Cell::<Rgb, _>::new(' ', Rgb(0, 0, 0), Rgb(255, 0, 0), Default::default());
    let vec = vec![canvas_cell; 4];
    let mut canvas = Layer::new((0, 0), RowVec1D::new((2, 2), vec).unwrap());
    // let mut canvas = RowVec1D::new((2, 2), vec).unwrap();
    dbg!(&canvas);

    let cell1 = Cell::<Rgb, _>::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), Default::default());
    let layer1 = Layer::new((1, 1), repeat((1, 1), cell1));
    dbg!(layer1);

    let cell2 = Cell::<Rgb, _>::new(
        '2',
        Rgb(0, 0, 255),
        Rgba(0, 255, 0, 127),
        Default::default(),
    );
    let layer2 = Layer::new((0, 0), repeat((1, 1), cell2));
    dbg!(layer2);

    (&layer1).rows(..);
    (&mut canvas).rows(..);

    (&layer1).over(&mut canvas);
    dbg!(&canvas);
}
