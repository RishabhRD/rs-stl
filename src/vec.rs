// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#![doc(hidden)]

use crate::{
    BidirectionalRange, ForwardRange, InputRange, OutputRange,
    RandomAccessRange,
};

impl<T> InputRange for Vec<T> {
    type Element = T;

    type Position = usize;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        self.len()
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self[*i]
    }
}

impl<T> ForwardRange for Vec<T> {
    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }
}

impl<T> BidirectionalRange for Vec<T> {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T> RandomAccessRange for Vec<T> {}

impl<T> OutputRange for Vec<T> {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }

    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j)
    }
}
