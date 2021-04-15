use std::ops::{
    Bound::{self, *},
    Range,
    RangeBounds,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive,
};

/// Converts `T: RangeBounds<usize>` to `Range<usize>`.
pub trait ToRange: RangeBounds<usize> {
    /// Converts to `Range` without bounds checking.
    fn unchecked(self, len: usize) -> Range<usize>;

    /// Converts to `Range` with bounds checking.
    fn checked(self, len: usize) -> Option<Range<usize>>;

    /// Converts to `Range` by cropping.
    fn cropped(self, len: usize) -> Range<usize>;
}

macro_rules! to_range {
    ($self:ident $start:ident $end:ident $len:ident $(
        $Start:pat, $End:pat, $($Range:ty [$start_block:expr, $end_block:expr])? {
            $unchecked:block
            $checked:block
            $cropped:block
        }
    )*) => {
        impl ToRange for (Bound<usize>, Bound<usize>) {
            fn unchecked(self, $len: usize) -> Range<usize> {
                match self { $(($Start, $End) => $unchecked )* }
            }

            fn checked(self, $len: usize) -> Option<Range<usize>> {
                match self { $(($Start, $End) => $checked)* }
            }

            fn cropped(self, $len: usize) -> Range<usize> {
                match self { $(($Start, $End) => $cropped)* }
            }
        }

        $($(
            #[allow(unused_variables)]
            impl ToRange for $Range {
                fn unchecked($self, $len: usize) -> Range<usize> {
                    let $start = $start_block;
                    let $end = $end_block;
                    $unchecked
                }

                fn checked($self, $len: usize) -> Option<Range<usize>> {
                    let $start = $start_block;
                    let $end = $end_block;
                    $checked
                }

                fn cropped($self, $len: usize) -> Range<usize> {
                    let $start = $start_block;
                    let $end = $end_block;
                    $cropped
                }
            }
        )?)*
    };
}

