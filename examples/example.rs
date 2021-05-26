use crossterm::{
    cursor::{Hide, Show},
    event::read,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use tender::{canvas::*, grid::*, style::*};

// fn main() {
// let mut red = Rgba(255, 0, 0, 127);
// let blue = Rgba(0, 0, 255, 127);
// let over: Rgba = red.over(blue);
// let paint: Rgba = blue.paint(red);
//
// println!("{:?}", over);
// println!("{:?}", paint);
//
// (red).over(&mut blue);
// }

fn main() {
    enter();
    let ret = example2();
    leave();
    dbg!(&ret);

    // let cell = Cell::new('a', RED, WHITE, ());
    // let mut vec = vec![];
    // write!(&mut vec, "{}", cell);
    // let string = String::from_utf8(vec).unwrap();
    // dbg!(&string);
    // dbg!(string.len());
    // let cell_full =
    // "\u{1b}[38;2;255;0;0m\u{1b}[48;2;255;255;255m\u{1b}[22m\u{1b}[23m\
    // u{1b}[24m\u{1b}[29ma"; let cell_short =
    // "\u{1b}[38;2;255;0;0;48;2;255;255;255;22;23;24;29ma";
    // dbg!(&cell_full);
    // dbg!(cell_full.len());
    // dbg!(&cell_short);
    // dbg!(cell_short.len());
    //
    // dbg!(90_000.0 * cell_short.len() as f64 / cell_full.len() as f64);

    /*
        let (w, h) = (151, 40);

        let canvas_cell = Cell::<Rgb, _>::new(' ', Rgb(0, 0, 0), Rgb(255, 0, 0), ());
        let canvas_cell = DamageCell::new(canvas_cell);
        let vec = vec![canvas_cell; w * h];
        let mut canvas = Layer::new((0, 0), RowVec1D::new((w, h), vec).unwrap());

        let cell1 = Cell::<Rgb, _>::new('1', Rgb(0, 255, 0), Rgba(0, 0, 0, 127), ());
        let layer1 = Layer::new((1, 1), repeat((10, 10), cell1));

        let cell2 = Cell::<Rgb, _>::new('2', Rgb(0, 0, 255), Rgba(0, 255, 0, 127), ());
        let layer2 = Layer::new((2, 2), repeat((10, 10), cell2));

        enter();

        print!("{}", canvas);
        // println!("{:?}", canvas.to_string());
        stdout().flush().unwrap();
        sleep(Duration::from_millis(500));
        (&layer1).over(&mut canvas);

        print!("{}", canvas);
        // println!("{:?}", canvas.to_string());
        stdout().flush().unwrap();
        sleep(Duration::from_millis(500));
        (&layer2).over(&mut canvas);

        print!("{}", canvas);
        // println!("{:?}", canvas.to_string());
        stdout().flush().unwrap();
        sleep(Duration::from_millis(500));

        leave();
    */
}

fn enter() {
    execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
    enable_raw_mode().unwrap();
}

fn leave() {
    read().unwrap();
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
}
