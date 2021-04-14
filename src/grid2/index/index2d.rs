use super::*;

pub trait Index2D: Clone {
    fn unchecked(self, size: Size) -> Rect;

    fn checked(self, size: Size) -> Option<Rect>;
}
