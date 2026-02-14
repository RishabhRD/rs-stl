// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Slice, SliceMut};

/// Any type that is movable, destructable and equality comparable.
///
/// As per Stepanov (not exact), Type is
///   - Movable
///   - Destructable
///   - Equality comparable
pub trait SemiRegular: Send + Eq {}
impl<T> SemiRegular for T where T: Send + Eq {}

/// Any SemiRegular type that is cloneable.
///
/// As per Stepanov (not exact), Type is
///   - SemiRegular
///   - Cloneable
pub trait Regular: SemiRegular + Clone {}
impl<T> Regular for T where T: SemiRegular + Clone {}

/// A multi-pass linear sequence of elements.
///
/// Representation:
/// ```text
///   _ _ _ _ _ _
///
///   ^            ^
///   |            |
/// start   -->   end
pub trait Collection {
    /// Type represting position in the collection.
    type Position: Regular + Ord;

    /// Type of element in the collection.
    type Element;

    /// Type that is like `&Element`. For collections whose elements are in
    /// memory, its simply `&Element`.
    type ElementRef<'a>: std::ops::Deref<Target = Self::Element>
    where
        Self: 'a; // Someday if rust supports yield once coroutines like swift,
                  // then this proxy reference technique is not needed.

    /// An unsafe collection representing contiguous subrange of `self`.
    type SubSequence: UnsafeSubSequence<
        Position = Self::Position,
        Element = Self::Element,
        SubSequence = Self::SubSequence,
        MutableSubSequence = Self::MutableSubSequence,
    >;

    /// An unsafe collection representing mutable contiguous subrange of `self`.
    ///
    /// For immutable collections, this is same as `SubSequence`.
    type MutableSubSequence: UnsafeSubSequence<
        Position = Self::Position,
        Element = Self::Element,
        SubSequence = Self::SubSequence,
        MutableSubSequence = Self::MutableSubSequence,
    >;

    /// Returns the position of first element in `self` if `self` is not empty;
    /// Returns end position otherwise.
    fn start(&self) -> Self::Position;

    /// Returns the position just after last element in collection.
    fn end(&self) -> Self::Position;

    /// Set `p` to next position after `p`.
    ///
    /// # Precondition
    ///   - Next position of `p` should be well defined.
    fn form_next(&self, p: &mut Self::Position);

    /// Increments `p` by `n`.
    ///
    /// # Precondition
    ///   - There are `n` valid positions in self after `p`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
    fn form_next_n(&self, p: &mut Self::Position, mut n: usize) {
        while n > 0 {
            self.form_next(p);
            n -= 1;
        }
    }

    /// Increments `p` by `n` and returns true; unless the distance is beyond `limit` in which case
    /// set `p` to `limit` and returns false.
    ///
    /// # Precondition
    ///   - `limit` should be increment-reachable from `p`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
    fn form_next_n_limited_by(
        &self,
        p: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        let mut n = n;
        while *p != limit && n > 0 {
            self.form_next(p);
            n -= 1;
        }
        n == 0
    }

    /// Returns position immediately after `p`.
    ///
    /// # Precondition
    ///   - Next position of `p` should be well defined.
    fn next(&self, mut p: Self::Position) -> Self::Position {
        self.form_next(&mut p);
        p
    }

    /// Returns nth position after `p`.
    ///
    /// # Precondition
    ///   - There are `n` valid positions in self after `p`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
    fn next_n(&self, mut p: Self::Position, n: usize) -> Self::Position {
        self.form_next_n(&mut p, n);
        p
    }

    /// Returns `n`th position after `p`, unless `n` is beyond `limit`.
    ///
    /// # Precondition
    ///   - `limit` should be increment-reachable from `p`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
    fn next_n_limited_by(
        &self,
        mut p: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        let success = self.form_next_n_limited_by(&mut p, n, limit);
        if success {
            Some(p)
        } else {
            None
        }
    }

    /// Returns number of elements in `[from, to)`.
    ///
    /// # Complexity
    ///   - O(1) if `RandomAccessCollection`; O(`n`) otherwise.
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
    ///   - O(1) if `RandomAccessCollection`; O(`n`) otherwise.
    fn count(&self) -> usize {
        self.distance(self.start(), self.end())
    }

    /// Returns count less than or equal to number of elements in collection.
    ///
    /// # Complexity
    ///   - O(1) if `RandomAccessCollection`; O(`n`) otherwise.
    fn underestimated_count(&self) -> usize {
        self.count()
    }

    /// Yields element at `i`th position.
    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_>;

    /// A contiguous subrange of `self` having elements in position `[from, to)`.
    ///
    /// The slice share positions with `self`.
    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::SubSequence>;
}

/// A collection whose elements are computed on element access.
pub trait LazyCollection: Collection
where
    Self::SubSequence: LazyCollection,
{
    /// Computes and returns `i`th element.
    fn compute_at(&self, i: &Self::Position) -> Self::Element;
}

