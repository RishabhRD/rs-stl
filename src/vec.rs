// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#![doc(hidden)]

use crate::{
    BidirectionalRange, BoundedRange, Collection, ForwardRange, InputRange,
    OutputRange, RandomAccessRange, SemiOutputRange,
};

impl<T> InputRange for Vec<T> {
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

impl<'a, T> Collection<'a> for Vec<T> where Self: 'a {}

impl<T> BoundedRange for Vec<T> {
    fn end(&self) -> Self::Position {
        self.len()
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

impl<T> SemiOutputRange for Vec<T> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.swap(*i, *j)
    }
}

impl<T> OutputRange for Vec<T> {
    type ElementMutRef<'a>
        = &'a mut T
    where
        Self: 'a;

    fn at_mut<'a>(&'a mut self, i: &Self::Position) -> Self::ElementMutRef<'a> {
        &mut self[*i]
    }
}
