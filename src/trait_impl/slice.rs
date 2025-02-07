// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, Collection, MutableCollection, MutableRange,
    RandomAccessRange, Range
};

impl<T> Range for [T] {
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
        assert!(i + n <= self.len());
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        self.at(i)
    }
}

impl<T> Collection for [T] {
    fn at(&self, i: &Self::Position) -> &T {
        assert!(*i != self.len());
        &self[*i]
    }
}

impl<T> BidirectionalRange for [T] {
    fn before(&self, i: Self::Position) -> Self::Position {
        assert!(i > 0);
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        assert!(i >= n);
        i - n
    }
}

impl<T> RandomAccessRange for [T] {}


impl<T> MutableRange for [T] {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j)
    }
}

impl<T> MutableCollection for [T] {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }
}
