// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, DoubleEndedLendingIterator,
    LazyCollection, LendingIterator, MutableCollection, Slice, SliceMut,
};

/// An iterator to iterate over element-ref of collection.
pub struct CollectionIter<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Slice representing remaining elements to iterate.
    slice: Slice<'a, C>,
}

impl<'a, C> CollectionIter<'a, C>
where
    C: Collection<Whole = C>,
{
    /// Creates a new instance of Self with given slice.
    pub fn new(slice: Slice<'a, C>) -> Self {
        Self { slice }
    }
}

impl<C> LendingIterator for CollectionIter<'_, C>
where
    C: Collection<Whole = C>,
{
    type Item<'a>
        = C::ElementRef<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.slice.pop_first()
    }
}

impl<C> DoubleEndedLendingIterator for CollectionIter<'_, C>
where
    C: BidirectionalCollection<Whole = C>,
{
    fn next_back(&mut self) -> Option<Self::Item<'_>> {
        self.slice.pop_last()
    }
}

/// An iterator to iterate over mutable reference of elements of collection.
pub struct MutableCollectionIter<'a, C>
where
    C: MutableCollection<Whole = C>,
{
    /// Slice representing remaining elements to iterate.
    slice: SliceMut<'a, C>,
}

impl<'a, C> MutableCollectionIter<'a, C>
where
    C: MutableCollection<Whole = C>,
{
    /// Creates a new instance of Self with given slice.
    pub fn new(slice: SliceMut<'a, C>) -> Self {
        Self { slice }
    }
}

impl<C> LendingIterator for MutableCollectionIter<'_, C>
where
    C: MutableCollection<Whole = C>,
{
    type Item<'a>
        = &'a mut C::Element
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.slice.pop_first_mut()
    }
}

impl<C> DoubleEndedLendingIterator for MutableCollectionIter<'_, C>
where
    C: BidirectionalCollection<Whole = C> + MutableCollection,
{
    fn next_back(&mut self) -> Option<Self::Item<'_>> {
        self.slice.pop_last_mut()
    }
}

/// An iterator to iterate over lazily computed elements of collection.
pub struct LazyCollectionIter<'a, C>
where
    C: LazyCollection<Whole = C>,
{
    /// Slice representing remaining elements to iterate.
    slice: Slice<'a, C>,
}

impl<'a, C> LazyCollectionIter<'a, C>
where
    C: LazyCollection<Whole = C>,
{
    /// Creates a new instance of Self with given slice.
    pub fn new(slice: Slice<'a, C>) -> Self {
        Self { slice }
    }
}

impl<C> Iterator for LazyCollectionIter<'_, C>
where
    C: LazyCollection<Whole = C>,
{
    type Item = C::Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.lazy_pop_first()
    }
}

impl<C> DoubleEndedIterator for LazyCollectionIter<'_, C>
where
    C: BidirectionalCollection<Whole = C> + LazyCollection,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.slice.lazy_pop_last()
    }
}
