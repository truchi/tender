use super::*;

pub fn example() {
    use std::{thread::sleep, time::Duration};

    let (w, h) = (151, 40);
    let mut screen = Screen::new(
        (0, 0),
        RowVec1D::new((w, h), vec![
            Damaged::new(Cell::new(' ', BLACK, RED, ()));
            w * h
        ])
        .unwrap(),
    );
    let layer1 = Layer::new(
        (0, 0),
        repeat((12, 12), Cell::new('1', LIME, BLACK.alpha(127), ())),
    );
    let layer2 = Layer::new(
        (2, 2),
        repeat((10, 10), Cell::new('2', BLUE, BLACK.alpha(127), ())),
    );

    screen.render().unwrap();
    screen.flush().unwrap();
    sleep(Duration::from_millis(500));

    screen.as_layer_mut().under(layer1.as_ref());
    screen.paint(Italic);
    screen.render_damage().unwrap();
    screen.flush().unwrap();
    sleep(Duration::from_millis(500));

    let frame = &mut screen.frame((10..30, 10..30)).unwrap();
    frame.under(layer2.as_ref());
    frame.bold();
    frame.render_damage().unwrap();
    frame.flush().unwrap();
    sleep(Duration::from_millis(500));

    let frame = &mut frame.frame((4..10, 4..10)).unwrap();
    frame.foreground(PURPLE);
    frame.striked();
    frame.render_damage().unwrap();
    frame.flush().unwrap();
}

pub fn example2() -> (f64, std::time::Duration, std::time::Duration) {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    let (w, h) = (150, 40);
    let mut screen = Screen::new(
        (0, 0),
        RowVec1D::new((w, h), vec![Damaged::default(); w * h]).unwrap(),
    );
    let background = Layer::new(
        (0, 0),
        repeat_with((w, h), |Point { x, .. }| {
            if x % 2 == 0 {
                Cell::new(' ', RED, GREEN, ())
            } else {
                Cell::new(' ', GREEN, RED, ())
            }
        }),
    );

    screen.as_layer_mut().under(background.as_ref());
    screen.render().unwrap();
    screen.flush().unwrap();

    let fps = 10.0;
    let spf = Duration::from_secs_f64(1.0 / fps);
    let now = Instant::now();
    let mut lag = Duration::from_millis(0);

    let margin = 0;
    for i in 0..(fps as u8 * 3) {
        let before = Instant::now();
        screen
            .frame((margin..w - margin, margin / 2..h - margin / 2))
            .unwrap()
            .char(if i % 2 == 0 { '▄' } else { '▀' });
        // screen.char(if i % 2 == 0 { '▄' } else { '▀' });
        screen.render().unwrap();
        screen.flush().unwrap();
        let after = before.elapsed();
        if spf > after {
            sleep(spf - after);
        } else {
            lag += after - spf;
        }
    }
    let elapsed = now.elapsed();

    screen.paint((' ', Background(WHITE)));
    screen.render().unwrap();
    screen.flush().unwrap();

    (fps, elapsed, lag)
}

pub fn example3() -> String {
    let (w, h) = (150, 40);
    let mut screen = Layer::new(
        (0, 0),
        RowVec1D::new((w, h), vec![Damaged::default(); w * h]).unwrap(),
    );
    let background = Layer::new(
        (0, 0),
        repeat_with((w, h), |Point { x, .. }| {
            if x % 2 == 0 {
                Cell::new('A', RED, BLUE, ())
                // Cell::new('A', RED, BLUE, (NoWeight, NoSlant, NoUnderline,
                // NoStrike))
            } else {
                Cell::new('B', BLUE, RED, ())
                // Cell::new('B', BLUE, RED, (Bold, Italic, Underlined,
                // Striked))
            }
        }),
    );

    let mut vec = vec![];
    screen.as_mut().under(background.as_ref());
    screen.render(&mut vec).unwrap();

    String::from_utf8(vec).unwrap()
}

