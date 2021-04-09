//! Composable external 2-dimensional iteration.
//!
//! Grids enable 2D abstraction over your collections. Collections are usually
//! 1-dimensional, just like `Vec` or `[T]` are. This crate provides traits to
//! elevate your types into two dimensions in order to query items, rows and
//! columns with a familiar geometrical interface. It largely revolves around
//! iteration and integrates well with `std::iter`.
//!
//! This crate is very simple: all you can do is query items and iterate over
//! columns or rows. You wont be able to add/remove columns/rows, for instance.
//! Hence it is probably not very well suited for math/physics. Rust ecosystem
//! already provides that anyway. I wrote this crate to power a
//! layer-compositing lib for terminal rendering.
//!
//! - [Organization](#organization)
//! - [Indexes](#indexes)
//! - [Grid](#grid)
//!   - [GridCol and GridRow](#gridcol-and-gridrow)
//!   - [GridItems](#griditems)
//!   - [GridCols and GridRows](#gridcols-and-gridrows)
//! - [Implementors](#implementors-1)
//! - [Adapters](#adapters)
//!
//! # Organization
//!
//! This module provides the main traits and structs at its root. Implementors
//! and adapters are found in submodules (and their iterators in that
//! submodule's iter module), although being re-exported at the root for
//! convenience.
//!
//! # Indexes
//!
//! Let's talk about indexes to introduce how things work.
//!
//! In a 2D world, you can get an item with an x/y-pair coordinate. This crate
//! provides a [`Coord`] type with [`Point`] (and [`Size`]) aliases and `(x, y)`
//! conversion.  
//! When you need a column or a row from your grids, you could just
//! use a `usize`. Yet we go one step further: we allow  `(usize, Range<usize>)`
//! as well in order to only get a section of that column or row.  
//! You want to get a rectangular view of that grid? Here is [`Rect`]:
//! `Size<Range<usize>>`, an x/y-pair of `Range`s.
//!
//! We use the [`Index0D`], [`Index1D`] and [`Index2D`] traits to convert to
//! theses index types and check against grid sizes. We try our best at
//! ergonomics: any [`Coord`] can be a `(x, y)`, any `Range` can be a
//! `Range`-like (aka implementing [`std::ops::RangeBounds`]).
//!
//! Some examples:
//!
//! ```
//! fn indexes_examples<T>(grid: &T)
//! where
//!     &T: GridCols, // See below
//! {
//!     let item = grid.item(Point { x: 1, y: 1 });
//!     let item = grid.item((1, 1)); // same as above
//!
//!     let col = grid.col(1);
//!     let col = grid.col((1, ..)); // same as above
//!     let col = grid.col((1, ..5));
//!     let col = grid.col((1, ..=5));
//!     let col = grid.col((1, 1..=5)); // etc...
//!
//!     let cols = grid.cropped_cols(Size { x: 0..10, y: 2..5 });
//!     let cols = grid.cropped_cols((..10, 2..=4)); // same as above, etc...
//!
//!     let cols = grid.cropped_cols((.., ..));
//!     let cols = grid.cols(); // same as above, no checks
//! }
//! ```
//!
//! # Grid
//!
//! The heart and soul of this module is the [`Grid`] trait. The core of
//! [`Grid`] looks like this:
//!
//! ```
//! trait Grid: WithSize {
//!     type Item;
//!     fn item(self, index: impl Index0D) -> Option<Self::Item>;
//! }
//! ```
//!
//! A grid has a [`size()`](WithSize::size) just like `Vec` has a `len()`, and
//! an [`Item`](Grid::Item) type. You can retrieve thoses [`Item`](Grid::Item)s
//! with [`item()`](Grid::item) through an [`Index0D`] (i.e. a [`Point`]).
//!
//! [`Grid`]'s full definition includes a number of other methods to construct
//! adapters for your base grids. It also resembles `IntoIterator` in that its
//! methods take `self`s, so you will see [`Grid`] (and its subtraits)
//! implemented for `T`, `&T`, `&mut T`.
//!
//! ## GridCol and GridRow
//!
//! A type implementing [`Grid`] can also implement [`GridCol`] and [`GridRow`]
//! to query [`col()`](GridCol::col)s and [`row()`](GridRow::row)s through an
//! [`Index1D`] (e.g. a `usize`). Let's look at what [`GridCol`] looks
//! like:
//!
//! ```
//! trait GridCol: Grid {
//!     type Col: IntoIterator<Item = Self::Item>;
//!     fn col(self, index: impl Index1D) -> Option<Self::Col>;
//! }
//! ```
//!
//! Theses traits ensure the types [`Col`](GridCol::Col) and
//! [`Row`](GridRow::Row) are iterable over [`Item`](Grid::Item)s. Notice that
//! [`Index1D`] can also be both an `usize` and a `Range` to further crop the
//! returned column/row.
//!
//! ## GridItems
//!
//! The [`GridItems`] trait lets you get [`items()`](GridItems::items):
//!
//! ```
//! trait GridItems: Grid {
//!     type Items: IntoIterator<Item = Self::Item>;
//!     fn cropped_items(self, index: impl Index2D) -> Option<Self::Items>;
//!     fn items(self) -> Self::Items;
//! }
//! ```
//!
//! We still get a 1D iterable over [`Item`](Grid::Item)s, however this time we
//! query with an [`Index2D`] (e.g. a [`Rect`]). We distinguish between
//! [`cropped_items()`](GridItems::cropped_items) and
//! [`items()`](GridItems::items) since the former requires checking the index.
//!
//! Note that there are no guaranties on the order in which the items are
//! returned. It could be column-major or row-major (or anything custom,
//! really).
//!
//! ## GridCols and GridRows
//!
//! Finally we have [`GridCols`] and [`GridRows`], providing
//! [`Cols`](GridCols::Cols) and [`Rows`](GridRows::Rows):
//!
//! ```
//! trait GridCols: GridCol {
//!     type Cols: IntoIterator<Item = Self::Col>;
//!     fn cropped_cols(self, index: impl Index2D) -> Option<Self::Cols>;
//!     fn cols(self) -> Self::Cols;
//! }
//! ```
//!
//! Similarly to [`GridItems`], we query with an [`Index2D`] when cropping. Yet
//! here we get iterables over [`Col`](GridCol::Col)s and
//! [`Row`](GridRow::Row)s, effectively being 2D iterables over
//! [`Item`](Grid::Item)s.
//!
//! # Implementors
//!
//! TODO doc
//!
//! # Adapters
//!
//! TODO doc

// TODO: tests, docs, examples
// #![warn(missing_docs)]
// - Implementors:
//   - [`Slice2D`]
// - Adapters:
//   - [`Cloned`], [`Copied`]
//   - [`Crop`]
//   - [`Repeat`], [`RepeatWith`]
//   - [`Zip`]

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
