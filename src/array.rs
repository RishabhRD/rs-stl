// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#![doc(hidden)]

use crate::{
    BidirectionalRange, BoundedRange, ForwardRange, InputRange, OutputRange,
    RandomAccessRange, SemiOutputRange,
};

impl<T, const N: usize> InputRange for [T; N] {
    type Element = T;

    type ElementRef<'a>
        = &'a T
    where
        Self: 'a;

    type Position = usize;

    fn start(&self) -> Self::Position {
        0
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
        &self[*i]
    }

    fn is_end(&self, i: &Self::Position) -> bool {
        *i == N
    }
}

impl<T, const N: usize> BoundedRange for [T; N] {
    fn end(&self) -> Self::Position {
        N
    }
}

impl<T, const N: usize> ForwardRange for [T; N] {
    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
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

impl<T, const N: usize> SemiOutputRange for [T; N] {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j);
    }
}

impl<T, const N: usize> OutputRange for [T; N] {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }
}
