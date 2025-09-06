// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// An iterator whose items are part of iterator.
pub trait LendingIterator {
    /// Type of item, iterator yields.
    type Item<'a>
    where
        Self: 'a;

    /// Returns next element if any, and consumes that element.
    fn next(&mut self) -> Option<Self::Item<'_>>;

    /// Calls `f` with items of iterator until `pred` returns true for first time.
    fn for_each_until<F, Pred>(&mut self, mut f: F, mut pred: Pred)
    where
        F: FnMut(Self::Item<'_>),
        Pred: FnMut(&Self::Item<'_>) -> bool,
    {
        while let Some(e) = self.next() {
            if pred(&e) {
                f(e)
            }
        }
    }

    /// Calls `f` with every items of iterator.
    fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(Self::Item<'_>),
    {
        while let Some(e) = self.next() {
            f(e)
        }
    }
}

/// A lending iterator whose elements can be consumed from other side too.
pub trait DoubleEndedLendingIterator: LendingIterator {
    /// Returns next back element if any, and consumes that element.
    fn next_back(&mut self) -> Option<Self::Item<'_>>;
}
