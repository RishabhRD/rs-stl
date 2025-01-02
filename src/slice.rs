// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#![doc(hidden)]

use crate::{
    BidirectionalRange, BoundedRange, ForwardRange, InputRange, OutputRange,
    RandomAccessRange, SemiOutputRange,
};

impl<T> InputRange for [T] {
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
        *i == self.len()
    }
}

impl<T> BoundedRange for [T] {
    fn end(&self) -> Self::Position {
        self.len()
    }
}

impl<T> ForwardRange for [T] {
    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }
}

impl<T> BidirectionalRange for [T] {
    fn before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }

    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i - n
    }
}

impl<T> RandomAccessRange for [T] {}

impl<T> SemiOutputRange for [T] {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j);
    }
}

impl<T> OutputRange for [T] {
    type ElementMutRef<'a>
        = &'a mut T
    where
        Self: 'a;

    fn at_mut<'a>(&'a mut self, i: &Self::Position) -> Self::ElementMutRef<'a> {
        &mut self[*i]
    }
}
