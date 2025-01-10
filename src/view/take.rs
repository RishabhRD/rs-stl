// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct TakeView<Range>
    where
        Range: InputRange + View,
    {
        pub range: Range,
        pub size: usize,
    }

    impl<R> InputRange for TakeView<R>
    where
        R: InputRange + View,
    {
        type Element = R::Element;

        type Position = (R::Position, usize);

        type ElementRef<'a>
            = R::ElementRef<'a>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            (self.range.start(), 0)
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            i.1 == self.size || self.range.is_end(&i.0)
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            (self.range.after(i.0), i.1 + 1)
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            self.range.at(&i.0)
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            (self.range.after_n(i.0, n), i.1 + n)
        }
    }

    impl<R> View for TakeView<R> where R: InputRange + View {}

    impl<R> BoundedRange for TakeView<R>
    where
        R: RandomAccessRange + BoundedRange + View,
    {
        fn end(&self) -> Self::Position {
            let dist =
                self.range.distance(self.range.start(), self.range.end());
            let num_ele = if dist > self.size { self.size } else { dist };
            (self.range.after_n(self.range.start(), num_ele), num_ele)
        }
    }

    impl<R> ForwardRange for TakeView<R>
    where
        R: ForwardRange + View,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            to.1 - from.1
        }
    }

    impl<R> BidirectionalRange for TakeView<R>
    where
        R: BidirectionalRange + View,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            (self.range.before(i.0), i.1 - 1)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            (self.range.before_n(i.0, n), i.1 - n)
        }
    }

    impl<R> RandomAccessRange for TakeView<R> where R: RandomAccessRange + View {}

    impl<R> SemiOutputRange for TakeView<R>
    where
        R: SemiOutputRange + View,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(&i.0, &j.0);
        }
    }

    impl<R> OutputRange for TakeView<R>
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
            self.range.at_mut(&i.0)
        }
    }
}

/// Returns a view of first n elements of given view.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view of first n elements of given view.
///   - If n > number of elements in given view, all elements of given view
///     are taken.
///   - Follows following traits:
///     - InputRange: YES
///     - BoundedRange: If given Range is BoundedRange + RandomAccessRange
///     - ForwardRange: If given Range is ForwardRange
///     - BidirectionalRange: If given Range is BidirectionalRange
///     - RandomAccessRange: If given Range is RandomAccessRange
///     - SemiOutputRange: If given Range is SemiOutputRange
///     - OutputRange: If given Range is OutputRange
///
/// # Infix Supported
/// YES
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
/// use view::infix::*;
///
/// let mut arr = [3, 1, 2, 7, 4];
/// view::take(arr.view_mut(), 3).sort_range();
/// assert_eq!(arr, [1, 2, 3, 7, 4]);
///
/// let mut arr = [3, 1, 2, 7, 4];
/// arr.view_mut().take(3).sort_range();
/// assert_eq!(arr, [1, 2, 3, 7, 4]);
/// ```
pub fn take<RangeView: InputRange + View>(
    view: RangeView,
    n: usize,
) -> __details::TakeView<RangeView> {
    __details::TakeView {
        range: view,
        size: n,
    }
}

pub mod infix {
    use super::__details;
    use crate::{InputRange, View};

    /// `take`.
    pub trait STLTakeViewExt: InputRange + View + Sized {
        fn take(self, n: usize) -> __details::TakeView<Self> {
            super::take(self, n)
        }
    }

    impl<R> STLTakeViewExt for R where R: InputRange + View + Sized {}
}
