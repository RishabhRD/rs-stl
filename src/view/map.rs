// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        util::ValueRef, BidirectionalRange, BoundedRange, ForwardRange,
        InputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct MapView<Range, F, OutputElement>
    where
        Range: InputRange + View,
        F: Fn(&Range::Element) -> OutputElement,
    {
        pub range: Range,
        pub op: F,
    }

    impl<R, F, O> InputRange for MapView<R, F, O>
    where
        R: InputRange + View,
        F: Fn(&R::Element) -> O,
    {
        type Element = O;

        type Position = R::Position;

        type ElementRef<'a>
            = ValueRef<O>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            ValueRef {
                val: (self.op)(&self.range.at(i)),
            }
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

    impl<R, F, O> View for MapView<R, F, O>
    where
        R: InputRange + View,
        F: Fn(&R::Element) -> O,
    {
    }

    impl<R, F, O> BoundedRange for MapView<R, F, O>
    where
        R: BoundedRange + View,
        F: Fn(&R::Element) -> O,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<R, F, O> ForwardRange for MapView<R, F, O>
    where
        R: ForwardRange + View,
        F: Fn(&R::Element) -> O,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R, F, O> BidirectionalRange for MapView<R, F, O>
    where
        R: BidirectionalRange + View,
        F: Fn(&R::Element) -> O,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<R, F, O> RandomAccessRange for MapView<R, F, O>
    where
        R: RandomAccessRange + View,
        F: Fn(&R::Element) -> O,
    {
    }

    impl<R, F, O> SemiOutputRange for MapView<R, F, O>
    where
        R: SemiOutputRange + View,
        F: Fn(&R::Element) -> O,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j);
        }
    }
}

/// Creates a view from given view where each element is op(&e).
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view from given view where each element is op(&e).
///   - Retrned view satisifes following traits:
///     - InputRange: YES
///     - BoundedRange: If given view is BoundedRange
///     - ForwardRange: If given view is ForwardRange
///     - BidirectionalRange: If given view is BidirectionalRange
///     - RandomAccessRange: If given view is RandomAccessRange
///     - SemiOutputRange: If given view is SemiOutputRange
///     - OutputRange: NO
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
/// view::map(arr.view_mut(), |x| x.0).sort_range();
/// assert!(arr.is_sorted_by(|x, y| x.0 < y.0));
///
/// let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
/// arr.view_mut().map(|x| x.0).sort_range();
/// assert!(arr.is_sorted_by(|x, y| x.0 < y.0));
/// ```
pub fn map<RangeView, UnaryOp, OutputElement>(
    view: RangeView,
    op: UnaryOp,
) -> __details::MapView<RangeView, UnaryOp, OutputElement>
where
    RangeView: InputRange + View,
    UnaryOp: Fn(&RangeView::Element) -> OutputElement,
{
    __details::MapView { range: view, op }
}

pub mod infix {
    use crate::{InputRange, View};

    use super::__details;

    /// `map`.
    pub trait STLMapExt: InputRange + View + Sized {
        fn map<UnaryOp, OutputElement>(
            self,
            op: UnaryOp,
        ) -> __details::MapView<Self, UnaryOp, OutputElement>
        where
            UnaryOp: Fn(&Self::Element) -> OutputElement,
        {
            super::map(self, op)
        }
    }

    impl<R> STLMapExt for R where R: InputRange + View {}
}
