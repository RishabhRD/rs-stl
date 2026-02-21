// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{string::util::char_len_for, value_ref::ValueRef, *};

impl Collection for String {
    type Position = usize;

    type Element = char;

    type ElementRef<'a>
        = ValueRef<char>
    where
        Self: 'a;

    type SubSequence = StringSlice;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        self.len()
    }

    fn form_next(&self, i: &mut Self::Position) {
        let len = char_len_for(self.as_bytes()[*i]);
        *i += len
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        let len = char_len_for(self.as_bytes()[*i]);
        let bytes = self.as_bytes();
        let val = unsafe { std::str::from_utf8_unchecked(&bytes[*i..i + len]) }
            .chars()
            .next()
            .unwrap();
        ValueRef { val }
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Slice<'_, Self::SubSequence> {
        let s = StringSlice::new(self.as_ptr(), from, to);
        unsafe { Slice::new(s.unsafe_slice(from, to)) }
    }
}

impl LazyCollection for String {
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.at(i).val
    }
}
