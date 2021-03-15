// TODO: tests, docs, examples
// #![warn(missing_docs)]

pub mod cloned;
pub mod crop;
pub mod grid1d;
pub mod repeat;
pub mod repeat_with;
pub mod zip;

pub use cloned::{Cloned, Copied};
pub use crop::Crop;
pub use grid1d::{ColGrid1D, Grid1D, RowGrid1D};
pub use repeat::{repeat, Repeat};
pub use repeat_with::{repeat_with, RepeatWith};
pub use zip::Zip;

mod grid;
mod index;
mod major;
mod utils;
mod with_size;

pub use self::grid::*;
pub use index::*;
pub use major::*;
pub use with_size::*;

use crate::geometry::*;
use utils::*;
