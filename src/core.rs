// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// 1-> Equality Comparable + Movable + Destructable.
///
/// As per Stepanov (not exact), Types can be:
///   - compared for equality
///   - moved
///   - destructed
pub trait SemiRegular: Eq {}
impl<T> SemiRegular for T where T: Eq {}

/// 2-> SemiRegular + Clonable
///
/// As per Stepanov (not exact), Types can be:
///   - SemiRegular
///   - cloned
pub trait Regular: SemiRegular + Clone {}
impl<T> Regular for T where T: SemiRegular + Clone {}

/// 3-> Models a single-pass range.
pub trait InputRange {
    /// Type of the element contained in self
    type Element;

    /// Type of the positions in self
    type Position: SemiRegular;

    /// Returns the position of first element in self,
    /// or if self is empty then start() == end()
    fn start(&self) -> Self::Position;

    /// Returns the "past the end" position in self, that is, the position
    /// immediately after the last element in self
    fn end(&self) -> Self::Position;

    /// Returns position immediately after i
    ///
    /// Requires: i != end()
    fn after(&self, i: Self::Position) -> Self::Position;

    /// Access element at position i
    ///
    /// Requires: i is a valid position in self and i != end()
    /// Complexity Requirement: O(1).
    fn at(&self, i: &Self::Position) -> &Self::Element;
}

/// 4-> Models a multi-pass range.
pub trait ForwardRange: InputRange<Position: Regular> {
    /// Returns distance between position `[from, to)`.
    ///
    /// Requires `[from, to)` represents valid position in range.
    /// Complexity: O(n)
    fn distance(&self, mut from: Self::Position, to: Self::Position) -> usize {
        let mut res = 0;
        while from != to {
            from = self.after(from);
            res += 1;
        }
        res
    }
}

/// 5-> Models a bidirectional range, which can be traversed forward as well as backward.
pub trait BidirectionalRange: ForwardRange {
    /// Returns position immediately before i
    ///
    /// Requires: i != first_position()
    fn before(&self, i: Self::Position) -> Self::Position;
}

/// 6-> Models a random access range (similar to array) where any position can be
/// reached in O(1).
///
/// Semantic Requirement: rng.distance(from, to) should work in O(1).
pub trait RandomAccessRange:
    BidirectionalRange<Position: Regular + Ord>
{
    /// Returns nth position after i
    ///
    /// Requires: there should be n valid positions after i
    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position;

    /// Returns nth position before i
    ///
    /// Requires: there should be n valid positions before i
    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position;
}

/// 7-> Models a mutable range.
pub trait OutputRange: ForwardRange {
    /// Access element at position i
    ///
    /// Requires: i is a valid position in self and i != end()
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;

    /// Swap elements at position i and j
    ///
    /// Requires: i and j is a valid position in self.
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);
}
