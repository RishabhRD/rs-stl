// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct PrefixView<Range>
    where
        Range: ForwardRange + View,
    {
        pub range: Range,
        pub end: Range::Position,
    }

    impl<R> InputRange for PrefixView<R>
    where
        R: ForwardRange + View,
    {
        type Element = R::Element;

        type Position = R::Position;

        type ElementRef<'a>
            = R::ElementRef<'a>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.end == *i
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            self.range.at(i)
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

    impl<R> View for PrefixView<R> where R: ForwardRange + View {}

    impl<R> BoundedRange for PrefixView<R>
    where
        R: ForwardRange + View,
    {
        fn end(&self) -> Self::Position {
            self.end.clone()
        }
    }

    impl<R> ForwardRange for PrefixView<R>
    where
        R: ForwardRange + View,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R> BidirectionalRange for PrefixView<R>
    where
        R: BidirectionalRange + View,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<R> RandomAccessRange for PrefixView<R> where R: RandomAccessRange + View {}

    impl<R> SemiOutputRange for PrefixView<R>
    where
        R: SemiOutputRange + View,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j);
        }
    }

    impl<R> OutputRange for PrefixView<R>
    where
        R: OutputRange + View,
    {
        type ElementMutRef<'a>
            = R::ElementMutRef<'a>
        where
            Self: 'a;

        fn at_mut<'a>(
            &'a mut self,
            i: &Self::Position,
        ) -> Self::ElementMutRef<'a> {
            self.range.at_mut(i)
        }
    }
}

/// Returns prefix of given view, i.e., `[view.start(), end)`.
///
/// # Precondition
///   - end is a valid position in view.
///
/// # Postcondition
///   - Returns a view that is prefix `[view.start(), end)` of given view.
///   - Returned view is:
///     - InputRange: YES
///     - ForwardRange: YES
///     - BoundedRange: YES
///     - BidirectionalRange: If given view is BidirectionalRange
///     - RandomAccessRange: If given view is RandomAccessRange
///     - SemiOutputRange: If given view is SemiOutputRange
///     - OutputRange: If given view is OutputRange
///   - Position type is same as given range Position type
///   - Element type is same as given range Element type
///   - ElementRef type is same as given range ElementRef type
///   - ElementMutRef type is same as given range ElementMutRef type
///
/// # Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use view::infix::*;
/// use rng::infix::*;
///
/// let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
/// view::prefix(arr.view_mut(), 4).stable_sort_by(|x, y| x.0 < y.0);
/// assert_eq!(arr, [(1, 2), (1, 1), (3, 1), (4, 4), (5, 5)]);
///
/// let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
/// arr.view_mut().prefix(4).stable_sort_by(|x, y| x.0 < y.0);
/// assert_eq!(arr, [(1, 2), (1, 1), (3, 1), (4, 4), (5, 5)]);
/// ```
pub fn prefix<RangeView>(
    view: RangeView,
    end: RangeView::Position,
) -> __details::PrefixView<RangeView>
where
    RangeView: ForwardRange + View,
{
    __details::PrefixView { range: view, end }
}

pub mod infix {
    use super::__details;
    use crate::{ForwardRange, View};

    /// Provides `.prefix(...)` method.
    pub trait STLPrefixExt: ForwardRange + View + Sized {
        fn prefix(self, end: Self::Position) -> __details::PrefixView<Self> {
            super::prefix(self, end)
        }
    }
    impl<R> STLPrefixExt for R where R: ForwardRange + View {}
}
