// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        BidirectionalRange, ForwardRange, InputRange, OutputRange,
        SemiOutputRange, View,
    };

    pub struct TakeWhileView<Range, Pred>
    where
        Range: InputRange + View,
        Pred: Fn(&Range::Element) -> bool,
    {
        pub range: Range,
        pub pred: Pred,
    }

    impl<R, F> InputRange for TakeWhileView<R, F>
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
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i) || !(self.pred)(&self.range.at(i))
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

    impl<R, F> View for TakeWhileView<R, F>
    where
        R: InputRange + View,
        F: Fn(&R::Element) -> bool,
    {
    }

    impl<R, F> ForwardRange for TakeWhileView<R, F>
    where
        R: ForwardRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R, F> BidirectionalRange for TakeWhileView<R, F>
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

    impl<R, F> SemiOutputRange for TakeWhileView<R, F>
    where
        R: SemiOutputRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j);
        }
    }

    impl<R, F> OutputRange for TakeWhileView<R, F>
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

/// Returns the prefix view of given view such that prefix only contains elements that satisfies
/// the given predicate. It starts excluding elements from given range from first position, element
/// did not satisfy the predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns the prefix view of given view such that prefix only contains elements that satisfies
///     the given predicate. It starts excluding elements from given range from first position, element
///     did not satisfy the predicate.
///   - Follows following traits:
///     - InputRange: YES
///     - BoundedRange: NO
///     - ForwardRange: If given Range is ForwardRange
///     - BidirectionalRange: If given Range is BidirectionalRange
///     - RandomAccessRange: NO
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
/// let arr = [1, 3, 5, 2, 4];
/// let v = view::take_while(arr.view(), |x| *x % 2 == 1);
/// assert!(v.equals(&[1, 3, 5]));
///
/// let arr = [1, 3, 5, 2, 4];
/// let v = arr.view().take_while(|x| *x % 2 == 1);
/// assert!(v.equals(&[1, 3, 5]));
/// ```
pub fn take_while<RangeView, Pred>(
    view: RangeView,
    pred: Pred,
) -> __details::TakeWhileView<RangeView, Pred>
where
    RangeView: InputRange + View,
    Pred: Fn(&RangeView::Element) -> bool,
{
    __details::TakeWhileView { range: view, pred }
}

pub mod infix {
    use super::__details;
    use crate::{InputRange, View};

    /// `take_while`.
    pub trait STLTakeWhileViewExt: InputRange + View + Sized {
        fn take_while<Pred>(
            self,
            pred: Pred,
        ) -> __details::TakeWhileView<Self, Pred>
        where
            Pred: Fn(&Self::Element) -> bool,
        {
            super::take_while(self, pred)
        }
    }
    impl<R> STLTakeWhileViewExt for R where R: InputRange + View + Sized {}
}
