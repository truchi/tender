//! Iterators for [`Array2D`]'s `Grid*` implementations.

mod majors;
mod majors_mut;
mod minor;

use super::*;
pub use majors::*;
pub use majors_mut::*;
pub use minor::*;
