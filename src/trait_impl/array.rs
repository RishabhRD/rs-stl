// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, Collection, MutableCollection, MutableRange,
    RandomAccessRange, Range, ReorderableRange,
};

impl<T, const N: usize> Range for [T; N] {
    type Position = usize;

    type Element = T;

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
    fn at(&self, i: &Self::Position) -> &Self::Element {
        assert!(*i != N);
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

impl<T, const N: usize> MutableRange for [T; N] {}

impl<T, const N: usize> ReorderableRange for [T; N] {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j)
    }
}

impl<T, const N: usize> MutableCollection for [T; N] {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }
}
