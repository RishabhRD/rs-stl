// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, View};

#[doc(hidden)]
pub mod __details_subrange {

    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };
    pub struct SubRangeView<Range: ForwardRange + View> {
        pub range: Range,
        pub start: Range::Position,
        pub end: Range::Position,
    }

    impl<R> View for SubRangeView<R> where R: ForwardRange + View {}

    impl<R> InputRange for SubRangeView<R>
    where
        R: ForwardRange + View,
    {
        type Element = R::Element;

        type Position = R::Position;

        fn start(&self) -> Self::Position {
            self.start.clone()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            *i == self.end
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            self.range.after(i)
        }

        fn at(&self, i: &Self::Position) -> &Self::Element {
            self.range.at(i)
        }

        fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
            self.range.after_n(i, n)
        }
    }

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
            self.range.swap_at(i, j);
        }
    }

    impl<R> OutputRange for SubRangeView<R>
    where
        R: OutputRange + View,
    {
        fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
            self.range.at_mut(i)
        }
    }
}

#[doc(hidden)]
pub trait STLSubRangeExt: ForwardRange + View + Sized {
    /// Returns subrange view `[start, end)` of given view.
    ///
    /// # Precondition
    ///   - `[start, end)` should represent valid positions in self.
    fn subrange(
        self,
        start: Self::Position,
        end: Self::Position,
    ) -> __details_subrange::SubRangeView<Self> {
        __details_subrange::SubRangeView {
            range: self,
            start,
            end,
        }
    }
}

impl<R> STLSubRangeExt for R where R: ForwardRange + View {}
