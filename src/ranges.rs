// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Regular, SemiRegular};

pub trait InputRange {
    // Type of the element contained in self
    type Element;

    // Type of the positions in self
    type Position: SemiRegular;

    // Returns the position of first element in self,
    // or if self is empty then is_last_position(first_position()) == true
    fn start(&self) -> Self::Position;

    // Returns the "past the end" position in self, that is, the position
    // immediately after the last element in self
    fn end(&self) -> Self::Position;

    // Returns position immediately after i
    //
    // Requires: is_last_position(i) == false
    fn after(&self, i: Self::Position) -> Self::Position;

    // Access element at position i
    //
    // Requires: i is a valid position in self and is_last_position(i) == false
    fn at(&self, i: &Self::Position) -> &Self::Element;
}

pub trait ForwardRange: InputRange<Position: Regular> {}

pub trait BidirectionalRange: ForwardRange {
    // Returns position immediately before i
    //
    // Requires: i != first_position()
    fn before(&self, i: Self::Position) -> Self::Position;
}

pub trait RandomAccessRange:
    BidirectionalRange<Position: Regular + Ord>
{
    // Returns nth position after i
    //
    // Requires: there should be n valid positions after i
    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position;

    // Returns nth position before i
    //
    // Requires: there should be n valid positions before i
    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position;
}

pub trait OutputRange: ForwardRange {
    // Access element at position i
    //
    // Requires: i is a valid position in self and is_last_position(i) == false
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}
