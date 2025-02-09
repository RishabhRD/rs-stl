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
    /// Type of position in the collection.
    type Position: Regular;

    /// Type of element access in the collection.
    type Element<'this>
    where
        Self: 'this;

    /// Type of element mutable access in the collection.
    type MutableElement<'this>
    where
        Self: 'this;

    /// Type of subrange of the collection.
    type Slice<'this>: Collection<
        Position = Self::Position,
        Element<'this> = Self::Element<'this>,
        MutableElement<'this> = Self::Element<'this>,
        Slice<'this> = Self::Slice<'this>,
        MutableSlice<'this> = Self::Slice<'this>,
    >
    where
        Self: 'this;

    /// Type of mutable subrange of the collection.
    type MutableSlice<'this>: Collection<
        Position = Self::Position,
        Element<'this> = Self::Element<'this>,
        MutableElement<'this> = Self::MutableElement<'this>,
        Slice<'this> = Self::Slice<'this>,
        MutableSlice<'this> = Self::MutableSlice<'this>,
    >
    where
        Self: 'this;

    /// Returns the position of first element in the collection.
    ///
    /// If collection is empty, returns end position of the collection.
    fn start(&self) -> Self::Position;

    /// Returns position past the last element of the collection.
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

    /// calls `callback` with slice of collection in range `[from, to)`.
    ///
    /// # Precondition
    ///   - `[from, to)` represents valid positions in collection.
    fn slice<'a, Callback, ReturnType>(
        &'a self,
        from: Self::Position,
        to: Self::Position,
        callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(&Self::Slice<'a>) -> ReturnType,
        Self: 'a;

    /// calls `callback` with ith element of collection in range.
    ///
    /// # Precondition
    ///   - `i != end()`
    fn at<'a, Callback, ReturnType>(
        &'a self,
        i: &Self::Position,
        callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(Self::Element<'a>) -> ReturnType,
        Self: 'a;
}

/// Models a collection that supports backward traversal.
pub trait BidirectionalCollection: Collection
where
    for<'a> Self::Slice<'a>: BidirectionalCollection,
    for<'a> Self::MutableSlice<'a>: BidirectionalCollection,
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

/// Models a collection where jumping from one position to another is O(1) operation.
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
pub trait RandomAccessCollection:
    BidirectionalCollection<Position: Ord>
where
    for<'a> Self::Slice<'a>: RandomAccessCollection,
    for<'a> Self::MutableSlice<'a>: RandomAccessCollection,
{
}

/// Models a collection which supports reordering of its elements.
pub trait ReorderableCollection: Collection
where
    for<'a> Self::MutableSlice<'a>: ReorderableCollection,
{
    /// calls `callback` with mutable slice of collection in range `[from, to)`.
    ///
    /// # Precondition
    ///   - `[from, to)` represents valid positions in collection.
    fn slice_mut<'a, Callback, ReturnType>(
        &mut self,
        from: Self::Position,
        to: Self::Position,
        callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(&mut Self::MutableSlice<'a>) -> ReturnType,
        Self: 'a;

    /// Swaps element at position `i` with element at position `j`.
    ///
    /// # Precondition
    ///   - `i != end() && j != end()`
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position);
}

/// Models a collection which supports mutating its elements.
pub trait MutableCollection: ReorderableCollection
where
    for<'a> Self::MutableSlice<'a>: MutableCollection,
{
    /// calls `callback` with ith element with mutable access of collection in range.
    ///
    /// # Precondition
    ///   - `i != end()`
    fn at_mut<'a, Callback, ReturnType>(
        &mut self,
        i: &Self::Position,
        callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(Self::MutableElement<'a>) -> ReturnType,
        Self: 'a;
}
