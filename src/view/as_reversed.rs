// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{BidirectionalRange, BoundedRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    #[derive(Clone)]
    pub struct ReverseView<Range>
    where
        Range: BidirectionalRange + BoundedRange + View,
    {
        pub range: Range,
    }

    impl<R> InputRange for ReverseView<R>
    where
        R: BidirectionalRange + BoundedRange + View,
    {
        type Element = R::Element;

        type Position = R::Position;

        type ElementRef<'a>
            = R::ElementRef<'a>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            self.range.end()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            *i == self.range.start()
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            self.range.at(&self.range.before(i.clone()))
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<R> View for ReverseView<R> where R: BidirectionalRange + BoundedRange + View
    {}

    impl<R> BoundedRange for ReverseView<R>
    where
        R: BidirectionalRange + BoundedRange + View,
    {
        fn end(&self) -> Self::Position {
            self.range.start()
        }
    }

    impl<R> ForwardRange for ReverseView<R>
    where
        R: BidirectionalRange + BoundedRange + View,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(to, from)
        }
    }

    impl<R> BidirectionalRange for ReverseView<R>
    where
        R: BidirectionalRange + BoundedRange + View,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

    impl<R> RandomAccessRange for ReverseView<R> where
        R: RandomAccessRange + BoundedRange + View
    {
    }

    impl<R> SemiOutputRange for ReverseView<R>
    where
        R: BidirectionalRange + SemiOutputRange + BoundedRange + View,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            let new_i = self.range.before(i.clone());
            let new_j = self.range.before(j.clone());
            self.range.swap_at(&new_i, &new_j)
        }
    }

    impl<R> OutputRange for ReverseView<R>
    where
        R: BidirectionalRange + OutputRange + BoundedRange + View,
    {
        type ElementMutRef<'a>
            = R::ElementMutRef<'a>
        where
            Self: 'a;

        fn at_mut<'a>(
            &'a mut self,
            i: &Self::Position,
        ) -> Self::ElementMutRef<'a> {
            self.range.at_mut(&self.range.before(i.clone()))
        }
    }
}

/// Returns reverse view of given view.
///
/// # Precondition
///
/// # Postcondition
///   - Returns reverse view of given view.
///   - Resulting view satisfies following traits:
///     - InputRange: YES
///     - BoundedRange: YES
///     - ForwardRange: YES
///     - BidirectionalRange: YES
///     - RandomAccessRange: If given view follows RandomAccessRange
///     - SemiOutputRange: If given view follows SemiOutputRange
///     - OutputRange: If given view follows OutputRange
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
/// let mut arr = [2, 1, 3];
/// view::as_reversed(arr.view_mut()).sort_range();
/// assert_eq!(arr, [3, 2, 1]);
///
/// let mut arr = [2, 1, 3];
/// arr.view_mut().as_reversed().sort_range();
/// assert_eq!(arr, [3, 2, 1]);
/// ```
pub fn as_reversed<RangeView>(
    view: RangeView,
) -> __details::ReverseView<RangeView>
where
    RangeView: BidirectionalRange + BoundedRange + View,
{
    __details::ReverseView { range: view }
}

pub mod infix {
    use crate::{BidirectionalRange, BoundedRange, View};

    /// `as_reversed`.
    pub trait STLAsReversedExt:
        BidirectionalRange + BoundedRange + View + Sized
    {
        #[allow(clippy::wrong_self_convention)]
        fn as_reversed(self) -> super::__details::ReverseView<Self> {
            super::as_reversed(self)
        }
    }

    impl<R> STLAsReversedExt for R where
        R: BidirectionalRange + BoundedRange + View + Sized
    {
    }
}
