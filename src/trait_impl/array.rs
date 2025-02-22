// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    ArraySlice, BidirectionalRange, Collection, HasSlice, MutableArraySlice,
    MutableCollection, MutableRange, RandomAccessRange, Range, RangeBase,
};

impl<T, const N: usize> RangeBase for [T; N] {
    type Position = usize;

    type Element = T;
}

impl<'a, T, const N: usize> HasSlice<'a> for [T; N] {
    type Slice = ArraySlice<'a, T>;

    type MutableSlice = MutableArraySlice<'a, T>;
}

impl<T, const N: usize> Range for [T; N] {
    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        N - 1
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        &self[*i]
    }

    fn slice<'a>(
        &'a self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as HasSlice<'a>>::Slice
    where
        Self: 'a,
    {
        ArraySlice::new(self, from, to)
    }
}

impl<T, const N: usize> BidirectionalRange for [T; N] {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T, const N: usize> RandomAccessRange for [T; N] {}

impl<T, const N: usize> MutableRange for [T; N] {
    fn slice_mut<'a>(
        &'a mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as HasSlice<'a>>::MutableSlice
    where
        Self: 'a,
    {
        MutableArraySlice::new(self, from, to)
    }

    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j)
    }
}

impl<T, const N: usize> Collection for [T; N] {
    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self[*i]
    }
}

impl<T, const N: usize> MutableCollection for [T; N] {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }
}
