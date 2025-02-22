// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

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

mod sealed {
    pub trait Sealed: Sized {}
    pub struct Bounds<T>(T);
    impl<T> Sealed for Bounds<T> {}
}
use sealed::{Bounds, Sealed};

pub trait RangeBase {
    /// Type of position in the range.
    type Position: Regular;

    /// Type of element in the range.
    type Element;
}

pub trait HasSlice<'this, ImplicitBounds: Sealed = Bounds<&'this Self>>:
    RangeBase
{
    /// Type of slice of range.
    type Slice: Range;

    /// Type of mutale slice of range.
    ///
    /// For read only ranges, it should be Slice.
    type MutableSlice: Range;
}

/// Models a multi-pass linear sequence of elements.
///
/// Representation:
/// ```text
///   _ _ _ _ _ _
///
///   ^            ^
///   |            |
/// start   -->   end
/// ```
pub trait Range: for<'this> HasSlice<'this> {
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

    /// Returns reference-like structure to ith element of range.
    ///
    /// # Precondition
    ///   - i is a valid position in range.
    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element>;

    /// Returns slice `[from, to)` of range.
    ///
    /// # Precondition
    ///   - `[from, to)` are valid positions in range.
    fn slice<'a>(
        &'a self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as HasSlice<'a>>::Slice
    where
        Self: 'a;
}

/// Models a range which supports backward traversal.
pub trait BidirectionalRange: Range
where
    for<'a> <Self as HasSlice<'a>>::Slice: BidirectionalRange,
    for<'a> <Self as HasSlice<'a>>::MutableSlice: BidirectionalRange,
{
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
pub trait RandomAccessRange: BidirectionalRange
where
    for<'a> <Self as HasSlice<'a>>::Slice: RandomAccessRange,
    for<'a> <Self as HasSlice<'a>>::MutableSlice: RandomAccessRange,
{
}

/// Models range that supports internal mutation.
pub trait MutableRange: Range
where
    for<'a> <Self as HasSlice<'a>>::MutableSlice: MutableRange,
{
    /// Returns slice `[from, to)` of range.
    ///
    /// # Precondition
    ///   - `[from, to)` are valid positions in range.
    fn slice_mut<'a>(
        &'a mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as HasSlice<'a>>::MutableSlice
    where
        Self: 'a;

    /// Swaps element at ith position with element at jth position.
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);
}

/// Models a range whose elements are stored in memory i.e., one can return reference to those
/// elements.
pub trait Collection: Range
where
    for<'a> <Self as HasSlice<'a>>::Slice: Collection,
    for<'a> <Self as HasSlice<'a>>::MutableSlice: Collection,
{
    /// Returns reference to ith element of range.
    ///
    /// # Precondition
    ///   - i is a valid position in range.
    fn at(&self, i: &Self::Position) -> &Self::Element;
}

/// Models a collection that supports mutating its elements.
pub trait MutableCollection: Collection + MutableRange
where
    for<'a> <Self as HasSlice<'a>>::Slice: Collection,
    for<'a> <Self as HasSlice<'a>>::MutableSlice: MutableCollection,
{
    /// Returns mutable reference to ith element of range.
    ///
    /// # Precondition
    ///   - i is a valid position in range.
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element;
}

/// Models a range whose elements are computed on element access.
pub trait LazyCollection: Range
where
    for<'a> <Self as HasSlice<'a>>::Slice: LazyCollection,
    for<'a> <Self as HasSlice<'a>>::MutableSlice: LazyCollection,
{
    /// Returns reference to ith element of range.
    ///
    /// # Precondition
    ///   - i is a valid position in range.
    fn at(&self, i: &Self::Position) -> Self::Element;
}
