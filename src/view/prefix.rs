// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{ForwardRange, View};

#[doc(hidden)]
pub mod __details_prefix_view {
    use crate::{
        BidirectionalRange, BoundedRange, ForwardRange, InputRange,
        OutputRange, RandomAccessRange, SemiOutputRange, View,
    };

    pub struct PrefixView<Range>
    where
        Range: ForwardRange + View,
    {
        pub range: Range,
        pub till: Range::Position,
    }

    impl<R> View for PrefixView<R> where R: ForwardRange + View {}

    impl<R> InputRange for PrefixView<R>
    where
        R: ForwardRange + View,
    {
        type Element = R::Element;

        type Position = R::Position;

        fn start(&self) -> Self::Position {
            self.range.start()
        }

        fn is_end(&self, i: &Self::Position) -> bool {
            i == &self.till
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

    impl<R> BoundedRange for PrefixView<R>
    where
        R: ForwardRange + View,
    {
        fn end(&self) -> Self::Position {
            self.till.clone()
        }
    }

    impl<R> ForwardRange for PrefixView<R>
    where
        R: ForwardRange + View,
    {
        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            self.range.distance(from, to)
        }
    }

    impl<R> BidirectionalRange for PrefixView<R>
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

    impl<R> RandomAccessRange for PrefixView<R> where R: RandomAccessRange + View {}

    impl<R> SemiOutputRange for PrefixView<R>
    where
        R: SemiOutputRange + View,
    {
        fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
            self.range.swap_at(i, j)
        }
    }

    impl<R> OutputRange for PrefixView<R>
    where
        R: OutputRange + View,
    {
        fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
            self.range.at_mut(i)
        }
    }
}

#[doc(hidden)]
pub trait STLPrefixViewExt: ForwardRange + View + Sized {
    /// Returns prefix view `[self.start(), till)` of given view.
    ///
    /// # Precondition
    ///   - till is a valid position in self.
    fn prefix(
        self,
        till: Self::Position,
    ) -> __details_prefix_view::PrefixView<Self> {
        __details_prefix_view::PrefixView { range: self, till }
    }
}

impl<R> STLPrefixViewExt for R where R: ForwardRange + View {}
