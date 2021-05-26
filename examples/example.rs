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
