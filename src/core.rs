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
    type Position: Regular + Ord;

    /// Type of element in the collection.
    type Element;

    /// Type that is like `&Element`. For collections whose elements are in
    /// memory, its simply `&Element`.
    type ElementRef<'a>: std::ops::Deref<Target = Self::Element>
    where
        Self: 'a; // Someday if rust supports yield once coroutines like swift,
                  // then this proxy reference technique is not needed.

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

    /// Mutates given `position` to next position in collection.
    ///
    /// # Precondition
    ///   - `position != end()`
    fn form_next(&self, position: &mut Self::Position);

    /// Increments given position by `n`.
    ///
    /// # Precondition
    ///   - There are `n` valid positions in self after `position`.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn form_next_n(&self, position: &mut Self::Position, mut n: usize) {
        while n > 0 {
            self.form_next(position);
            n -= 1;
        }
    }

    /// Increments given position by `n`, or so that it equals the given limit.
    ///
    /// # Precondition
    ///   - `limit` is a valid position in `self`.
    ///   - `limit` should be increment-reachable from `position`.
    ///
    /// # Postcondition
    ///   - Returns true if `position` has been incremented exactly by `n`
    ///     without going beyond limit, otherwise returns false.
    ///   - When the return value is false, the value `position` is equal to
    ///     `limit`.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        let mut n = n;
        while *position != limit && n > 0 {
            self.form_next(position);
            n -= 1;
        }
        n == 0
    }

    /// Returns position immediately after `position`.
    ///
    /// # Precondition
    ///   - `position != end()`
    fn next(&self, mut position: Self::Position) -> Self::Position {
        self.form_next(&mut position);
        position
    }

    /// Returns nth position after `position`.
    ///
    /// # Precondition
    ///   - There are n valid positions in self after `position`.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn next_n(&self, mut position: Self::Position, n: usize) -> Self::Position {
        self.form_next_n(&mut position, n);
        position
    }

    /// Returns `n`th position after given position, unless `n` is beyond `limit`.
    ///
    /// # Precondition
    ///   - `limit` is a valid position in `self`.
    ///   - `limit` should be increment-reachable from `position`.
    ///
    /// # Postcondition
    ///   - Returns `n`th position after given position, unless `n` is beyond `limit`.
    ///   - Otherwise, returns None.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn next_n_limited_by(
        &self,
        mut position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        let success = self.form_next_n_limited_by(&mut position, n, limit);
        if success {
            Some(position)
        } else {
            None
        }
    }

    /// Returns number of elements in `[from, to)`.
    ///
    /// # Precondition
    ///   - `[from, to)` represents valid positions in the collection.
    ///
    /// # Complexity
    ///   - O(1) if RandomAccessCollection; O(n) otherwise.
    fn distance(&self, mut from: Self::Position, to: Self::Position) -> usize {
        let mut dist = 0;
        while from != to {
            dist += 1;
            from = self.next(from);
        }
        dist
    }

    /// Returns number of elements in collection.
    ///
    /// # Complexity
    ///   - O(1) if RandomAccessCollection; O(n) otherwise.
    fn count(&self) -> usize {
        self.distance(self.start(), self.end())
    }

    /// Returns count less than or equal to number of elements in collection.
    ///
    /// # Complexity
    ///   - O(1) if RandomAccessCollection; O(n) otherwise.
    fn underestimated_count(&self) -> usize {
        self.count()
    }

    /// Access element at position i.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    ///   - O(1)
    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_>;

    /// Returns slice of collection in positions `[from, to)`.
    ///
    /// # Precondition
    ///   - `[from, to)` represents valid positions in collection.
    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole>;
}

/// Models a collection whose elements are computed on element access.
pub trait LazyCollection: Collection
where
    Self::Whole: LazyCollection,
{
    /// Computes element at position `i`.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    ///   - O(1)
    fn compute_at(&self, i: &Self::Position) -> Self::Element;
}

/// Models a bidirectional collection, which can be traversed forward as well as backward.
pub trait BidirectionalCollection: Collection
where
    Self::Whole: BidirectionalCollection,
{
    /// Mutates the given position to position just before `position`.
    ///
    /// # Precondition
    ///   - `position != start()`
    fn form_prior(&self, position: &mut Self::Position);

    /// Mutates the given position to nth position before `position`.
    ///
    /// # Precondition
    ///   - There are n valid positions in self before `position`.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn form_prior_n(&self, position: &mut Self::Position, mut n: usize) {
        while n > 0 {
            self.form_prior(position);
            n -= 1;
        }
    }

    /// Decrements given position by `n`, or so that it equals the given limit.
    ///
    /// # Precondition
    ///   - `limit` is a valid position in `self`.
    ///   - `limit` should be decrement-reachable from `position`.
    ///
    /// # Postcondition
    ///   - Returns true if `position` has been decremented exactly by `n`
    ///     without going beyond limit, otherwise returns false.
    ///   - When the return value is false, the value `position` is equal to
    ///     `limit`.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        let mut n = n;
        while *position != limit && n > 0 {
            self.form_prior(position);
            n -= 1;
        }
        n == 0
    }

    /// Returns position immediately before `position`
    ///
    /// # Precondition
    ///   - `position != start()`
    fn prior(&self, mut position: Self::Position) -> Self::Position {
        self.form_prior(&mut position);
        position
    }

    /// Returns nth position before `position`
    ///
    /// # Precondition
    ///   - self has n valid positions before `position`.
    ///
    /// # Complexity
    /// n applications of before.
    fn prior_n(
        &self,
        mut position: Self::Position,
        n: usize,
    ) -> Self::Position {
        self.form_prior_n(&mut position, n);
        position
    }

    /// Returns `n`th position before given position, unless `n` is beyond `limit`.
    ///
    /// # Precondition
    ///   - `limit` is a valid position in `self`.
    ///   - `limit` should be increment-reachable from `position`.
    ///
    /// # Postcondition
    ///   - Returns `n`th position before given position, unless `n` is beyond `limit`.
    ///   - Otherwise, returns None.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection; O(n) otherwise.
    fn prior_n_limited_by(
        &self,
        mut position: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        let success = self.form_prior_n_limited_by(&mut position, n, limit);
        if success {
            Some(position)
        } else {
            None
        }
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
///   - `self.distance(from, to)` -> O(1).
///   - `self.form_next_n(i)` -> O(1).
///   - `self.form_prior_n(i)` -> O(1).
///   - `self.next_n(i)` -> O(1).
///   - `self.prior_n(i)` -> O(1).
///
///   NOTE: If complexity requirements are not formed any algorithm on RandomAccessCollection
///   have undefined behavior.
pub trait RandomAccessCollection: BidirectionalCollection
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

    /// Returns mutable slice of collection in positions `[from, to)`.
    ///
    /// # Precondition
    ///   - `[from, to)` represents valid positions in collection.
    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::Whole>;
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
    ///   - O(1)
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}
