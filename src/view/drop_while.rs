// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct DropWhileView<Range, Pred>
    where
        Range: InputRange + View,
        Pred: Fn(&Range::Element) -> bool,
    {
        pub range: Range,
        pub pred: Pred,
    }

    impl<R, F> InputRange for DropWhileView<R, F>
    where
        R: InputRange + View,
        F: Fn(&R::Element) -> bool,
    {
        type Element = R::Element;

        type Position = R::Position;

        type ElementRef<'a>
            = R::ElementRef<'a>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            let mut cur = self.range.start();
            while !self.range.is_end(&cur) && (self.pred)(&self.range.at(&cur))
            {
                cur = self.range.after(cur);
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

    impl<R, F> View for DropWhileView<R, F>
    where
        R: InputRange + View,
        F: Fn(&R::Element) -> bool,
    {
    }

    impl<R, F> BoundedRange for DropWhileView<R, F>
    where
        R: BoundedRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<R, F> ForwardRange for DropWhileView<R, F>
    where
        R: ForwardRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R, F> BidirectionalRange for DropWhileView<R, F>
    where
        R: BidirectionalRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            self.range.before(i)
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.before_n(i, n)
        }
    }

    impl<R, F> RandomAccessRange for DropWhileView<R, F>
    where
        R: RandomAccessRange + View,
        F: Fn(&R::Element) -> bool,
    {
    }

    impl<R, F> SemiOutputRange for DropWhileView<R, F>
    where
        R: SemiOutputRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j);
        }
    }

    impl<R, F> OutputRange for DropWhileView<R, F>
    where
        R: OutputRange + View,
        F: Fn(&R::Element) -> bool,
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

/// Returns suffix view of given range such that, view starts from the first position p such that p
/// doesn't satisfy pred.
///
/// # Precondition
///
/// # Postcondition
///   - Returns suffix view of given range such that, view starts from the first position p such that p
///     doesn't satisfy pred.
///   - Follows following traits:
///     - InputRange: YES
///     - BoundedRange: If given Range is BoundedRange
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
/// let mut arr = [1, 3, 5, 4, 2, 6];
/// let mut v = view::drop_while(arr.view_mut(), |x| *x % 2 == 1);
/// v.sort_range();
/// assert!(v.equals(&[2, 4, 6]));
/// assert_eq!(arr, [1, 3, 5, 2, 4, 6]);
///
/// let mut arr = [1, 3, 5, 4, 2, 6];
/// let mut v = arr.view_mut().drop_while(|x| *x % 2 == 1);
/// v.sort_range();
/// assert!(v.equals(&[2, 4, 6]));
/// assert_eq!(arr, [1, 3, 5, 2, 4, 6]);
/// ```
pub fn drop_while<RangeView, Pred>(
    view: RangeView,
    pred: Pred,
) -> __details::DropWhileView<RangeView, Pred>
where
    RangeView: InputRange + View,
    Pred: Fn(&RangeView::Element) -> bool,
{
    __details::DropWhileView { range: view, pred }
}

pub mod infix {
    use super::__details;
    use crate::{InputRange, View};

    /// `drop_while`.
    pub trait STLDropWhileViewExt: InputRange + View + Sized {
        fn drop_while<Pred>(
            self,
            pred: Pred,
        ) -> __details::DropWhileView<Self, Pred>
        where
            Pred: Fn(&Self::Element) -> bool,
        {
            super::drop_while(self, pred)
        }
    }
    impl<R> STLDropWhileViewExt for R where R: InputRange + View + Sized {}
}
