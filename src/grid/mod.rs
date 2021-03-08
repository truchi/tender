// TODO: tests, docs, examples
// #![warn(missing_docs)]

pub mod grid1d;
pub mod repeat;
pub mod repeat_with;

pub use grid1d::{ColGrid1D, Grid1D, RowGrid1D};
pub use repeat::{repeat, Repeat};
pub use repeat_with::{repeat_with, RepeatWith};

mod cloned;
mod crop;
mod grid;
mod index;
mod major;
mod utils;
mod with_size;
mod zip;

pub use self::grid::*;
pub use cloned::*;
pub use crop::*;
pub use index::*;
pub use major::*;
pub use with_size::*;
pub use zip::*;

use crate::geometry::*;
use utils::*;
