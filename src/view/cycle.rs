// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{InputRange, View};

mod __details {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange, View,
    };

    #[derive(Clone)]
    pub struct CycleView<Range>
    where
        Range: InputRange + View,
    {
        pub range: Range,
    }

    #[derive(Clone, PartialEq, Eq)]
    pub struct CyclePosition<P> {
        pub cur_pos: P,
        pub cycle_completed: usize,
    }

    impl<R> InputRange for CycleView<R>
    where
        R: InputRange + View,
    {
        type Element = R::Element;

        type Position = CyclePosition<R::Position>;

        type ElementRef<'a>
            = R::ElementRef<'a>
        where
            Self: 'a;

        fn start(&self) -> Self::Position {
            CyclePosition {
                cur_pos: self.range.start(),
                cycle_completed: 0,
            }
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(&i.cur_pos)
        }

        fn after(&self, mut i: Self::Position) -> Self::Position {
            // Note: self.range.after should handle the bounds check here
            i.cur_pos = self.range.after(i.cur_pos);
            if self.range.is_end(&i.cur_pos) {
                i.cur_pos = self.range.start();
                i.cycle_completed += 1;
            }
            i
        }

        fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
            self.range.at(&i.cur_pos)
        }
    }

    impl<R> View for CycleView<R> where R: InputRange + View {}

    impl<R> ForwardRange for CycleView<R> where R: ForwardRange + View {}

    impl<R> BidirectionalRange for CycleView<R>
    where
        R: BoundedRange + BidirectionalRange + View,
    {
        fn before(&self, mut i: Self::Position) -> Self::Position {
            if i.cur_pos == self.range.start() {
                i.cur_pos = self.range.before(self.range.end());
                assert_ne!(
                    i.cycle_completed, 0,
                    "before called on start of range"
                );
                i.cycle_completed -= 1;
            } else {
                i.cur_pos = self.range.before(i.cur_pos);
            }
            i
        }
    }
}

/// Returns a view that is cyclic view of given view.
///
/// # Precondition
///
/// # Postcondition
///   - Returns a view that is cyclic view of given view.
///   - Satisfies following range traits:
///     - InputRange: YES
///     - BoundedRange: NO
///     - ForwardRange: If given view if ForwardRange
///     - BidirectionalRange: If given view if BidirectionalRange and BoundedRange
///     - RandomAccessRange: NO
///     - SemiOutputRange: NO
///     - OutputRange: NO
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
/// let arr = [1, 2, 3];
/// let v = view::cycle(arr.view());
/// let n = v.at(&v.after_n(v.start(), 3));
/// assert_eq!(*n, 1);
///
/// let arr = [1, 2, 3];
/// let v = arr.view().cycle();
/// let n = v.at(&v.after_n(v.start(), 4));
/// assert_eq!(*n, 2);
/// ```
pub fn cycle<RangeView>(view: RangeView) -> __details::CycleView<RangeView>
where
    RangeView: InputRange + View,
{
    __details::CycleView { range: view }
}

pub mod infix {
    use super::__details;
    use crate::{InputRange, View};

    /// `cycle`.
    pub trait STLCycleExt: InputRange + View + Sized {
        fn cycle(self) -> __details::CycleView<Self> {
            super::cycle(self)
        }
    }

    impl<R> STLCycleExt for R where R: InputRange + View + Sized {}
}
