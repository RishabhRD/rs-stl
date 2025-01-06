// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange, View,
    };

    pub struct FilterView<Range, Pred>
    where
        Range: InputRange + View,
        Pred: Fn(&Range::Element) -> bool,
    {
        pub range: Range,
        pub pred: Pred,
    }

    impl<Range, Pred> FilterView<Range, Pred>
    where
        Range: InputRange + View,
        Pred: Fn(&Range::Element) -> bool,
    {
        fn satisfies_pred(&self, i: &Range::Position) -> bool {
            (self.pred)(&self.range.at(i))
        }
    }

    impl<R, F> InputRange for FilterView<R, F>
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
            let mut i = self.range.start();
            while !self.range.is_end(&i) {
                if self.satisfies_pred(&i) {
                    break;
                }
                i = self.range.after(i);
            }
            i
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
        }

        fn after(&self, mut i: Self::Position) -> Self::Position {
            i = self.range.after(i);
            while !self.range.is_end(&i) {
                if self.satisfies_pred(&i) {
                    break;
                }
                i = self.range.after(i)
            }
            i
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            self.range.at(i)
        }
    }

    impl<R, F> View for FilterView<R, F>
    where
        R: InputRange + View,
        F: Fn(&R::Element) -> bool,
    {
    }

    impl<R, F> BoundedRange for FilterView<R, F>
    where
        R: BoundedRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<R, F> ForwardRange for FilterView<R, F>
    where
        R: ForwardRange + View,
        F: Fn(&R::Element) -> bool,
    {
    }

    impl<R, F> BidirectionalRange for FilterView<R, F>
    where
        R: BidirectionalRange + View,
        F: Fn(&R::Element) -> bool,
    {
        fn before(&self, mut i: Self::Position) -> Self::Position {
            i = self.range.before(i);
            loop {
                if self.satisfies_pred(&i) {
                    break;
                }
                i = self.range.before(i)
            }
            i
        }
    }

    // TODO: Should SemiOutputRange be supported? Theoratically, its possible
    // but I see some edge cases with side effects and find it rarely useful
    // for any stuff. If there are good usecases, please contribute.
}

/// Creates a filter view which contains only the element that satisfies given predicate.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view from given view which contains only the elements that satisfied pred.
///   - Follows following traits:
///     - InputRange: YES.
///     - BoundedRange: If given view satisfies BoundedRange.
///     - ForwardRange: If given view satisfies ForwardRange.
///     - BidirectionalRange: If given view satisfies BidirectionalRange.
///     - RandomAccessRange: NO
///     - SemiOutputRange: NO
///     - OutputRange: NO
///
/// # Example
/// ```rust
/// use stl::*;
/// use rng::infix::*;
/// use view::infix::*;
///
/// let arr = [1, 2, 3, 4, 5, 6];
///
/// let v = view::filter(arr.view(), |x| x % 2 == 1);
/// assert!(v.all_of(|x| x % 2 == 1));
///
/// let v = arr.view()
///            .filter(|x| x % 2 == 1);
/// assert!(v.all_of(|x| x % 2 == 1));
/// ```
pub fn filter<RangeView, Pred>(
    view: RangeView,
    pred: Pred,
) -> __details::FilterView<RangeView, Pred>
where
    RangeView: InputRange + View,
    Pred: Fn(&RangeView::Element) -> bool,
{
    __details::FilterView { range: view, pred }
}

pub mod infix {
    use crate::{InputRange, View};

    use super::__details;

    /// `filter`.
    pub trait STLFilterExt: InputRange + View + Sized {
        fn filter<Pred>(self, pred: Pred) -> __details::FilterView<Self, Pred>
        where
            Pred: Fn(&Self::Element) -> bool,
        {
            super::filter(self, pred)
        }
    }

    impl<R> STLFilterExt for R where R: InputRange + View {}
}
