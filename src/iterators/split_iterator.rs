// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use lender::{Lender, Lending};

use crate::{
    Collection, CollectionExt, ReorderableCollection, Slice, SliceMut,
};

/// An iterator of slices which are separated by elements that match `predicate`.
pub struct SplitIterator<'a, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool,
{
    /// Base collection.
    base_collection: &'a C::Whole,

    /// Position of start of remaining elements.
    from: C::Position,

    /// Position of end of remaining elements.
    to: C::Position,

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
            base_collection: slice.whole(),
            from: slice.from,
            to: slice.to,
            predicate,
        }
    }
}

impl<'a, C, Pred> Lending<'a> for SplitIterator<'_, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool,
{
    type Lend = Slice<'a, C::Whole>;
}

impl<C, Pred> Lender for SplitIterator<'_, C, Pred>
where
    C: Collection,
    Pred: FnMut(&C::Element) -> bool + Clone,
{
    fn next(&mut self) -> Option<lender::Lend<'_, Self>> {
        if self.from == self.to {
            return None;
        }
        let mut next_from = self
            .base_collection
            .slice(self.from.clone(), self.to.clone())
            .first_position_where(self.predicate.clone());
        let res = self
            .base_collection
            .slice(self.from.clone(), next_from.clone());
        if next_from != self.to {
            self.base_collection.form_next(&mut next_from);
        }
        self.from = next_from;
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
    /// Base collection.
    base_collection: &'a mut C::Whole,

    /// Position of start of remaining elements.
    from: C::Position,

    /// Position of end of remaining elements.
    to: C::Position,

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
            base_collection: slice.whole(),
            from: slice.from,
            to: slice.to,
            predicate,
        }
    }
}

impl<'a, C, Pred> Lending<'a> for SplitIteratorMut<'_, C, Pred>
where
    C: ReorderableCollection,
    C::Whole: ReorderableCollection,
    Pred: FnMut(&C::Element) -> bool,
{
    type Lend = SliceMut<'a, C::Whole>;
}

impl<C, Pred> Lender for SplitIteratorMut<'_, C, Pred>
where
    C: ReorderableCollection,
    C::Whole: ReorderableCollection,
    Pred: FnMut(&C::Element) -> bool + Clone,
{
    fn next(&mut self) -> Option<lender::Lend<'_, Self>> {
        if self.from == self.to {
            return None;
        }
        let p = self
            .base_collection
            .slice(self.from.clone(), self.to.clone())
            .first_position_where(self.predicate.clone());
        let next_from = if p != self.to {
            self.base_collection.next(p.clone())
        } else {
            p.clone()
        };
        let res = self.base_collection.slice_mut(self.from.clone(), p);
        self.from = next_from;
        Some(res)
    }
}
