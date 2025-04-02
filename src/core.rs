// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Slice, SliceMut};

/// Any type that is movable, destructable and equality comparable.
///
/// As per Stepanov (not exact), Type is
///   - Movable
///   - Destructable
///   - Equality comparable
pub trait SemiRegular: Eq {}
impl<T> SemiRegular for T where T: Eq {}

/// Any SemiRegular type that is cloneable.
///
/// As per Stepanov (not exact), Type is
///   - SemiRegular
///   - Cloneable
pub trait Regular: SemiRegular + Clone {}
impl<T> Regular for T where T: SemiRegular + Clone {}

/// Models a multi-pass linear sequence of elements.
///
/// Representation:
/// ```text
///   _ _ _ _ _ _
///
///   ^            ^
///   |            |
/// start   -->   end
pub trait Collection {
    /// Type of positions in the collection.
    type Position: Regular;

    /// Type of element in the collection.
    type Element; // TODO: Finalize what to do with LazyCollection?

    /// Type representing whole collection.
    /// i.e., `Self == Slice<W> ? W : Self`
    type Whole: Collection<
        Position = Self::Position,
        Element = Self::Element,
        Whole = Self::Whole,
    >;

    /// Returns the position of first element in self,
    /// or if self is empty then start() == end()
    fn start(&self) -> Self::Position;

    /// Returns the position just after last element in collection.
    fn end(&self) -> Self::Position;

    /// Advances the given position to position just after i.
    ///
    /// # Precondition
    ///   - i != end()
    fn advance(&self, i: &mut Self::Position);

    /// Advances the given position to nth position after i.
    ///
    /// # Precondition
    ///   - There are n valid positions in self after i.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn advance_n(&self, i: &mut Self::Position, mut n: usize) {
        while n > 0 {
            self.advance(i);
            n -= 1;
        }
    }

    /// Returns position immediately after i
    ///
    /// # Precondition
    ///   - i != end()
    fn after(&self, mut i: Self::Position) -> Self::Position {
        self.advance(&mut i);
        i
    }

    /// Returns nth position after i.
    ///
    /// # Precondition
    ///   - There are n valid positions in self after i.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn after_n(&self, mut i: Self::Position, n: usize) -> Self::Position {
        self.advance_n(&mut i, n);
        i
    }

    /// Returns number of elements in `[from, to)`.
    ///
    /// # Precondition
    ///   - `[from, to)` represents valid positions in the collection.
    ///
    /// # Complexity
    ///   O(n).
    fn distance(&self, mut from: Self::Position, to: Self::Position) -> usize {
        let mut dist = 0;
        while from != to {
            dist += 1;
            from = self.after(from);
        }
        dist
    }

    /// Access element at position i.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    ///   O(1)
    fn at(&self, i: &Self::Position) -> &Self::Element;

    /// Returns slice of collection in positions [from, to).
    ///
    /// # Precondition
    ///   - [from, to) represents valid positions in collection.
    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<Self::Whole>;
}

/// Models a bidirectional collection, which can be traversed forward as well as backward.
/// Backward iteration is supported through `before` and `before_n` methods.
pub trait BidirectionalCollection: Collection
where
    Self::Whole: BidirectionalCollection,
{
    /// Backsteps the given position to position just before i.
    ///
    /// # Precondition
    ///   - i != start()
    fn backstep(&self, i: &mut Self::Position);

    /// Backsteps the given position to nth position before i.
    ///
    /// # Precondition
    ///   - There are n valid positions in self before i.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn backstep_n(&self, i: &mut Self::Position, mut n: usize) {
        while n > 0 {
            self.backstep(i);
            n -= 1;
        }
    }

    /// Returns position immediately before i
    ///
    /// # Precondition
    ///   - i != start()
    fn before(&self, mut i: Self::Position) -> Self::Position {
        self.backstep(&mut i);
        i
    }

    /// Returns nth position before i
    ///
    /// # Precondition
    ///   - self has n valid positions before i.
    ///
    /// # Complexity
    /// n applications of before.
    fn before_n(&self, mut i: Self::Position, n: usize) -> Self::Position {
        self.backstep_n(&mut i, n);
        i
    }
}

/// Models a random access collection (similar to array) where jumping to any position from any
/// other position is O(1) operation.
///
/// - RandomAccessCollection is extension of BidirectionalCollection.
/// - RandomAccessCollection enforces the Position of collection should be ordered.
/// - RandomAccessCollection doesn't add any new method but introduces complexity
///   requirements mentioned below. The complexity requirements ensure that
///   any position jump is O(1).
///
/// # Complexity Requirements
///   - `rng.distance(from, to)` -> O(1).
///   - `rng.advance_n(i)` -> O(1).
///   - `rng.backstep_n(i)` -> O(1).
///   - `rng.after_n(i)` -> O(1).
///   - `rng.before_n(i)` -> O(1).
///
///   NOTE: If complexity requirements are not formed any algorithm on RandomAccessCollection
///   have undefined behavior.
pub trait RandomAccessCollection:
    BidirectionalCollection<Position: Regular + Ord>
where
    Self::Whole: RandomAccessCollection,
{
}

/// Models a collection which supports internally reordering its element.
pub trait ReorderableCollection: Collection
where
    Self::Whole: ReorderableCollection,
{
    /// Swaps element at position i with element at position j.
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);

    /// Returns mutable slice of collection in positions [from, to).
    ///
    /// # Precondition
    ///   - [from, to) represents valid positions in collection.
    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<Self::Whole>;
}

/// Models a collection which supports mutating its element
pub trait MutableCollection: ReorderableCollection
where
    Self::Whole: MutableCollection,
{
    /// Mutably Access element at position i.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    ///   O(1)
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}
