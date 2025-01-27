// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

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

use crate::{CollectionIterator, LazyCollectionIterator};

/// Defines Position and Element type of Range
///
/// # TODO
///   - This should be part of Range itself, however, that would require to use
///     GAT with HRTB. However, borrow checker current limitation results in
///     deducing the resultant lifetime of subrange to 'static.
///   - See [https://github.com/rust-lang/rust/issues/87479](https://github.com/rust-lang/rust/issues/87479)
///   - This implementation is a workaround for the same.
///     See [the better alternative to lifetime](https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats#hrtb-supertrait)
pub trait RangeBase {
    /// Type of positions in range.
    type Position: Regular;

    /// Type of element of range.
    type Element;
}

/// Defines subrange type of range
///
/// # TODO
///   - This should be part of Range itself, however, that would require to use
///     GAT with HRTB. However, borrow checker current limitation results in
///     deducing the resultant lifetime of subrange to 'static.
///   - See [https://github.com/rust-lang/rust/issues/87479](https://github.com/rust-lang/rust/issues/87479)
///   - This implementation is a workaround for the same.
///     See [the better alternative to lifetime](https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats#hrtb-supertrait)
pub trait SubRangeable<'this, ImplicitBounds: Sealed = Bounds<&'this Self>>:
    RangeBase
{
    /// Type of subrange of the range.
    type SubRange: Range<
        Element = Self::Element,
        Position = Self::Position,
        SubRange = Self::SubRange,
    >;
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
pub trait Range: for<'this> SubRangeable<'this> {
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

    /// Returns subrange `[from, to)` of given range.
    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as SubRangeable<'_>>::SubRange;

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
}

/// Models a range whose `Element`s are present in memory.
///
/// If elements are present in memory, then it should be possible to obtain
/// reference to elements.
pub trait Collection: Range
where
    for<'a> <Self as SubRangeable<'a>>::SubRange: Collection,
{
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
pub trait LazyCollection: Range
where
    for<'a> <Self as SubRangeable<'a>>::SubRange: LazyCollection,
{
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

/// AnyCollection weakens the element access with returning `reference like type` to Element.
///
/// This allows unified implementation of algorithms for Collection and LazyCollection.
pub trait AnyCollection: Range {
    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element>;
}

impl<R> AnyCollection for R
where
    R: Collection,
    for<'a> <Self as SubRangeable<'a>>::SubRange: Collection,
{
    fn at_ref(
        &self,
        i: &Self::Position,
    ) -> impl std::ops::Deref<Target = Self::Element> {
        self.at(i)
    }
}

// TODO: This is AnyCollection implementation for a LazyCollection. However,
// this leads to conflicting implementation because of Collection's implementation
// for AnyCollection. Currently, every lazy collection would need to implement
// it manually.
//
// impl<R> AnyCollection for R
// where
//     R: LazyCollection,
//     for<'a> <Self as SubRangeable<'a>>::SubRange: LazyCollection,
// {
//     fn at_ref(
//         &self,
//         i: &Self::Position,
//     ) -> impl std::ops::Deref<Target = Self::Element> {
//         crate::util::ValueRef { val: self.at(i) }
//     }
// }

/// Models a range that supports forward as well as backward traversal.
pub trait BidirectionalRange: Range
where
    for<'a> <Self as SubRangeable<'a>>::SubRange: BidirectionalRange,
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
pub trait RandomAccessRange: BidirectionalRange<Position: Ord>
where
    for<'a> <Self as SubRangeable<'a>>::SubRange: RandomAccessRange,
{
}
