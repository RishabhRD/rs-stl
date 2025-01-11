// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    #[derive(Clone)]
    pub struct SubRangeView<Range: ForwardRange + View> {
        pub range: Range,
        pub start: Range::Position,
        pub end: Range::Position,
    }

    impl<R> InputRange for SubRangeView<R>
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
            self.start.clone()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            *i == self.end
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

    impl<R> View for SubRangeView<R> where R: ForwardRange + View {}

    impl<R> BoundedRange for SubRangeView<R>
    where
        R: ForwardRange + View,
    {
        fn end(&self) -> Self::Position {
            self.end.clone()
        }
    }

    impl<R> ForwardRange for SubRangeView<R>
    where
        R: ForwardRange + View,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R> BidirectionalRange for SubRangeView<R>
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

    impl<R> RandomAccessRange for SubRangeView<R> where R: RandomAccessRange + View {}

    impl<R> SemiOutputRange for SubRangeView<R>
    where
        R: SemiOutputRange + View,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j)
        }
    }

    impl<R> OutputRange for SubRangeView<R>
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

/// Returns a subrange of given view from `[start, end)`.
///
/// # Precondition
///   - `[start, end)` represents valid position in view.
///
/// # Postcondition
///   - Returns a view that is subrange `[start, end)` of given view.
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
/// view::subrange(arr.view_mut(), 1, 4).stable_sort_by(|x, y| x.0 < y.0);
/// assert_eq!(arr, [(3, 1), (1, 2), (1, 1), (4, 4), (5, 5)]);
///
/// let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
/// arr.view_mut().subrange(1, 4).stable_sort_by(|x, y| x.0 < y.0);
/// assert_eq!(arr, [(3, 1), (1, 2), (1, 1), (4, 4), (5, 5)]);
/// ```
pub fn subrange<RangeView>(
    view: RangeView,
    start: RangeView::Position,
    end: RangeView::Position,
) -> __details::SubRangeView<RangeView>
where
    RangeView: ForwardRange + View,
{
    __details::SubRangeView {
        range: view,
        start,
        end,
    }
}

pub mod infix {
    use crate::{ForwardRange, View};

    use super::__details;

    /// `subrange`.
    pub trait STLSubRangeExt: ForwardRange + View + Sized {
        fn subrange(
            self,
            start: Self::Position,
            end: Self::Position,
        ) -> __details::SubRangeView<Self> {
            super::subrange(self, start, end)
        }
    }

    impl<R> STLSubRangeExt for R where R: ForwardRange + View {}
}
