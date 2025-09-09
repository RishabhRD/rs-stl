// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::{
    BidirectionalCollection, Collection, LazyCollection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice,
};

/// A contiguous mutable sub-collection of a mutable collection.
#[derive(PartialEq, Eq)]
pub struct SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Pointer to the whole collection.
    _whole: *mut Whole,

    /// Phantom data to bind the lifetime to struct.
    _phantom: PhantomData<&'a mut Whole>,

    /// Start position of slice.
    pub from: Whole::Position,

    /// End position of slice.
    pub to: Whole::Position,
}

impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Creates a new instance of slice with given collection and position range.
    pub fn new(
        collection: &'a mut Whole,
        from: Whole::Position,
        to: Whole::Position,
    ) -> Self {
        Self {
            _whole: collection as *mut Whole,
            _phantom: PhantomData,
            from,
            to,
        }
    }

    /// Returns the mutable reference to whole collection.
    pub fn whole(&self) -> &'a mut Whole {
        unsafe { &mut *self._whole }
    }

    /// Splits slice into 2 parts where first part would have `[from, position)`
    /// and second part would have `[position, to)`.
    pub fn split_at(
        self,
        position: Whole::Position,
    ) -> (Slice<'a, Whole>, Slice<'a, Whole>) {
        self.assert_bounds_check_slice(&position);
        let whole = self.whole();
        let left = Slice::new(whole, self.from, position.clone());
        let right = Slice::new(whole, position, self.to);
        (left, right)
    }

    /// Splits slice into 2 mutable parts where first part would have `[from, position)`
    /// and second part would have `[position, to)`.
    pub fn split_at_mut(self, position: Whole::Position) -> (Self, Self) {
        self.assert_bounds_check_slice(&position);
        let left = Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: self.from.clone(),
            to: position.clone(),
        };
        let right = Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: position,
            to: self.to.clone(),
        };
        (left, right)
    }

    /// Removes and returns the first element and its position if non-empty; returns None otherwise.
    pub fn pop_first_with_pos(
        &mut self,
    ) -> Option<(
        <Self as Collection>::Position,
        <Self as Collection>::ElementRef<'_>,
    )> {
        if self.from == self.to {
            None
        } else {
            let e = self.whole().at(&self.from);
            let p = self.from.clone();
            self.whole().form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the first element if non-empty; returns None otherwise.
    pub fn pop_first(
        &mut self,
    ) -> Option<<Self as Collection>::ElementRef<'_>> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self.whole().at(&self.from));
            self.whole().form_next(&mut self.from);
            e
        }
    }

    /// Removes and returns the mutable reference to first element and its position if non-empty; returns None otherwise.
    pub fn pop_first_with_pos_mut(
        &mut self,
    ) -> Option<(<Self as Collection>::Position, &mut Whole::Element)>
    where
        Whole: MutableCollection,
    {
        if self.from == self.to {
            None
        } else {
            let p = self.from.clone();
            self.whole().form_next(&mut self.from);
            let e = self.whole().at_mut(&p);
            Some((p, e))
        }
    }

    /// Removes and returns the mutable reference to first element if non-empty; returns None otherwise.
    pub fn pop_first_mut(&mut self) -> Option<&mut Whole::Element>
    where
        Whole: MutableCollection,
    {
        if self.from == self.to {
            None
        } else {
            let p = self.from.clone();
            self.whole().form_next(&mut self.from);
            let e = Some(self.whole().at_mut(&p));
            e
        }
    }

    /// Removes the first element if non-empty and returns true; returns false otherwise.
    pub fn drop_first(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self.whole().form_next(&mut self.from);
            true
        }
    }

    /// Panics if position is out of bounds of slice for reading element.
    fn assert_bounds_check_read(&self, position: &Whole::Position) {
        if *position < self.from || *position >= self.to {
            panic!("Out of bounds read to slice.");
        }
    }

    /// Panics if position is out of bounds of slice for defining sub-slice.
    fn assert_bounds_check_slice(&self, position: &Whole::Position) {
        if *position < self.from || *position > self.to {
            panic!("Out of bounds slicing to slice.");
        }
    }
}

impl<Whole> SliceMut<'_, Whole>
where
    Whole: LazyCollection<Whole = Whole> + ReorderableCollection,
{
    /// Removes and returns the "lazily computed" first element if non-empty; returns None otherwise.
    pub fn lazy_pop_first(&mut self) -> Option<<Self as Collection>::Element> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self.whole().compute_at(&self.from));
            self.whole().form_next(&mut self.from);
            e
        }
    }

    /// Removes and returns the first element and its position if non-empty; returns None otherwise.
    pub fn lazy_pop_first_with_pos(
        &mut self,
    ) -> Option<(
        <Self as Collection>::Position,
        <Self as Collection>::Element,
    )> {
        if self.from == self.to {
            None
        } else {
            let e = self.whole().compute_at(&self.from);
            let p = self.from.clone();
            self.whole().form_next(&mut self.from);
            Some((p, e))
        }
    }
}

