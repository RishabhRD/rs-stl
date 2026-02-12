// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::marker::PhantomData;

use crate::{
    iterators::{SplitEvenlyIterator, SplitWhereIterator},
    BidirectionalCollection, Collection, CollectionExt, LazyCollection,
    RandomAccessCollection, UnsafeSubSequence,
};

pub struct Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence>,
{
    subsequence: SubSequence,
    _marker: PhantomData<&'a ()>,
}

impl<'a, SubSequence> Slice<'a, SubSequence>
where
    SubSequence: UnsafeSubSequence<SubSequence = SubSequence>,
{
    pub fn new(subsequence: SubSequence) -> Self {
        Slice {
            subsequence,
            _marker: PhantomData,
        }
    }

    fn pop_first(&mut self) -> SubSequence::ElementRef<'a> {
        let mut i = self.subsequence.start();
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        self.subsequence.form_next(&mut i);
        unsafe { self.subsequence.set_start(i) };
        r
    }

    fn pop_last(&mut self) -> SubSequence::ElementRef<'a>
    where
        SubSequence: BidirectionalCollection,
    {
        let i = self.subsequence.prior(self.subsequence.end());
        let r = unsafe { self.subsequence.unsafe_at(&i) };
        unsafe { self.subsequence.set_start(i) };
        r
    }

    fn pop_prefix_upto(&mut self, p: SubSequence::Position) -> Self {
        let s = unsafe {
            self.subsequence.unsafe_slice(self.subsequence.start(), p)
        };
        Self::new(s)
    }
}
