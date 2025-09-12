// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection, RandomAccessCollection,
};

/// A contiguous sub-collection of a collection.
#[derive(PartialEq, Eq)]
pub struct Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Reference to the whole collection.
    _whole: &'a Whole,

    /// Start position of slice.
    pub from: Whole::Position,

    /// End position of slice.
    pub to: Whole::Position,
}

// Base accessor algorithms.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Creates a new instance of slice with given collection and position range.
    pub fn new(
        collection: &'a Whole,
        from: Whole::Position,
        to: Whole::Position,
    ) -> Self {
        Self {
            _whole: collection,
            from,
            to,
        }
    }

    /// Returns the reference to whole collection.
    pub fn whole(&self) -> &'a Whole {
        self._whole
    }

    /// Access element at position i.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    ///   - O(1)
    pub fn at(&self, i: &Whole::Position) -> Whole::ElementRef<'a> {
        self.assert_bounds_check_read(i);
        self._whole.at(i)
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

/// Splitting algorithms.
impl<Whole> Slice<'_, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Splits slice into 2 parts where first part would have `[from, position)`
    /// and second part would have `[position, to)`.
    pub fn split_at(self, position: Whole::Position) -> (Self, Self) {
        self.assert_bounds_check_slice(&position);
        let prefix = Self {
            _whole: self._whole,
            from: self.from.clone(),
            to: position.clone(),
        };
        let suffix = Self {
            _whole: self._whole,
            from: position,
            to: self.to.clone(),
        };
        (prefix, suffix)
    }

    /// Trims the prefix of slice upto given `position` and returns the prefix.
    pub fn trim_prefix_upto(&mut self, position: Whole::Position) -> Self {
        self.assert_bounds_check_slice(&position);
        let prefix = Self {
            _whole: self._whole,
            from: self.from.clone(),
            to: position.clone(),
        };
        self.from = position;
        prefix
    }

    /// Splits slice into 2 parts where first part would have `[from, position]`
    /// and second part would have `[next(position), to)`.
    ///
    /// # Precondition
    ///   - `position != self.end()`.
    pub fn split_after(self, mut position: Whole::Position) -> (Self, Self) {
        self.form_next(&mut position);
        self.split_at(position)
    }

    /// Trims the prefix of slice through given `position`(inclusive) and returns the prefix.
    ///
    /// # Precondition
    ///   - `position != self.end()`.
    pub fn trim_prefix_through(
        &mut self,
        mut position: Whole::Position,
    ) -> Self {
        self.form_next(&mut position);
        self.trim_prefix_upto(position)
    }
}

/// Shrinking Algorithms.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: Collection<Whole = Whole>,
{
    /// Removes and returns the first element and its position if non-empty; returns None otherwise.
    pub fn pop_first_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::ElementRef<'a>)> {
        if self.from == self.to {
            None
        } else {
            let e = self._whole.at(&self.from);
            let p = self.from.clone();
            self._whole.form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the first element if non-empty; returns None otherwise.
    pub fn pop_first(&mut self) -> Option<Whole::ElementRef<'a>> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self._whole.at(&self.from));
            self._whole.form_next(&mut self.from);
            e
        }
    }

    /// Removes the first element if non-empty and returns true; returns false otherwise.
    pub fn drop_first(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self._whole.form_next(&mut self.from);
            true
        }
    }
}

/// Shrinking Algorithms for BidirectionalCollection.
impl<'a, Whole> Slice<'a, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole>,
{
    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    pub fn pop_last_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::ElementRef<'a>)> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = self._whole.at(&ele_pos);
            self._whole.form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    pub fn pop_last(&mut self) -> Option<<Self as Collection>::ElementRef<'a>> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = Some(self._whole.at(&ele_pos));
            self._whole.form_prior(&mut self.to);
            e
        }
    }

    /// Removes the last element if non-empty and returns true; returns false otherwise.
    pub fn drop_last(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self._whole.form_prior(&mut self.to);
            true
        }
    }
}

/// Shrinking Algorithms for LazyCollection.
impl<Whole> Slice<'_, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    /// Removes and returns the first element and its position if non-empty; returns None otherwise.
    pub fn lazy_pop_first_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::Element)> {
        if self.from == self.to {
            None
        } else {
            let e = self._whole.compute_at(&self.from);
            let p = self.from.clone();
            self._whole.form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the "lazily computed" first element if non-empty; returns None otherwise.
    pub fn lazy_pop_first(&mut self) -> Option<Whole::Element> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self._whole.compute_at(&self.from));
            self._whole.form_next(&mut self.from);
            e
        }
    }
}

/// Shrinking Algorithms for BidirectionalCollection + LazyCollection.
impl<Whole> Slice<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole> + LazyCollection,
{
    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    pub fn lazy_pop_last_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::Element)> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = self._whole.compute_at(&ele_pos);
            self._whole.form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    pub fn lazy_pop_last(&mut self) -> Option<Whole::Element> {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self._whole.prior(self.to.clone());
            let e = Some(self._whole.compute_at(&ele_pos));
            self._whole.form_prior(&mut self.to);
            e
        }
    }
}

impl<Whole> Collection for Slice<'_, Whole>
where
    Whole: Collection<Whole = Whole>,
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
        self._whole.form_next(i);
    }

    fn form_next_n(&self, i: &mut Self::Position, n: usize) {
        self._whole.form_next_n(i, n);
    }

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self._whole.form_next_n_limited_by(position, n, limit)
    }

    fn next(&self, i: Self::Position) -> Self::Position {
        self._whole.next(i)
    }

    fn next_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self._whole.next_n(i, n)
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        self._whole.distance(from, to)
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        self.assert_bounds_check_read(i);
        self._whole.at(i)
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::Whole> {
        self.assert_bounds_check_slice(&from);
        self.assert_bounds_check_slice(&to);
        Slice::new(self._whole, from, to)
    }
}

impl<Whole> LazyCollection for Slice<'_, Whole>
where
    Whole: LazyCollection<Whole = Whole>,
{
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.assert_bounds_check_read(i);
        self._whole.compute_at(i)
    }
}

impl<Whole> BidirectionalCollection for Slice<'_, Whole>
where
    Whole: BidirectionalCollection<Whole = Whole>,
{
    fn form_prior(&self, i: &mut Self::Position) {
        self._whole.form_prior(i);
    }

    fn form_prior_n(&self, i: &mut Self::Position, n: usize) {
        self._whole.form_prior_n(i, n);
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        self._whole.form_prior_n_limited_by(position, n, limit)
    }

    fn prior(&self, i: Self::Position) -> Self::Position {
        self._whole.prior(i)
    }

    fn prior_n(&self, i: Self::Position, n: usize) -> Self::Position {
        self._whole.prior_n(i, n)
    }
}

impl<Whole> RandomAccessCollection for Slice<'_, Whole> where
    Whole: RandomAccessCollection<Whole = Whole>
{
}
