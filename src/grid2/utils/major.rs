pub trait Major {}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ColMajor {}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RowMajor {}

impl Major for ColMajor {}
impl Major for RowMajor {}
