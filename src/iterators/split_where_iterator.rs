// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    Collection, CollectionExt, ReorderableCollection, Slice, SliceMut,
    UnsafeReorderableSubSequence,
};

/// An iterator of slices which are separated by elements that match `predicate`.
pub struct SplitWhereIterator<'a, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool,
{
    /// Rest of collection.
    rest: Slice<'a, C>,

    /// Predicate upon which splitting would be done.
    predicate: Pred,
}

impl<'a, C, Predicate> SplitWhereIterator<'a, C, Predicate>
where
    C: Collection,
    Predicate: FnMut(&C::Element) -> bool,
{
    pub(crate) fn new(slice: Slice<'a, C>, predicate: Predicate) -> Self {
        SplitWhereIterator {
            rest: slice,
            predicate,
        }
    }
}

impl<'a, C, Pred> Iterator for SplitWhereIterator<'a, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool + Clone,
{
    type Item = Slice<'a, C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let p = self
            .rest
            .first_position_where(self.predicate.clone())
            .unwrap_or(self.rest.end());
        let res = self.rest.pop_prefix_upto(p);
        self.rest.drop_first();
        Some(res)
    }
}

/// An iterator of mutable slices which are separated by elements that match `predicate`.
pub struct SplitWhereIteratorMut<'a, C, Pred>
where
    C: ReorderableCollection,
    C::MutableSubSequence: UnsafeReorderableSubSequence,
    Pred: FnMut(&C::Element) -> bool,
{
    /// Rest of collection.
    rest: SliceMut<'a, C>,

    /// Predicate upon which splitting would be done.
    predicate: Pred,
}

impl<'a, C, Pred> SplitWhereIteratorMut<'a, C, Pred>
where
    C: ReorderableCollection,
    C::MutableSubSequence: UnsafeReorderableSubSequence,
    Pred: FnMut(&C::Element) -> bool,
{
    pub(crate) fn new(slice: SliceMut<'a, C>, predicate: Pred) -> Self {
        SplitWhereIteratorMut {
            rest: slice,
            predicate,
        }
    }
}

impl<'a, C, Pred> Iterator for SplitWhereIteratorMut<'a, C, Pred>
where
    C: ReorderableCollection,
    C::MutableSubSequence: UnsafeReorderableSubSequence,
    Pred: FnMut(&C::Element) -> bool + Clone,
{
    type Item = SliceMut<'a, C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let p = self
            .rest
            .first_position_where(self.predicate.clone())
            .unwrap_or(self.rest.end());
        let res = self.rest.pop_prefix_upto(p);
        self.rest.drop_first();
        Some(res)
    }
}
