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

fn _main() {
    let cell = Cell::<Rgb, Rgb> {
        char:   'a',
        styles: Styles {
            foreground: Rgb(0, 255, 0).into(),
            background: Rgb(255, 255, 255).into(),
            ..Default::default()
        },
    };
    let top = Cell::<Rgba, Rgba> {
        char:   'b',
        styles: Styles {
            foreground: Rgba(0, 255, 0, 255 / 2).into(),
            background: Rgba(255, 0, 0, 255 / 2).into(),
            ..Default::default()
        },
    };

    // println!("{:#?}", top.over(cell));

    let red = Rgba(255, 0, 0, 255 / 2);
    let green = Rgb(0, 255, 0);
    println!("{:#?}", red.over(green));
    println!("{:#?}", Rgba(255, 0, 0, 255 / 2).over(Rgb(255, 255, 255)));
    println!("{:#?}", Rgba(255, 255, 255, 0).over(Rgb(255, 255, 255)));
}

fn main() {
    let mut canvas = Canvas::new((151, 40).into(), Rgb(255, 0, 0));

    let layer1 = GridLayer {
        position: (1, 1).into(),
        grid:     repeat((40, 20), Cell::<PreRgba> {
            char:   'a',
            styles: Styles {
                foreground: Rgba(0, 255, 0, 255).into(),
                background: Rgba(0, 0, 0, 255 / 2).into(),
                ..Default::default()
            },
        }),
    };
    let layer2 = GridLayer {
        position: (2, 2).into(),
        grid:     repeat((40, 20), Cell::<PreRgba> {
            char:   'b',
            styles: Styles {
                foreground: Rgba(0, 0, 255, 255 / 2).into(),
                background: Rgba(0, 255, 0, 255 / 2).into(),
                ..Default::default()
            },
        }),
    };

    enter();

    canvas.render(&mut stdout());
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    canvas.over(layer1);

    canvas.render(&mut stdout());
    stdout().flush().unwrap();
    sleep(Duration::from_millis(500));

    canvas.over(layer2);

    canvas.render(&mut stdout());
    stdout().flush().unwrap();

    leave();
}

fn enter() {
    execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
    enable_raw_mode().unwrap();
}

fn leave() {
    // read().unwrap();
    sleep(Duration::from_millis(500));
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
}