/// A collection which supports backward traversal as well as forward traversal.
pub trait BidirectionalCollection: Collection
where
    Self::SubSequence: BidirectionalCollection,
    Self::MutableSubSequence: BidirectionalCollection,
{
    /// Sets `p` to position just before `p`.
    ///
    /// # Precondition
    ///   - Position before `p` should be well defined.
    fn form_prior(&self, p: &mut Self::Position);

    /// Sets `p` to `n` positions before `p`.
    ///
    /// # Precondition
    ///   - There are `n` valid positions in self before `p`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
    fn form_prior_n(&self, p: &mut Self::Position, mut n: usize) {
        while n > 0 {
            self.form_prior(p);
            n -= 1;
        }
    }

    /// Increments `p` by `n` and returns true; unless the distance is beyond `limit`
    /// in which case set `p` to `limit` and returns false.
    ///
    /// # Precondition
    ///   - `limit` should be decrement-reachable from `position`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
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

    /// Returns position immediately before `p`
    ///
    /// # Precondition
    ///   - Position before `p` should be well defined.
    fn prior(&self, mut p: Self::Position) -> Self::Position {
        self.form_prior(&mut p);
        p
    }

    /// Returns `n`th position before `p`
    ///
    /// # Precondition
    ///   - There are `n` valid positions before `p`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
    fn prior_n(&self, mut p: Self::Position, n: usize) -> Self::Position {
        self.form_prior_n(&mut p, n);
        p
    }

    /// Returns `n`th position before `p`, unless `n` position is beyond `limit`.
    ///
    /// # Precondition
    ///   - `limit` should be increment-reachable from `p`.
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; O(`n`) otherwise.
    fn prior_n_limited_by(
        &self,
        mut p: Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> Option<Self::Position> {
        let success = self.form_prior_n_limited_by(&mut p, n, limit);
        if success {
            Some(p)
        } else {
            None
        }
    }
}

/// A collection that supports efficient random access traversal.
///
/// Random access collections can move any position any distance and measure
/// distance between positions in O(1) time.
pub trait RandomAccessCollection: BidirectionalCollection
where
    Self::SubSequence: RandomAccessCollection,
    Self::MutableSubSequence: RandomAccessCollection,
{
}

/// A collection which supports internally reordering its element.
pub trait ReorderableCollection: Collection
where
    Self::MutableSubSequence: UnsafeReorderableSubSequence,
{
    /// Swaps element at position `i` with element at position `j`.
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);

    /// A contiguous mutable subrange of `self` having elements in position `[from, to)`.
    ///
    /// The slice share positions with `self`.
    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::MutableSubSequence>;
}

/// A collection which allows mutable access to its elements.
pub trait MutableCollection: ReorderableCollection
where
    Self::MutableSubSequence: UnsafeMutableSubSequence,
{
    /// Returns mutable reference to element at `i`th position.
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}

/// A low-level, unchecked view into contiguous subsequence of a collection.
///
/// NOTE: The trait doesn't enforce usual lifetime tracking. It is intended to
/// be wrapped in a safe view (i.e., `Slice`).
pub trait UnsafeSubSequence: Collection {
    /// Yields reference to `i`th element with lifetime `'a`.
    unsafe fn unsafe_at<'a>(&self, i: &Self::Position) -> Self::ElementRef<'a>;

    /// Returns a slice of elements in position range `[from, to)`.
    unsafe fn unsafe_slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubSequence;

    /// Set the start position of `self` to `p`.
    ///
    /// - Precondition: `p` belongs to `[self.start(), self.end()]`.
    unsafe fn set_start(&mut self, p: Self::Position);

    /// Set the end position of `self` to `p`.
    ///
    /// - Precondition: `p` belongs to `[self.start(), self.end()]`.
    unsafe fn set_end(&mut self, p: Self::Position);
}

/// A low-level re-orderable contiguous subsequence of a collection.
///
/// NOTE: The trait doesn't enforce usual lifetime tracking. It is intended to
/// be wrapped in a safe view (i.e., `SliceMut`).
pub trait UnsafeReorderableSubSequence:
    ReorderableCollection + UnsafeSubSequence
where
    Self::MutableSubSequence: UnsafeReorderableSubSequence,
{
    /// Returns a mutable slice of elements in position range `[from, to)`.
    unsafe fn unsafe_slice_mut(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::MutableSubSequence;
}

/// A low-level mutable contiguous subsequence of a collection.
///
/// NOTE: The trait doesn't enforce usual lifetime tracking. It is intended to
/// be wrapped in a safe view (i.e., `SliceMut`).
pub trait UnsafeMutableSubSequence:
    UnsafeReorderableSubSequence + MutableCollection
where
    Self::MutableSubSequence: UnsafeMutableSubSequence,
{
    fn unsafe_at_mut<'a>(&self, i: &Self::Position) -> &'a mut Self::Element;
}
