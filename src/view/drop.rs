// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    #[derive(Clone)]
    pub struct DropView<Range>
    where
        Range: InputRange + View,
    {
        pub range: Range,
        pub n: usize,
    }

    impl<R> InputRange for DropView<R>
    where
        R: InputRange + View,
    {
        type Element = R::Element;

        type Position = R::Position;

        type ElementRef<'a>
            = R::ElementRef<'a>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            let mut n = self.n;
            let mut cur = self.range.start();
            while n > 0 && !self.range.is_end(&cur) {
                cur = self.range.after(cur);
                n -= 1;
            }
            cur
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
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

    impl<R> View for DropView<R> where R: InputRange + View {}

    impl<R> BoundedRange for DropView<R>
    where
        R: BoundedRange + View,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<R> ForwardRange for DropView<R>
    where
        R: ForwardRange + View,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R> BidirectionalRange for DropView<R>
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

    impl<R> RandomAccessRange for DropView<R> where R: RandomAccessRange + View {}

    impl<R> SemiOutputRange for DropView<R>
    where
        R: SemiOutputRange + View,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j)
        }
    }

    impl<R> OutputRange for DropView<R>
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

/// Returns a view with dropping first n elements of given view.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view with dropping first n elements of given view.
///   - n > number of elements in view, then drop all elements of given view.
///   - Resulting view's position can be use to access elements in given view also.
///   - Resulting view satisfies following traits:
///     - InputRange: YES.
///     - BoundedRange: If given view is BoundedRange.
///     - ForwardRange: If given view is ForwardRange.
///     - BidirectionalRange: If given view is BidirectionalRange.
///     - RandomAccessRange: If given view is RandomAccessRange.
///     - SemiOutputRange: If given view is SemiOutputRange.
///     - OutputRange: If given view is OutputRange.
///
/// # Infix Supported
/// YES
///
/// # Examples
/// ```rust
/// use stl::*;
/// use rng::infix::*;
/// use view::infix::*;
///
/// let mut arr = [3, 1, 2, 7, 4];
/// view::drop(arr.view_mut(), 2).sort_range();
/// assert_eq!(arr, [3, 1, 2, 4, 7]);
///
/// let mut arr = [3, 1, 2, 7, 4];
/// arr.view_mut().drop(2).sort_range();
/// assert_eq!(arr, [3, 1, 2, 4, 7]);
/// ```
pub fn drop<RangeView>(
    view: RangeView,
    n: usize,
) -> __details::DropView<RangeView>
where
    RangeView: InputRange + View,
{
    __details::DropView { range: view, n }
}

pub mod infix {
    use crate::{InputRange, View};

    /// `drop`.
    pub trait STLDropExt: InputRange + View + Sized {
        fn drop(self, n: usize) -> super::__details::DropView<Self> {
            super::drop(self, n)
        }
    }

    impl<R> STLDropExt for R where R: InputRange + View + Sized {}
}
