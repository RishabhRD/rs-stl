// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalRange, ForwardRange, InputRange, OutputRange,
    RandomAccessRange,
};

impl<T> InputRange for Vec<T> {
    type Element = T;

    type Position = usize;

    fn start_position(&self) -> Self::Position {
        0
    }

    fn end_position(&self) -> Self::Position {
        self.len()
    }

    fn position_after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self[*i]
    }
}

impl<T> ForwardRange for Vec<T> {}

impl<T> BidirectionalRange for Vec<T> {
    fn position_before(&self, i: Self::Position) -> Self::Position {
        i - 1
    }
}

impl<T> RandomAccessRange for Vec<T> {
    fn nth_position_after(
        &self,
        i: Self::Position,
        n: usize,
    ) -> Self::Position {
        i + n
    }

    fn nth_position_before(
        &self,
        i: Self::Position,
        n: usize,
    ) -> Self::Position {
        i - n
    }
}

impl<T> OutputRange for Vec<T> {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        &mut self[*i]
    }
}
