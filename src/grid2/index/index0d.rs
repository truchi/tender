use super::*;

pub trait Index0D: Clone {
    fn unchecked(self) -> Point;
    fn checked(self, size: Size) -> Option<Point>;
}
