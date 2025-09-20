// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::{
    iterators::SplitEvenlyIteratorMut, BidirectionalCollection, Collection,
    CollectionExt, LazyCollection, MutableCollection, RandomAccessCollection,
    ReorderableCollection, Slice,
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

/// Base accessor algorithms.
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

    /// Access element at position i.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    ///   - O(1)
    pub fn at(&self, i: &Whole::Position) -> Whole::ElementRef<'a> {
        self.assert_bounds_check_read(i);
        self.whole().at(i)
    }

    /// Mutably Access element at position i.
    ///
    /// # Precondition
    ///   - i is a valid position in self and i != end()
    ///
    /// # Complexity Requirement
    ///   - O(1)
    pub fn at_mut(&mut self, i: &Whole::Position) -> &'a mut Whole::Element
    where
        Whole: MutableCollection,
    {
        self.assert_bounds_check_read(i);
        self.whole().at_mut(i)
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

/// Dropping algorithms
impl<Whole> SliceMut<'_, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Removes the first element if non-empty and returns true; returns false otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// assert!(s.drop_first());
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_first(&mut self) -> bool {
        if self.from == self.to {
            false
        } else {
            self.whole().form_next(&mut self.from);
            true
        }
    }

    /// Removes the last element if non-empty and returns true; returns false otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// assert!(s.drop_last());
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn drop_last(&mut self) -> bool
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            false
        } else {
            self.whole().form_prior(&mut self.to);
            true
        }
    }

    /// Drops prefix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection;
    ///   - O(n) otherwise, where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// s.drop_prefix(2);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_prefix(&mut self, max_length: usize) {
        let mut new_from = self.from.clone();
        self.whole().form_next_n_limited_by(
            &mut new_from,
            max_length,
            self.to.clone(),
        );
        self.from = new_from;
    }

    /// Drops the prefix of slice upto given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// s.drop_prefix_upto(2);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_prefix_upto(&mut self, position: Whole::Position) {
        self.assert_bounds_check_slice(&position);
        self.from = position;
    }

    /// Drops the prefix of slice till and including given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///   - `position != self.end()`
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// s.drop_prefix_through(2);
    /// assert!(s.equals(&[3]));
    /// ```
    pub fn drop_prefix_through(&mut self, position: Whole::Position) {
        self.assert_bounds_check_read(&position);
        self.from = self.whole().next(position);
    }

    /// Drops the element of `self` till the first element satisfies `predicate`.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// s.drop_while(|x| *x < 2);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn drop_while<Pred>(&mut self, mut predicate: Pred)
    where
        Pred: FnMut(&Whole::Element) -> bool,
    {
        self.from = self.first_position_where(|e| !predicate(e));
    }

    /// Drops suffix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// s.drop_suffix(2);
    /// assert!(s.equals(&[0, 1]));
    /// ```
    pub fn drop_suffix(&mut self, max_length: usize) {
        let n = self.count();
        if max_length > n {
            self.to = self.from.clone()
        } else {
            self.to = self.next_n(self.start(), n - max_length)
        }
    }

    /// Drops suffix from given `position`.
    ///
    /// # Precondition
    ///   - `position` is a valid position in self.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// s.drop_suffix_from(2);
    /// assert!(s.equals(&[0, 1]));
    /// ```
    pub fn drop_suffix_from(&mut self, position: Whole::Position) {
        self.assert_bounds_check_slice(&position);
        self.to = position;
    }
}

