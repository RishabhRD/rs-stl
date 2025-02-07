// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, Collection, MutableCollection, MutableRange,
    RandomAccessRange, Range
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Array<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Range for Array<T, N> {
    type Position = usize;

    type Element = T;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        N
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        assert!(i != N);
        i + 1
    }

    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        self.at(i)
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }
}

impl<T, const N: usize> Collection for Array<T, N> {
    fn at(&self, i: &Self::Position) -> &T {
        assert!(*i != N);
        &self.0[*i]
    }
}

impl<T, const N: usize> BidirectionalRange for Array<T, N> {
    fn before(&self, i: Self::Position) -> Self::Position {
        assert!(i > 0);
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        n - i
    }
}

impl<T, const N: usize> RandomAccessRange for Array<T, N> {}


impl<T, const N: usize> MutableRange for Array<T, N> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.0.swap(*i, *j);
    }
}

impl<T, const N: usize> MutableCollection for Array<T, N> {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        assert!(*i != N);
        &mut self.0[*i]
    }
}