to_range!(self start end len
    Unbounded, Unbounded, RangeFull [(), ()] {
        { 0..len }
        { Some(0..len) }
        { 0..len }
    }
    Unbounded, Included(end), RangeToInclusive<usize> [(), self.end] {
        { 0..end + 1 }
        { if end < len { Some(0..end + 1) } else { None } }
        { 0..(if end < len { end + 1 } else { len }) }
    }
    Unbounded, Excluded(end), RangeTo<usize> [(), self.end] {
        { 0..end }
        { if end <= len { Some(0..end) } else { None } }
        { 0..end.min(len) }
    }
    Included(start), Unbounded, RangeFrom<usize> [self.start, ()] {
        { start..len }
        { if start <= len { Some(start..len) } else { None } }
        { start.min(len)..len }
    }
    Included(start), Included(end), RangeInclusive<usize> [*self.start(), *self.end()] {
        { start..end + 1 }
        { if end < len && start <= end + 1 { Some(start..end + 1) } else { None } }
        { let end = if end < len { end + 1 } else { len }; start.min(end)..end }
    }
    Included(start), Excluded(end), Range<usize> [self.start, self.end] {
        { start..end }
        { if start <= end && end <= len { Some(start..end) } else { None } }
        { let end = end.min(len); start.min(end)..end }
    }
    Excluded(start), Unbounded, {
        { start + 1..len }
        { if start < len { Some(start + 1..len) } else { None } }
        { (if start < len { start + 1 } else { len })..len }
    }
    Excluded(start), Included(end), {
        { start + 1..end + 1 }
        { if start <= end && end < len { Some(start + 1..end + 1) } else { None }}
        {
            let end = if end < len { end + 1 } else { len };
            let start = if start < end { start + 1 } else { end };
            start..end
        }
    }
    Excluded(start), Excluded(end), {
        { start + 1..end }
        { if start < end && end <= len { Some(start + 1..end) } else { None }}
        { let end = end.min(len); (if start < end { start + 1 } else { end })..end }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::{fmt::Debug, ops::RangeBounds};

    const MAX: usize = usize::MAX;

    fn copied(bound: Bound<&usize>) -> Bound<usize> {
        match bound {
            Included(bound) => Included(*bound),
            Excluded(bound) => Excluded(*bound),
            Unbounded => Unbounded,
        }
    }

    fn is_valid(range: Range<usize>, len: usize) {
        assert!(range.start <= range.end, "({:?}): start > end", range);
        assert!(range.end <= len, "({:?}): end > len ({})", range, len);
    }

    fn test_bounds<T: RangeBounds<usize> + ToRange + Debug + Clone>(
        range: T,
        len: usize,
        unchecked: Range<usize>,
        checked: Option<Range<usize>>,
        cropped: Range<usize>,
    ) {
        let unchecked_ = range.clone().unchecked(len);
        let checked_ = range.clone().checked(len);
        let cropped_ = range.clone().cropped(len);

        assert_eq!(
            unchecked_, unchecked,
            "({:?}).unchecked({}) == {:?} != {:?}",
            range, len, unchecked_, unchecked
        );
        assert_eq!(
            checked_, checked,
            "({:?}).checked({}) == {:?} != {:?}",
            range, len, checked_, checked
        );
        assert_eq!(
            cropped_, cropped,
            "({:?}).cropped({}) == {:?} != {:?}",
            range, len, cropped_, cropped
        );

        checked_.map(|checked_| is_valid(checked_, len));
        is_valid(cropped_, len);
    }

    fn test_range<T: RangeBounds<usize> + ToRange + Debug + Clone>(
        range: T,
        len: usize,
        unchecked: Range<usize>,
        checked: Option<Range<usize>>,
        cropped: Range<usize>,
    ) {
        test_bounds(
            range.clone(),
            len,
            unchecked.clone(),
            checked.clone(),
            cropped.clone(),
        );
        let range = (
            copied(range.clone().start_bound()),
            copied(range.clone().end_bound()),
        );
        test_bounds(range, len, unchecked, checked, cropped);
    }

    #[test]
    fn range_full() {
        test_range(.., 5, 0..5, Some(0..5), 0..5);
        test_range(.., MAX, 0..MAX, Some(0..MAX), 0..MAX);
    }

    #[test]
    fn range_to_inclusive() {
        test_range(..=3, 5, 0..4, Some(0..4), 0..4);
        test_range(..=4, 5, 0..5, Some(0..5), 0..5);
        test_range(..=5, 5, 0..6, None, 0..5);
        test_range(..=MAX - 1, 5, 0..MAX, None, 0..5);

        assert_eq!((..=MAX).checked(5), None);
        assert_eq!((..=MAX).cropped(5), 0..5);
        assert_eq!((..=MAX).checked(MAX), None);
        assert_eq!((..=MAX).cropped(MAX), 0..MAX);
    }

    #[test]
    fn range_to() {
        test_range(..3, 5, 0..3, Some(0..3), 0..3);
        test_range(..5, 5, 0..5, Some(0..5), 0..5);
        test_range(..7, 5, 0..7, None, 0..5);
        test_range(..MAX, 5, 0..MAX, None, 0..5);
        test_range(..MAX, MAX, 0..MAX, Some(0..MAX), 0..MAX);
    }

    #[test]
    fn range_from() {
        test_range(0.., 5, 0..5, Some(0..5), 0..5);
        test_range(1.., 5, 1..5, Some(1..5), 1..5);
        test_range(7.., 5, 7..5, None, 5..5);
        test_range(MAX.., 5, MAX..5, None, 5..5);
        test_range(MAX.., MAX, MAX..MAX, Some(MAX..MAX), MAX..MAX);
    }

    #[test]
    fn range_inclusive() {
        test_range(0..=3, 5, 0..4, Some(0..4), 0..4);
        test_range(1..=4, 5, 1..5, Some(1..5), 1..5);
        test_range(1..=5, 5, 1..6, None, 1..5);
        test_range(2..=7, 5, 2..8, None, 2..5);
        test_range(9..=7, 5, 9..8, None, 5..5);
        test_range(3..=2, 5, 3..3, Some(3..3), 3..3);
        test_range(5..=4, 5, 5..5, Some(5..5), 5..5);

        assert_eq!((3..=MAX).checked(5), None);
        assert_eq!((3..=MAX).cropped(5), 3..5);
        assert_eq!((3..=MAX).checked(MAX), None);
        assert_eq!((3..=MAX).cropped(MAX), 3..MAX);
    }

    #[test]
    fn range() {
        test_range(0..3, 5, 0..3, Some(0..3), 0..3);
        test_range(1..5, 5, 1..5, Some(1..5), 1..5);
        test_range(2..7, 5, 2..7, None, 2..5);
        test_range(9..7, 5, 9..7, None, 5..5);
        test_range(3..2, 5, 3..2, None, 2..2);
        test_range(MAX..MAX, 5, MAX..MAX, None, 5..5);
        test_range(MAX..MAX, MAX, MAX..MAX, Some(MAX..MAX), MAX..MAX);
    }

    #[test]
    fn excluded_unbounded() {
        test_range((Excluded(0), Unbounded), 5, 1..5, Some(1..5), 1..5);
        test_range((Excluded(4), Unbounded), 5, 5..5, Some(5..5), 5..5);
        test_range((Excluded(5), Unbounded), 5, 6..5, None, 5..5);
        test_range((Excluded(6), Unbounded), 5, 7..5, None, 5..5);

        assert_eq!((Excluded(MAX), Unbounded).checked(5), None);
        assert_eq!((Excluded(MAX), Unbounded).cropped(5), 5..5);
        assert_eq!((Excluded(MAX), Unbounded).checked(MAX), None);
        assert_eq!((Excluded(MAX), Unbounded).cropped(MAX), MAX..MAX);
    }

    #[test]
    fn excluded_included() {
        test_range((Excluded(0), Included(3)), 5, 1..4, Some(1..4), 1..4);
        test_range((Excluded(0), Included(4)), 5, 1..5, Some(1..5), 1..5);
        test_range((Excluded(0), Included(5)), 5, 1..6, None, 1..5);
        test_range((Excluded(3), Included(3)), 5, 4..4, Some(4..4), 4..4);
        test_range((Excluded(4), Included(4)), 5, 5..5, Some(5..5), 5..5);
        test_range((Excluded(5), Included(5)), 5, 6..6, None, 5..5);

        assert_eq!((Excluded(5), Included(MAX)).checked(5), None);
        assert_eq!((Excluded(5), Included(MAX)).cropped(5), 5..5);
        assert_eq!((Excluded(MAX), Included(5)).checked(5), None);
        assert_eq!((Excluded(MAX), Included(5)).cropped(5), 5..5);
        assert_eq!((Excluded(MAX), Included(MAX)).checked(5), None);
        assert_eq!((Excluded(MAX), Included(MAX)).cropped(5), 5..5);
        assert_eq!((Excluded(MAX), Included(MAX)).checked(MAX), None);
        assert_eq!((Excluded(MAX), Included(MAX)).cropped(MAX), MAX..MAX);
    }

    #[test]
    fn excluded_excluded() {
        test_range((Excluded(0), Excluded(4)), 5, 1..4, Some(1..4), 1..4);
        test_range((Excluded(0), Excluded(5)), 5, 1..5, Some(1..5), 1..5);
        test_range((Excluded(0), Excluded(6)), 5, 1..6, None, 1..5);
        test_range((Excluded(3), Excluded(4)), 5, 4..4, Some(4..4), 4..4);
        test_range((Excluded(4), Excluded(5)), 5, 5..5, Some(5..5), 5..5);
        test_range((Excluded(5), Excluded(6)), 5, 6..6, None, 5..5);

        assert_eq!((Excluded(5), Excluded(MAX)).checked(5), None);
        assert_eq!((Excluded(5), Excluded(MAX)).cropped(5), 5..5);
        assert_eq!((Excluded(MAX), Excluded(5)).checked(5), None);
        assert_eq!((Excluded(MAX), Excluded(5)).cropped(5), 5..5);
        assert_eq!((Excluded(MAX), Excluded(MAX)).checked(5), None);
        assert_eq!((Excluded(MAX), Excluded(MAX)).cropped(5), 5..5);
        assert_eq!((Excluded(MAX), Excluded(MAX)).checked(MAX), None);
        assert_eq!((Excluded(MAX), Excluded(MAX)).cropped(MAX), MAX..MAX);
    }

    #[test]
    fn valid() {
        let bounds = [
            0,
            1,
            2,
            71,
            72,
            73,
            usize::MAX - 2,
            usize::MAX - 1,
            usize::MAX,
        ];

        for bound in &bounds {
            for start in &[Unbounded, Included(*bound), Excluded(*bound)] {
                for bound in &bounds {
                    for end in &[Unbounded, Included(*bound), Excluded(*bound)] {
                        for len in &bounds {
                            (*start, *end)
                                .checked(*len)
                                .map(|checked| is_valid(checked, *len));
                            is_valid((*start, *end).cropped(*len), *len);
                        }
                    }
                }
            }
        }
    }
}