impl<Whole> SliceMut<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole> + ReorderableCollection,
{
    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    pub fn pop_last_with_pos(
        &mut self,
    ) -> Option<(
        <Self as Collection>::Position,
        <Self as Collection>::ElementRef<'_>,
    )> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = self.whole().at(&ele_pos);
            self.whole().form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    pub fn pop_last(&mut self) -> Option<<Self as Collection>::ElementRef<'_>> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = Some(self.whole().at(&ele_pos));
            self.whole().form_prior(&mut self.to);
            e
        }
    }

    /// Removes the last element if non-empty and returns true; returns false otherwise.
    pub fn drop_last(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self.whole().form_prior(&mut self.to);
            true
        }
    }
}

impl<Whole> SliceMut<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole> + MutableCollection,
{
    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    pub fn pop_last_with_pos_mut(
        &mut self,
    ) -> Option<(<Self as Collection>::Position, &'_ mut Whole::Element)> {
        if self.from == self.to {
            None
        } else {
            self.whole().form_prior(&mut self.to);
            let e = self.whole().at_mut(&self.to);
            Some((self.to.clone(), e))
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    pub fn pop_last_mut(&mut self) -> Option<&'_ mut Whole::Element> {
        if self.from == self.to {
            None
        } else {
            self.whole().form_prior(&mut self.to);
            let e = Some(self.whole().at_mut(&self.to));
            e
        }
    }
}

impl<Whole> SliceMut<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole>
        + LazyCollection
        + ReorderableCollection,
{
    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    pub fn lazy_pop_last_with_pos(
        &mut self,
    ) -> Option<(
        <Self as Collection>::Position,
        <Self as Collection>::Element,
    )> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = self.whole().compute_at(&ele_pos);
            self.whole().form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    pub fn lazy_pop_last(&mut self) -> Option<<Self as Collection>::Element> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = Some(self.whole().compute_at(&ele_pos));
            self.whole().form_prior(&mut self.to);
            e
        }
    }
}

impl<Whole> Collection for SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    type Position = Whole::Position;

    type Element = Whole::Element;

    type ElementRef<'a>
        = Whole::ElementRef<'a>
    where
        Self: 'a;

    type Whole = Whole;

    fn start(&self) -> Self::Position {
        self.from.clone()
    }

    fn end(&self) -> Self::Position {
        self.to.clone()
    }

    fn form_next(&self, i: &mut Self::Position) {
        self.whole().form_next(i);
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        self.whole().form_next_n(i, n);
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.whole().form_next_n_limited_by(position, n, limit)
    }

    fn next(&self, i: Self::Position) -> Self::Position {
        self.whole().next(i)
    }

    fn next_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole().next_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self.whole().distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        self.assert_bounds_check_read(i);
        self.whole().at(i)
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole> {
        self.assert_bounds_check_slice(&from);
        self.assert_bounds_check_slice(&to);
        Slice::new(self.whole(), from, to)
    }
}

impl<Whole> LazyCollection for SliceMut<'_, Whole>
where
    Whole: LazyCollection<Whole = Whole> + ReorderableCollection,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.assert_bounds_check_read(i);
        self.whole().compute_at(i)
    }
}

impl<Whole> BidirectionalCollection for SliceMut<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole> + ReorderableCollection,
{
    fn form_prior(&self, i: &mut Self::Position) {
        self.whole().form_prior(i);
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        self.whole().form_prior_n(i, n);
    }

    fn prior(&self, i: Self::Position) -> Self::Position {
        self.whole().prior(i)
    }

    fn prior_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self.whole().prior_n(i, n)
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self.whole().form_prior_n_limited_by(position, n, limit)
    }
}

impl<Whole> RandomAccessCollection for SliceMut<'_, Whole> where
    Whole: RandomAccessCollection<Whole = Whole> + ReorderableCollection
{
}

impl<Whole> ReorderableCollection for SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.assert_bounds_check_read(i);
        self.assert_bounds_check_read(j);
        self.whole().swap_at(i, j);
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> SliceMut<'_, Self::Whole> {
        self.assert_bounds_check_slice(&from);
        self.assert_bounds_check_slice(&to);
        SliceMut::new(self.whole(), from, to)
    }
}

impl<Whole> MutableCollection for SliceMut<'_, Whole>
where
    Whole: MutableCollection<Whole = Whole>,
{
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        self.assert_bounds_check_read(i);
        self.whole().at_mut(i)
    }
}
