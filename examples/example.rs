use crossterm::{
    cursor::{Hide, Show},
    event::read,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use tender::*;

fn main() {
    enter();
    let ret = test();
    leave();
    dbg!(&ret);
    // dbg!(ret.len());
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
