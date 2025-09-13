// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    Collection, CollectionExt, ReorderableCollection, Slice, SliceMut,
};

/// An iterator of slices which are separated by elements that match `predicate`.
pub struct SplitIterator<'a, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool,
{
    /// Rest of collection.
    rest: Slice<'a, C::Whole>,

    /// Predicate upon which splitting would be done.
    predicate: Pred,
}

impl<'a, C, Pred> SplitIterator<'a, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool,
{
    pub(crate) fn new(slice: Slice<'a, C::Whole>, predicate: Pred) -> Self {
        SplitIterator {
            rest: slice,
            predicate,
        }
    }
}

impl<'a, C, Pred> Iterator for SplitIterator<'a, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool + Clone,
{
    type Item = Slice<'a, C::Whole>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let p = self.rest.first_position_where(self.predicate.clone());
        let res = self.rest.pop_prefix_upto(p);
        self.rest.drop_first();
        Some(res)
    }
}

/// An iterator of mutable slices which are separated by elements that match `predicate`.
pub struct SplitIteratorMut<'a, C, Pred>
where
    C: ReorderableCollection,
    C::Whole: ReorderableCollection,
    Pred: FnMut(&C::Element) -> bool,
{
    /// Rest of collection.
    rest: SliceMut<'a, C::Whole>,

    /// Predicate upon which splitting would be done.
    predicate: Pred,
}

impl<'a, C, Pred> SplitIteratorMut<'a, C, Pred>
where
    C: ReorderableCollection,
    C::Whole: ReorderableCollection,
    Pred: FnMut(&C::Element) -> bool,
{
    pub(crate) fn new(slice: SliceMut<'a, C::Whole>, predicate: Pred) -> Self {
        SplitIteratorMut {
            rest: slice,
            predicate,
        }
    }
}

impl<'a, C, Pred> Iterator for SplitIteratorMut<'a, C, Pred>
where
    C: ReorderableCollection,
    C::Whole: ReorderableCollection,
    Pred: FnMut(&C::Element) -> bool + Clone,
{
    type Item = SliceMut<'a, C::Whole>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let p = self.rest.first_position_where(self.predicate.clone());
        let res = self.rest.pop_prefix_upto(p);
        self.rest.drop_first();
        Some(res)
    }
}
