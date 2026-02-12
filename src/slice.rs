// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::{BidirectionalCollection, UnsafeSubSequence};

/// A safe view into unsafe subsequence of a collection.
pub struct Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence>,
{
    /// The unsafe subsequence.
    subsequence: SubSequence,

    /// A marker data managing lifetime of `self`.
    _marker: PhantomData<&'a ()>,
}

impl<'a, SubSequence> Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence>,
{
    /// Returns a slice for unsafe subsequence `s`.
    pub unsafe fn new(s: SubSequence) -> Self {
        Slice {
            subsequence: s,
            _marker: PhantomData,
        }
    }

    /// Removes and returns first element.
    ///
    /// The removed element becomes independent to `self` and thus can be used
    /// in parallel to `self`.
    fn pop_first(&mut self) -> SubSequence::ElementRef<'a> {
        let mut i = self.subsequence.start();
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        self.subsequence.form_next(&mut i);
        unsafe { self.subsequence.set_start(i) };
        r
    }

    /// Removes and returns last element.
    ///
    /// The removed element becomes independent to `self` and thus can be used
    /// in parallel to `self`.
    fn pop_last(&mut self) -> SubSequence::ElementRef<'a>
    where
        SubSequence: BidirectionalCollection,
    {
        let i = self.subsequence.prior(self.subsequence.end());
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        unsafe { self.subsequence.set_end(i) };
        r
    }

    /// Removes and returns a subsequence of elements starting from `self.start()` upto but not
    /// including `p`.
    fn pop_prefix_upto(&mut self, p: SubSequence::Position) -> Self {
        let s = unsafe {
            self.subsequence
                .unsafe_slice(self.subsequence.start(), p.clone())
        };
        unsafe { self.subsequence.set_start(p) };
        Self::new(s)
    }
}