/// Pop algorithms.
impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Removes and returns the first element if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let first = s.pop_first().unwrap();
    /// assert_eq!(*first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_first(&mut self) -> Option<Whole::ElementRef<'a>> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self.whole().at(&self.from));
            self.whole().form_next(&mut self.from);
            e
        }
    }

    /// Removes and returns the mutable reference to first element if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let first = s.pop_first_mut().unwrap();
    /// assert_eq!(*first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_first_mut(&mut self) -> Option<&'a mut Whole::Element>
    where
        Whole: MutableCollection,
    {
        if self.from == self.to {
            None
        } else {
            let e = Some(self.whole().at_mut(&self.from));
            self.whole().form_next(&mut self.from);
            e
        }
    }

    /// Removes and returns the first element and its position if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let (first_pos, first) = s.pop_first_with_pos().unwrap();
    /// assert_eq!(first_pos, 0);
    /// assert_eq!(*first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_first_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::ElementRef<'a>)> {
        if self.from == self.to {
            None
        } else {
            let e = self.whole().at(&self.from);
            let p = self.from.clone();
            self.whole().form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the first element and its position if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let (first_pos, first) = s.pop_first_with_pos_mut().unwrap();
    /// assert_eq!(first_pos, 0);
    /// assert_eq!(*first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_first_with_pos_mut(
        &mut self,
    ) -> Option<(Whole::Position, &'a mut Whole::Element)>
    where
        Whole: MutableCollection,
    {
        if self.from == self.to {
            None
        } else {
            let e = self.whole().at_mut(&self.from);
            let p = self.from.clone();
            self.whole().form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let last = s.pop_last().unwrap();
    /// assert_eq!(*last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn pop_last(&mut self) -> Option<Whole::ElementRef<'a>>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = Some(self.whole().at(&ele_pos));
            self.whole().form_prior(&mut self.to);
            e
        }
    }

    /// Removes and returns the last element if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let last = s.pop_last_mut().unwrap();
    /// assert_eq!(*last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn pop_last_mut(&mut self) -> Option<&'a mut Whole::Element>
    where
        Whole: BidirectionalCollection + MutableCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = Some(self.whole().at_mut(&ele_pos));
            self.whole().form_prior(&mut self.to);
            e
        }
    }

    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let (last_pos, last) = s.pop_last_with_pos().unwrap();
    /// assert_eq!(last_pos, 2);
    /// assert_eq!(*last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn pop_last_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::ElementRef<'a>)>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = self.whole().at(&ele_pos);
            self.whole().form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }

    /// Removes and returns the last element and its position if non-empty; returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let (last_pos, last) = s.pop_last_with_pos_mut().unwrap();
    /// assert_eq!(last_pos, 2);
    /// assert_eq!(*last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn pop_last_with_pos_mut(
        &mut self,
    ) -> Option<(Whole::Position, &'a mut Whole::Element)>
    where
        Whole: BidirectionalCollection + MutableCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = self.whole().at_mut(&ele_pos);
            self.whole().form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }

    /// Removes and returns prefix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty and return the full_mut slice as result.
    ///
    /// # Complexity
    ///   - O(1) for RandomAccessCollection;
    ///   - O(n) otherwise, where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let prefix = s.pop_prefix(2);
    /// assert!(prefix.equals(&[0, 1]));
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_prefix(&mut self, max_length: usize) -> Self {
        let old_from = self.from.clone();
        let mut new_from = self.from.clone();
        self.whole().form_next_n_limited_by(
            &mut new_from,
            max_length,
            self.to.clone(),
        );
        self.from = new_from;
        Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: old_from,
            to: self.from.clone(),
        }
    }

    /// Removes and returns the prefix slice upto given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let prefix = s.pop_prefix_upto(2);
    /// assert!(prefix.equals(&[0, 1]));
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_prefix_upto(&mut self, position: Whole::Position) -> Self {
        self.assert_bounds_check_slice(&position);
        let prefix = Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: self.from.clone(),
            to: position.clone(),
        };
        self.from = position;
        prefix
    }

    /// Removes and returns the prefix till and including given `position`.
    ///
    /// # Precondition
    ///   - `position` is valid position in `self`.
    ///   - `position != self.end()`
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let prefix = s.pop_prefix_through(2);
    /// assert!(prefix.equals(&[0, 1, 2]));
    /// assert!(s.equals(&[3]));
    /// ```
    pub fn pop_prefix_through(&mut self, position: Whole::Position) -> Self {
        self.assert_bounds_check_read(&position);
        let old_from = self.from.clone();
        self.from = self.whole().next(position);
        Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: old_from,
            to: self.from.clone(),
        }
    }

    /// Removes and returns  the element of `self` till the first element satisfies `predicate` as a slice.
    ///
    /// # Complexity
    ///   - O(n) where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let prefix = s.pop_while(|x| *x < 2);
    /// assert!(prefix.equals(&[0, 1]));
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn pop_while<Pred>(&mut self, mut predicate: Pred) -> Self
    where
        Pred: FnMut(&Whole::Element) -> bool,
    {
        let p = self.first_position_where(|e| !predicate(e));
        let res = SliceMut {
            _whole: self._whole,
            _phantom: PhantomData,
            from: self.from.clone(),
            to: p.clone(),
        };
        self.from = p;
        res
    }

    /// Removes and returns suffix upto specified maximum length.
    ///
    /// # Postcondition
    ///   - If `max_length > self.count()`, make `self` empty and returns the
    ///     full_mut slice as suffix.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let suffix = s.pop_suffix(2);
    /// assert!(s.equals(&[0, 1]));
    /// assert!(suffix.equals(&[2, 3]));
    /// ```
    pub fn pop_suffix(&mut self, max_length: usize) -> Self {
        let n = self.count();
        let old_to = self.to.clone();
        if max_length > n {
            self.to = self.from.clone()
        } else {
            self.to = self.next_n(self.start(), n - max_length)
        }
        Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: self.to.clone(),
            to: old_to,
        }
    }

    /// Removes and returns suffix from given `position`.
    ///
    /// # Precondition
    ///   - `position` is a valid position in self.
    ///
    /// # Complexity
    ///   - O(n), where `n == self.count()`.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3];
    /// let mut s = arr.full_mut();
    /// let suffix = s.pop_suffix_from(2);
    /// assert!(s.equals(&[0, 1]));
    /// assert!(suffix.equals(&[2, 3]));
    /// ```
    pub fn pop_suffix_from(&mut self, position: Whole::Position) -> Self {
        self.assert_bounds_check_slice(&position);
        let old_to = self.to.clone();
        self.to = position;
        Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: self.to.clone(),
            to: old_to,
        }
    }
}

