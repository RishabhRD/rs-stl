// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    slice::MutableSlice, CollectionIterator, LazyCollectionIterator, Slice,
};

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
pub trait Range {
    /// Type of positions in range.
    type Position: Regular;

    /// Type of element of range.
    type Element;

    /// Returns the position of first element in the range.
    ///
    /// If range is empty, returns end position of the range.
    fn start(&self) -> Self::Position;

    /// Returns position past the last element of the range.
    fn end(&self) -> Self::Position;

    /// Returns the position immediately after i.
    ///
    /// # Precondition
    ///   - `i != self.end()`.
    fn after(&self, i: Self::Position) -> Self::Position;

    /// Returns nth position after i.
    ///
    /// # Precondition
    ///   - There should be atleast n valid positions after i.
    ///   - Complexity: O(n) by default. Types can provide efficient
    ///     implementations if possible.
    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        let mut i = i;
        let mut n = n;
        while n > 0 {
            i = self.after(i);
            n -= 1;
        }
        i
    }

    /// Returns distance between `from` and `to`.
    ///
    /// # Precondition
    ///   - to should be reachable from from.
    ///
    /// # Postcondition
    ///   - Returns distance between `from` and `to`.
    ///   - Complexity: O(n) by default. Types can provide efficient implementation if possible.
    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        let mut from = from;
        let mut dist = 0;
        while from != to {
            dist += 1;
            from = self.after(from);
        }
        dist
    }

    /// Returns reference like view of element at position i.
    ///
    /// # Precondition
    ///   - `i != self.end()`
    fn at_as_deref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element>;
}

/// Models a range whose `Element`s are present in memory.
///
/// If elements are present in memory, then it should be possible to obtain
/// reference to elements.
pub trait Collection: Range {
    /// Returns reference to element at position i.
    fn at(&self, i: &Self::Position) -> &Self::Element;

    /// Returns iterator that iterates over references to elements in collection.
    fn iter(&self) -> CollectionIterator<Self> {
        CollectionIterator::new(self)
    }
}

/// Models a range whose elements are lazily computed on element access.
///
/// Thus, accessing element at any position would return value.
pub trait LazyCollection: Range {
    /// Returns the element at ith position.
    ///
    /// # Precondition
    ///   - i != self.end()
    fn at(&self, i: &Self::Position) -> Self::Element;

    /// Returns iterator that iterates over element values in lazy collection.
    fn iter(&self) -> LazyCollectionIterator<Self> {
        LazyCollectionIterator::new(self)
    }
}

/// Models a range that supports forward as well as backward traversal.
pub trait BidirectionalRange: Range {
    /// Returns position immediately before i.
    ///
    /// # Precondition
    ///   - `i != self.start()`
    fn before(&self, i: Self::Position) -> Self::Position;

    /// Returns nth position before i.
    ///
    /// # Precondition
    ///   - There are n valid positions in range before i.
    ///
    /// # Postcondition
    ///   - Returns nth position before i.
    ///   - Complexity: O(n) by default. Types can provide efficient implementation if possible.
    fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
        let mut i = i;
        let mut n = n;
        while n > 0 {
            i = self.before(i);
            n -= 1
        }
        i
    }
}

/// Models a random access range where jumping from one position to another is O(1) operation.
///
/// # Precondition
///   - self.after should work in O(1)
///   - self.after_n should work in O(1)
///   - self.before should work in O(1)
///   - self.before_n should work in O(1)
///   - self.distance should work in O(1)
///
/// # Postcondition
///   - RandomAccessRange doesn't provide any additional method but with given
///     precondition it ensures that jumping from one position to other can be
///     done in O(1).
pub trait RandomAccessRange: BidirectionalRange<Position: Ord> {}

#[doc(hidden)]
// TODO: make it unusable for outside use. One can use .slice() method but not
// the name __SliceExtension__.
pub trait __SliceExtension__: Range + Sized {
    fn slice(&self) -> Slice<'_, Self> {
        Slice::new(self, self.start(), self.end())
    }
}

impl<R> __SliceExtension__ for R where R: Range {}

/// Marker trait for marking range is mutable.
pub trait MutableRange: Range {}

/// Models a range whose elements can be reordered inside range.
pub trait ReorderableRange: MutableRange {
    /// Swaps element at ith position with element at jth position.
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);
}

/// Models a mutable collection that provides ability to mutate individual elements.
pub trait MutableCollection: MutableRange + Collection {
    /// Returns mutable reference to element at position i.
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}

/// Models a lazy collection which provides access to mutable view of element at any position.
pub trait MutableLazyCollection: MutableRange + LazyCollection {
    type ElementMut;

    /// Returns value of mutable view of element at position i.
    fn at_mut(&mut self, i: &Self::Position) -> Self::ElementMut;
}

#[doc(hidden)]
// TODO: make it unusable for outside use. One can use .slice() method but not
// the name __MutableSliceExtension__.
pub trait __MutableSliceExtension__: MutableRange + Sized {
    fn slice_mut(&mut self) -> MutableSlice<'_, Self> {
        MutableSlice::new(self, self.start(), self.end())
    }
}

impl<R> __MutableSliceExtension__ for R where R: MutableRange {}
