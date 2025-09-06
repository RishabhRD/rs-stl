// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::LendingIterator;

/// Adaptor for base lending iterator that yields only the items which satisfy predicate.
pub struct FilterIter<I, F>
where
    I: LendingIterator,
    F: FnMut(&I::Item<'_>) -> bool,
{
    pub base: I,
    pub predicate: F,
}

impl<I, F> FilterIter<I, F>
where
    I: LendingIterator,
    F: FnMut(&I::Item<'_>) -> bool,
{
    /// Creates a new instance of filter adaptor.
    pub fn new(base: I, predicate: F) -> Self {
        FilterIter { base, predicate }
    }
}

impl<I, F> LendingIterator for FilterIter<I, F>
where
    I: LendingIterator,
    F: FnMut(&I::Item<'_>) -> bool,
{
    type Item<'a>
        = I::Item<'a>
    where
        Self: 'a;

    // TODO: fix the implementation when borrow checker once we have polonius.
    fn next(&mut self) -> Option<Self::Item<'_>> {
        while let Some(mut item) = self.base.next() {
            if (self.predicate)(&item) {
                let item = unsafe {
                    let ptr = &mut item as *mut Self::Item<'_>;
                    let ptr = std::mem::transmute::<
                        *mut Self::Item<'_>,
                        *mut Self::Item<'_>,
                    >(ptr);
                    let res = ptr.read();
                    core::mem::forget(item);
                    res
                };
                return Some(item);
            }
        }
        None
    }
}
