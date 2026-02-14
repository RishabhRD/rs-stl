// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    ArraySlice, BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
    UnsafeMutableSubSequence, UnsafeReorderableSubSequence, UnsafeSubSequence,
};

/// Unsafe mutable slice for array-like data structures.
pub struct ArraySliceMut<T> {
    /// Start address of array.
    start_address: *mut T,

    /// Start position of slice.
    start_position: usize,

    /// End position of slice.
    end_position: usize,
}

impl<T> ArraySliceMut<T> {
    pub(in crate::std_impl::array) fn new(
        start_address: *mut T,
        start_position: usize,
        end_position: usize,
    ) -> Self {
        ArraySliceMut {
            start_address,
            start_position,
            end_position,
        }
    }
}

impl<T> Collection for ArraySliceMut<T> {
    type Position = usize;

    type Element = T;

    type ElementRef<'a>
        = &'a T
    where
        Self: 'a;

    type SubSequence = ArraySlice<T>;

    type MutableSubSequence = ArraySliceMut<T>;

    fn start(&self) -> Self::Position {
        self.start_position
    }

    fn end(&self) -> Self::Position {
        self.end_position
    }

    fn form_next(&self, p: &mut Self::Position) {
        *p += 1;
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        unsafe { self.unsafe_at(i) }
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<'_, Self::SubSequence> {
        unsafe { Slice::new(self.unsafe_slice(from, to)) }
    }

    fn form_next_n(&self, p: &mut Self::Position, n: usize) {
        *p += n;
    }

    fn form_next_n_limited_by(
        &self,
        p: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        if *p + n <= limit {
            *p += n;
            true
        } else {
            *p = limit;
            false
        }
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }
}

impl<T> BidirectionalCollection for ArraySliceMut<T> {
    fn form_prior(&self, p: &mut Self::Position) {
        *p -= 1
    }

    fn form_prior_n(&self, p: &mut Self::Position, n: usize) {
        *p -= n
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        if *position >= limit + n {
            *position -= n;
            true
        } else {
            *position = limit;
            false
        }
    }
}

impl<T> RandomAccessCollection for ArraySliceMut<T> {}

impl<T> ReorderableCollection for ArraySliceMut<T> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        assert!(*i >= self.start_position && *i < self.end_position);
        assert!(*j >= self.start_position && *j < self.end_position);

        unsafe {
            let pi = self.start_address.add(*i + self.start_position);
            let pj = self.start_address.add(*j + self.start_position);

            std::ptr::swap(pi, pj);
        }
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::SliceMut<'_, Self::MutableSubSequence> {
        unsafe { SliceMut::new(self.unsafe_slice_mut(from, to)) }
    }
}

impl<T> MutableCollection for ArraySliceMut<T> {
    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        unsafe { self.unsafe_at_mut(i) }
    }
}

impl<T> UnsafeSubSequence for ArraySliceMut<T> {
    unsafe fn unsafe_at<'a>(&self, i: &Self::Position) -> Self::ElementRef<'a> {
        assert!(*i >= self.start_position && *i < self.end_position);
        unsafe {
            self.start_address
                .add(i + self.start_position)
                .as_ref_unchecked()
        }
    }

    unsafe fn unsafe_slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubSequence {
        assert!(to >= from);
        assert!(from >= self.start_position && from <= self.end_position);
        assert!(to >= self.start_position && to <= self.end_position);
        ArraySlice::new(self.start_address, from, to)
    }

    unsafe fn set_start(&mut self, p: Self::Position) {
        assert!(p >= self.start_position && p <= self.end_position);
        self.start_position = p
    }

    unsafe fn set_end(&mut self, p: Self::Position) {
        assert!(p >= self.start_position && p <= self.end_position);
        self.end_position = p
    }
}

impl<T> UnsafeReorderableSubSequence for ArraySliceMut<T> {
    unsafe fn unsafe_slice_mut(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::MutableSubSequence {
        assert!(to >= from);
        assert!(from >= self.start_position && from <= self.end_position);
        assert!(to >= self.start_position && to <= self.end_position);
        ArraySliceMut::new(self.start_address, from, to)
    }
}

impl<T> UnsafeMutableSubSequence for ArraySliceMut<T> {
    unsafe fn unsafe_at_mut<'a>(
        &self,
        i: &Self::Position,
    ) -> &'a mut Self::Element {
        assert!(*i >= self.start_position && *i < self.end_position);
        unsafe {
            self.start_address
                .add(i + self.start_position)
                .as_mut_unchecked()
        }
    }
}