pub fn example4() -> (f64, std::time::Duration, std::time::Duration) {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    let (w, h) = (150, 40);
    let mut screen = Screen::new(
        (0, 0),
        RowVec1D::new((w, h), vec![
            Damaged::new(Cell::new(' ', WHITE, BLACK, ()));
            w * h
        ])
        .unwrap(),
    );

    screen.render().unwrap();
    screen.flush().unwrap();

    let fps = 60.0;
    let spf = Duration::from_secs_f64(1.0 / fps);
    let now = Instant::now();
    let mut lag = Duration::from_millis(0);

    for i in 0..140 {
        let before = Instant::now();
        let frame = &mut screen.frame((i..i + 10, 10..20)).unwrap();
        let layer = Layer::new((0, 0), repeat((10, 10), Cell::new(' ', RED, RED, ())));

        frame.under(layer.as_ref());
        screen.render_damage().unwrap();
        screen.flush().unwrap();

        let after = before.elapsed();
        if spf > after {
            sleep(spf - after);
        } else {
            lag += after - spf;
        }
    }
    let elapsed = now.elapsed();

    screen.background(WHITE);
    screen.render().unwrap();
    screen.flush().unwrap();

    (fps, elapsed, lag)
}

pub fn example5() -> (f64, std::time::Duration, std::time::Duration) {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    let (w, h) = (150, 40);
    let mut screen = Screen::new(
        (0, 0),
        RowVec1D::new((w, h), vec![
            Damaged::new(Cell::new(' ', WHITE, BLACK, ()));
            w * h
        ])
        .unwrap(),
    );

    screen.render().unwrap();
    screen.flush().unwrap();

    let fps = 5.0;
    let spf = Duration::from_secs_f64(1.0 / fps);
    let now = Instant::now();
    let mut lag = Duration::from_millis(0);

    for i in 0..(fps as u8 * 2) {
        let before = Instant::now();
        let color = if i % 2 == 0 { BLUE } else { RED };
        screen.background(color);
        // screen.frame((20..30, 20..30)).unwrap().background(color);
        screen.render().unwrap();
        screen.flush().unwrap();

        let after = before.elapsed();
        if spf > after {
            sleep(spf - after);
        } else {
            lag += after - spf;
        }
    }
    let elapsed = now.elapsed();

    screen.background(WHITE);
    screen.render().unwrap();
    screen.flush().unwrap();

    (fps, elapsed, lag)
}

pub fn example6() -> (f64, std::time::Duration, std::time::Duration) {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    let (w, h) = (150, 40);
    let size = 10;
    let mut screen = Screen::new(
        (0, 0),
        RowVec1D::new((w, h), vec![Damaged::default(); w * h]).unwrap(),
    );
    let background = Layer::new((0, 0), repeat((w, h), Cell::new(' ', WHITE, WHITE, ())));
    let red = Layer::new(
        (0, 0),
        repeat((2 * size, size), Cell::new(' ', RED, RED, ())),
    );
    let green = Layer::new(
        (0, 0),
        repeat((2 * size, size), Cell::new(' ', GREEN, GREEN, ())),
    );
    let blue = Layer::new(
        (0, 0),
        repeat((2 * size, size), Cell::new(' ', BLUE, BLUE, ())),
    );
    let _purple = Layer::new(
        (0, 0),
        repeat((2 * size, size), Cell::new(' ', PURPLE, PURPLE, ())),
    );
    let pink = Layer::new(
        (0, 0),
        repeat((2 * size, size), Cell::new(' ', PINK, PINK, ())),
    );

    screen.render().unwrap();
    screen.flush().unwrap();

    let fps = 60.0;
    let spf = Duration::from_secs_f64(1.0 / fps);
    let now = Instant::now();
    let mut lag = Duration::from_millis(0);

    for i in 0..(fps as usize * 5) {
        let before = Instant::now();

        screen.as_layer_mut().under(background.as_ref());
        screen
            .frame((w / 2 - size..w / 2 + size, (i % h)..((i % h) + size).min(h)))
            .as_mut()
            .unwrap()
            .under(red.as_ref());
        screen
            .frame((
                (i % w)..((i % w) + 2 * size).min(w),
                (h - size) / 2..(h + size) / 2,
            ))
            .as_mut()
            .unwrap()
            .under(green.as_ref());
        screen
            .frame((
                (i % w)..((i % w) + 2 * size).min(w),
                (i % h)..((i % h) + size).min(h),
            ))
            .as_mut()
            .unwrap()
            .under(blue.as_ref());
        screen
            .frame((
                (w - (i % w))..((w - (i % w)) + 2 * size).min(w),
                (h - (i % h))..((h - (i % h)) + size).min(h),
            ))
            .as_mut()
            .unwrap()
            .under(pink.as_ref());
        screen.render_damage().unwrap();
        screen.flush().unwrap();

        let after = before.elapsed();
        if spf > after {
            sleep(spf - after);
        } else {
            lag += after - spf;
        }
    }
    let elapsed = now.elapsed();

    screen.background(WHITE);
    screen.render().unwrap();
    screen.flush().unwrap();

    (fps, elapsed, lag)
}

