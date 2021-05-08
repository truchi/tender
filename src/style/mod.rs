pub mod attributes;
mod cell;
pub mod color;
// mod cursor;

pub use attributes::*;
pub use cell::*;
pub use color::*;
// pub use cursor::*;

pub trait Over<Bottom, Output> {
    fn over(self, bottom: Bottom) -> Output;
}

// pub trait Paint<Top, Output> {
// fn paint(self, top: Top) -> Output;
// }
//
// impl<T, Top: Over<T, Output>, Output> Paint<Top, Output> for T {
// fn paint(self, top: Top) -> Output {
// top.over(self)
// }
// }

pub struct Dedup<T>(pub T, pub T);

// Ansi Ansi  OVER ? ? => Ansi Ansi
// Ansi Ansia OVER ? ? => Ansi Ansi
//
// Rgb Rgb  OVER ? ? => Rgb Rgb
// Rgb Rgba OVER ? ? => Rgb Rgb
//
// Composited cells:
//    Rgb     Rgb  OVER bottom = top
// PreRgba    Rgb  OVER bottom = top
//
//    Rgb  PreRgba OVER    Rgb     Rgb
//    Rgb  PreRgba OVER PreRgba    Rgb
//    Rgb  PreRgba OVER    Rgb  PreRgba
//    Rgb  PreRgba OVER PreRgba PreRgba
//
// PreRgba PreRgba OVER
