pub use Weight::*;

/// [`Weight`](crate::Weight) (`Bold`, `Light`, `NoWeight`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Weight {
    Bold,
    Light,
    NoWeight,
}

impl Default for Weight {
    fn default() -> Self {
        NoWeight
    }
}
