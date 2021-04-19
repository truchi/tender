//! Iterators for [`Grid1D`]'s `Grid*` implementations.
//!
//! You will not find here `Major`/`MajorMut` since we use regular slices for
//! that. You won't find either `MinorsMut` since there is no safe way to do
//! this.
//!
//! Refer to these types through `Grid*`'s associated types (e.g.
//! `&RowGrid1D::Cols`).

mod majors;
mod majors_mut;
mod minor;
mod minor_mut;
mod minors;

use super::*;
pub use majors::*;
pub use majors_mut::*;
pub use minor::*;
pub use minor_mut::*;
pub use minors::*;
