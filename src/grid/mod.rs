//! Composable external 2-dimensional iteration.
//!
//! Abstractions over 2D collections iteration.
//!
//! # Organization
//!
//! This module provides the main traits and structs at its root. Implementors
//! and adapters are found in submodules (and their iterators in that
//! submodule's iter module):
//! - Implementors:
//!   - [`Slice2D`]
//! - Adapters:
//!   - [`Cloned`], [`Copied`]
//!   - [`Crop`]
//!   - [`Repeat`], [`RepeatWith`]
//!   - [`Zip`]

// TODO: tests, docs, examples
// #![warn(missing_docs)]

pub mod cloned;
pub mod crop;
pub mod repeat;
pub mod slice2d;
pub mod zip;

pub use cloned::{Cloned, Copied};
pub use crop::Crop;
pub use repeat::{repeat, repeat_with, Repeat, RepeatWith};
pub use slice2d::{ColSlice2D, ColVec2D, RowSlice2D, RowVec2D, Slice2D, Vec2D};
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
