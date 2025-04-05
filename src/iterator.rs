// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// Models a single pass sequence, whose current element can be peeked.
pub trait PeekableIterator: Iterator {
    /// Returns reference to current item of iterator.
    fn peek(&self) -> Option<&Self::Item>;
}

/// Models a factory of PeekableIterator
pub trait IntoPeekableIterator: IntoIterator {
    /// Returns peekable iterator by consuming self.
    fn into_peekable_iter(self) -> Self::IntoIter;
}
