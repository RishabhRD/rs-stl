// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::SemiRegular;

pub trait InputRange {
    // Type of the element contained in self
    type Element;

    // Type of the positions in self
    type Position: SemiRegular;

    // Returns the position of first element in self,
    // or if self is empty then is_last_position(first_position()) == true
    fn first_position(&self) -> Self::Position;

    // Returns true if index is "past the end" position in self, that is, the
    // position is immediately after the last element in self
    fn is_last_position(&self, i: &Self::Position) -> bool;

    // Returns position immediately after i
    //
    // Requires: is_last_position(i) == false
    fn position_after(&self, i: Self::Position) -> Self::Position;

    // Access element at position i
    //
    // Requires: i is a valid position in self and is_last_position(i) == false
    fn at(&self, i: &Self::Position) -> &Self::Element;
}

pub trait OutputRange: InputRange {
    // Access element at position i
    //
    // Requires: i is a valid position in self and is_last_position(i) == false
    fn at(&mut self, i: &Self::Position) -> &mut Self::Element;
}
