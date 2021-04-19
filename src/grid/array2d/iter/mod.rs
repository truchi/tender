//! Iterators for [`Array2D`]'s `Grid*` implementations.

mod majors;
mod majors_mut;
mod minor;
mod minor_mut;

use super::*;
pub use majors::*;
pub use majors_mut::*;
pub use minor::*;
pub use minor_mut::*;
