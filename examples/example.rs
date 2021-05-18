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