pub fn example7() -> (f64, std::time::Duration, std::time::Duration) {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    let out = stdout();
    let mut out = out.lock();
    let (w, h) = (150, 40);

    let fps = 20.0;
    let spf = Duration::from_secs_f64(1.0 / fps);
    let now = Instant::now();
    let mut lag = Duration::from_millis(0);
    let layer1 = Layer::new(
        (0, 0),
        repeat_with((w, h), |Point { x, .. }| {
            if x % 2 == 0 {
                Cell::new('▄', RED, GREEN, ())
            } else {
                Cell::new('▀', GREEN, RED, ())
            }
        }),
    );
    let layer2 = Layer::new(
        (0, 0),
        repeat_with((w, h), |Point { x, .. }| {
            if x % 2 == 0 {
                Cell::new('▄', GREEN, RED, ())
            } else {
                Cell::new('▀', RED, GREEN, ())
            }
        }),
    );

    for i in 0..(fps as u8 * 3) {
        let before = Instant::now();

        if i % 2 == 0 { layer1 } else { layer2 }
            .render(&mut out)
            .unwrap();
        out.flush().unwrap();

        let after = before.elapsed();
        if spf > after {
            sleep(spf - after);
        } else {
            lag += after - spf;
        }
    }
    let elapsed = now.elapsed();

    (fps, elapsed, lag)
}

pub fn example8() -> (f64, std::time::Duration, std::time::Duration) {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    let out = stdout();
    let mut out = out.lock();
    let (w, h) = (150, 40);

    let fps = 30.0;
    let spf = Duration::from_secs_f64(1.0 / fps);
    let now = Instant::now();

    let mut lag = Duration::from_millis(0);
    let mut layer = Vec::with_capacity(w * h);
    for _ in 0..h {
        for i in 0..w {
            layer.push(Damaged::new(if i % 2 == 0 {
                Cell::new('X', RED, GREEN, ())
            } else {
                Cell::new('O', GREEN, RED, ())
            }));
        }
    }
    let mut layer = Layer::new((0, 0), RowVec1D::new((w, h), layer).unwrap());

    layer.render(&mut out).unwrap();
    out.flush().unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut previous: Option<(Cell, usize, usize)> = None;
    for _ in 0..(fps as u64 * 3) {
        let before = Instant::now();

        if let Some((cell, x, y)) = previous {
            (&mut layer.grid).item((x, y)).unwrap().current = cell;
        }

        let cell: &mut Cell = &mut (&mut layer.grid).item((x, y)).unwrap().current;
        previous = Some((*cell, x, y));

        *cell = Default::default();

        if x + 1 < w {
            x += 1;
        } else if y + 1 < h {
            x = 0;
            y += 1;
        } else {
            x = 0;
            y = 0;
        }

        layer.render_damage(&mut out).unwrap();
        out.flush().unwrap();

        let after = before.elapsed();
        if spf > after {
            sleep(spf - after);
        } else {
            lag += after - spf;
        }
    }
    let elapsed = now.elapsed();

    (fps, elapsed, lag)
}
