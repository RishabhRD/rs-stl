// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use lender::{DoubleEndedLender, Lend, Lender, Lending};

use crate::{BidirectionalCollection, MutableCollection, SliceMut};

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

impl<'a, C> Lending<'a> for MutableCollectionIter<'_, C>
where
    C: MutableCollection<Whole = C>,
{
    type Lend = &'a mut C::Element;
}

impl<C> Lender for MutableCollectionIter<'_, C>
where
    C: MutableCollection<Whole = C>,
{
    fn next(&mut self) -> Option<Lend<'_, Self>> {
        self.slice.pop_first_mut()
    }
}

impl<C> DoubleEndedLender for MutableCollectionIter<'_, C>
where
    C: BidirectionalCollection<Whole = C> + MutableCollection,
{
    fn next_back(&mut self) -> Option<Lend<'_, Self>> {
        self.slice.pop_last_mut()
    }
}