/// Pop algorithms for `LazyCollection`.
impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: LazyCollection<Whole = Whole> + ReorderableCollection,
{
    /// Removes and returns the lazily computed first element if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = CollectionExt::map([1, 2, 3], |x| *x);
    /// let mut s = arr.full_mut();
    /// let first = s.lazy_pop_first().unwrap();
    /// assert_eq!(first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn lazy_pop_first(&mut self) -> Option<Whole::Element> {
        if self.from == self.to {
            None
        } else {
            let e = Some(self.whole().compute_at(&self.from));
            self.whole().form_next(&mut self.from);
            e
        }
    }

    /// Removes and returns the lazily computed first element and its position if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = CollectionExt::map([1, 2, 3], |x| *x);
    /// let mut s = arr.full_mut();
    /// let (first_pos, first) = s.lazy_pop_first_with_pos().unwrap();
    /// assert_eq!(first_pos, 0);
    /// assert_eq!(first, 1);
    /// assert!(s.equals(&[2, 3]));
    /// ```
    pub fn lazy_pop_first_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::Element)> {
        if self.from == self.to {
            None
        } else {
            let e = self.whole().compute_at(&self.from);
            let p = self.from.clone();
            self.whole().form_next(&mut self.from);
            Some((p, e))
        }
    }

    /// Removes and returns the lazily computed last element if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = CollectionExt::map([1, 2, 3], |x| *x);
    /// let mut s = arr.full_mut();
    /// let last = s.lazy_pop_last().unwrap();
    /// assert_eq!(last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn lazy_pop_last(&mut self) -> Option<Whole::Element>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = Some(self.whole().compute_at(&ele_pos));
            self.whole().form_prior(&mut self.to);
            e
        }
    }

    /// Removes and returns the lazily computed last element and its position if non-empty;
    /// returns None otherwise.
    ///
    /// # Example
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = CollectionExt::map([1, 2, 3], |x| *x);
    /// let mut s = arr.full_mut();
    /// let (last_pos, last) = s.lazy_pop_last_with_pos().unwrap();
    /// assert_eq!(last_pos, 2);
    /// assert_eq!(last, 3);
    /// assert!(s.equals(&[1, 2]));
    /// ```
    pub fn lazy_pop_last_with_pos(
        &mut self,
    ) -> Option<(Whole::Position, Whole::Element)>
    where
        Whole: BidirectionalCollection,
    {
        if self.from == self.to {
            None
        } else {
            let ele_pos = self.whole().prior(self.to.clone());
            let e = self.whole().compute_at(&ele_pos);
            self.whole().form_prior(&mut self.to);
            Some((ele_pos, e))
        }
    }
}

