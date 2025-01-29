// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    ArraySlice, BidirectionalRange, Collection, RandomAccessRange, Range,
    RangeBase, SubRangeable,
};

impl<T, const N: usize> RangeBase for [T; N] {
    type Position = usize;

    type Element = T;
}

impl<'this, T, const N: usize> SubRangeable<'this> for [T; N] {
    type SubRange = ArraySlice<'this, T>;
}

impl<T, const N: usize> Range for [T; N] {
    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        self.len()
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        assert!(i != self.len());
        i + 1
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as SubRangeable<'_>>::SubRange {
        ArraySlice::new(self, from, to)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        assert!(i + n <= N);
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn at_as_deref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        self.at(i)
    }
}

impl<T, const N: usize> Collection for [T; N] {
    fn at(&self, i: &Self::Position) -> &T {
        assert!(*i != self.len());
        &self[*i]
    }
}

impl<T, const N: usize> BidirectionalRange for [T; N] {
    fn before(&self, i: Self::Position) -> Self::Position {
        assert!(i > 0);
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        assert!(i >= n);
        i - n
    }
}

impl<T, const N: usize> RandomAccessRange for [T; N] {}
