pub mod attributes;
pub mod color;

mod cell;
mod comp;
mod damaged;
mod paint;

pub use attributes::*;
pub use cell::*;
pub use color::*;
pub use comp::*;
pub use damaged::*;
pub use paint::*;

use std::fmt::{self, Display, Formatter};

pub trait Over<Bottom> {
    type Output;

    fn over(self, bottom: Bottom) -> Self::Output;
}

impl<Top: Over<Bottom, Output = Bottom> + Copy, Bottom: Copy> Over<&mut Bottom> for &Top {
    type Output = ();

    fn over(self, bottom: &mut Bottom) {
        *bottom = (*self).over(*bottom);
    }
}

impl<Top: Over<Bottom, Output = Top> + Copy, Bottom: Copy> Over<&Bottom> for &mut Top {
    type Output = ();

    fn over(self, bottom: &Bottom) {
        *self = (*self).over(*bottom);
    }
}

pub trait Under<Top> {
    type Output;

    fn under(self, top: Top) -> Self::Output;
}

impl<Top: Over<Bottom>, Bottom> Under<Top> for Bottom {
    type Output = Top::Output;

    fn under(self, top: Top) -> Self::Output {
        top.over(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Dedup<T>(pub T, pub T);

#[derive(Copy, Clone, Debug)]
pub struct CSI<T>(pub T);

impl<T: Copy> Display for CSI<T>
where
    CS<T>: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[{}m", CS(self.0))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CS<T>(pub T);

macro_rules! csi_tuples {
    ($([$fmt:literal $($field:tt $T:ident)*])*) => {
        $(impl<$($T,)*> Display for CS<Dedup<($($T,)*)>>
        where
            $($T: PartialEq + Copy,)*
            $(CS<$T>: Display,)*
        {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let Dedup(previous, current) = self.0;
                let mut separator = false;

                $(if previous.$field != current.$field {
                    if separator {
                        write!(f, ";{}", CS(current.$field))?;
                    } else {
                        write!(f, "{}", CS(current.$field))?;
                        separator = true;
                    }
                })*

                Ok(())
            }
        })*

        $(impl<$($T,)*> Display for CS<($($T,)*)>
        where
            $($T: Copy,)*
            $(CS<$T>: Display,)*
        {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, $fmt, $(CS(self.0.$field),)*)
            }
        })*
    };
}

csi_tuples!(
    ["{};{}"             0 T1 1 T2]
    ["{};{};{}"          0 T1 1 T2 2 T3]
    ["{};{};{};{}"       0 T1 1 T2 2 T3 3 T4]
    ["{};{};{};{};{}"    0 T1 1 T2 2 T3 3 T4 4 T5]
    ["{};{};{};{};{};{}" 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6]
);