/// Splitting algorithms.
impl<'a, Whole> SliceMut<'a, Whole>
where
    Whole: ReorderableCollection<Whole = Whole>,
{
    /// Returns two disjoint mutable slices of `self` split at the given `position`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3, 4];
    /// let (s1, s2) = arr.full_mut().split_at(2);
    /// assert!(s1.equals(&[0, 1]));
    /// assert!(s2.equals(&[2, 3, 4]));
    /// ```
    pub fn split_at(self, position: Whole::Position) -> (Self, Self) {
        self.assert_bounds_check_slice(&position);
        let prefix = Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: self.from.clone(),
            to: position.clone(),
        };
        let suffix = Self {
            _whole: self._whole,
            _phantom: PhantomData,
            from: position,
            to: self.to.clone(),
        };
        (prefix, suffix)
    }

    /// Returns two disjoint mutable slices of `self`, split immediately *after* the
    /// given `position`.
    ///
    /// # Precondition
    ///   - `position != self.end()`.
    ///
    /// # Examples
    /// ```rust
    /// use stl::*;
    ///
    /// let mut arr = [0, 1, 2, 3, 4];
    /// let (s1, s2) = arr.full_mut().split_after(2);
    /// assert!(s1.equals(&[0, 1, 2]));
    /// assert!(s2.equals(&[3, 4]));
    /// ```
    pub fn split_after(self, mut position: Whole::Position) -> (Self, Self) {
        self.form_next(&mut position);
        self.split_at(position)
    }

    /// Splits `self` into at max `max_slices` mutable slices with each slice
    /// being of at min size of `min_size` by returning an Iterator of mutable
    /// slices.
    ///
    /// # Precondition
    ///   - `max_slices > 0`,
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; otherwise O(n) where `n == self.count()`.
    pub fn split_evenly_in_with_min_size(
        self,
        max_slices: usize,
        min_size: usize,
    ) -> SplitEvenlyIteratorMut<'a, Whole> {
        let n = self.count();
        let num_slices = if min_size == 0 {
            max_slices
        } else {
            usize::min(usize::max(n / min_size, 1), max_slices)
        };

        let slice_size = n / num_slices;
        let num_bigger_slices = n % num_slices;

        SplitEvenlyIteratorMut::new(
            self,
            num_slices,
            slice_size,
            num_bigger_slices,
        )
    }

    /// Splits `self` into `num_slices` mutable slices by returning an Iterator
    /// of mutable slices.
    ///
    /// # Precondition
    ///   - `num_slices > 0`,
    ///
    /// # Complexity
    ///   - O(1) for `RandomAccessCollection`; otherwise O(n) where `n == self.count()`.
    pub fn split_evenly_in(
        self,
        num_slices: usize,
    ) -> SplitEvenlyIteratorMut<'a, Whole> {
        self.split_evenly_in_with_min_size(num_slices, 0)
    }
}

unsafe impl<'a, Whole> Send for SliceMut<'a, Whole> where
    Whole: ReorderableCollection<Whole = Whole> + Send
{
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
