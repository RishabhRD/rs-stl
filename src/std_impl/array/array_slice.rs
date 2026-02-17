// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, RandomAccessCollection, Slice,
    UnsafeSubSequence,
};

/// Unsafe slice for array-like data structures.
pub struct ArraySlice<T> {
    /// Start address of array.
    start_address: *const T,

    /// Start position of slice.
    start_position: usize,

    /// End position of slice.
    end_position: usize,
}

impl<T> ArraySlice<T> {
    pub(in crate::std_impl::array) fn new(
        start_address: *const T,
        start_position: usize,
        end_position: usize,
    ) -> Self {
        ArraySlice {
            start_address,
            start_position,
            end_position,
        }
    }
}

impl<T> Collection for ArraySlice<T> {
    type Position = usize;

    type Element = T;

    type ElementRef<'a>
        = &'a T
    where
        Self: 'a;

    type SubSequence = ArraySlice<T>;

    type MutableSubSequence = ArraySlice<T>;

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

impl<T> BidirectionalCollection for ArraySlice<T> {
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

impl<T> RandomAccessCollection for ArraySlice<T> {}

impl<T> UnsafeSubSequence for ArraySlice<T> {
    unsafe fn unsafe_at<'a>(&self, i: &Self::Position) -> Self::ElementRef<'a> {
        assert!(*i >= self.start_position && *i < self.end_position);
        unsafe { self.start_address.add(*i).as_ref_unchecked() }
    }

    unsafe fn unsafe_slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubSequence {
        assert!(to >= from);
        assert!(from >= self.start_position && from <= self.end_position);
        assert!(to >= self.start_position && to <= self.end_position);
        ArraySlice {
            start_address: self.start_address,
            start_position: from,
            end_position: to,
        }
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
