// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, View};

#[doc(hidden)]
pub mod __details_suffix_view {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct SuffixView<Range>
    where
        Range: ForwardRange + View,
    {
        pub range: Range,
        pub from: Range::Position,
    }

    impl<R> View for SuffixView<R> where R: ForwardRange + View {}

    impl<R> InputRange for SuffixView<R>
    where
        R: ForwardRange + View,
    {
        type Element = R::Element;

        type Position = R::Position;

        fn start(&self) -> Self::Position {
            self.from.clone()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            self.range.is_end(i)
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

    impl<R> BoundedRange for SuffixView<R>
    where
        R: ForwardRange + BoundedRange + View,
    {
        fn end(&self) -> Self::Position {
            self.range.end()
        }
    }

    impl<R> ForwardRange for SuffixView<R>
    where
        R: ForwardRange + View,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R> BidirectionalRange for SuffixView<R>
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

    impl<R> RandomAccessRange for SuffixView<R> where R: RandomAccessRange + View {}

    impl<R> SemiOutputRange for SuffixView<R>
    where
        R: SemiOutputRange + View,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j)
        }
    }

    impl<R> OutputRange for SuffixView<R>
    where
        R: OutputRange + View,
    {
        fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
            self.range.at_mut(i)
        }
    }
}

#[doc(hidden)]
pub trait STLSuffixViewExt: ForwardRange + View + Sized {
    /// Returns suffix view `[from, self.is_end())` of given view.
    ///
    /// # Precondition
    ///   - till is a valid position in self.
    fn suffix(
        self,
        from: Self::Position,
    ) -> __details_suffix_view::SuffixView<Self> {
        __details_suffix_view::SuffixView { range: self, from }
    }
}

impl<R> STLSuffixViewExt for R where R: ForwardRange + View {}
