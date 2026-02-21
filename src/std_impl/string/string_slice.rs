// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{string::util::char_len_for, value_ref::ValueRef, *};

/// Unsafe slice for string.
pub struct StringSlice {
    start_address: *const u8,

    start_position: usize,

    end_position: usize,
}

impl StringSlice {
    pub(in crate::std_impl::string) fn new(
        start_address: *const u8,
        start_position: usize,
        end_position: usize,
    ) -> Self {
        StringSlice {
            start_address,
            start_position,
            end_position,
        }
    }
}

impl Collection for StringSlice {
    type Position = usize;

    type Element = char;

    type ElementRef<'a>
        = ValueRef<char>
    where
        Self: 'a;

    type SubSequence = StringSlice;

    fn start(&self) -> Self::Position {
        self.start_position
    }

    fn end(&self) -> Self::Position {
        self.end_position
    }

    fn form_next(&self, i: &mut Self::Position) {
        let len = unsafe {
            char_len_for(*self.start_address.add(*i).as_ref_unchecked())
        };
        *i += len
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
}

impl LazyCollection for StringSlice {
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.at(i).val
    }
}

impl UnsafeSubSequence for StringSlice {
    unsafe fn unsafe_at<'a>(&self, i: &Self::Position) -> Self::ElementRef<'a> {
        let p = unsafe { self.start_address.add(*i) };
        let l = unsafe { char_len_for(*p.as_ref_unchecked()) };
        let r = unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(p, l))
        }
        .chars()
        .next()
        .unwrap();
        ValueRef { val: r }
    }

    unsafe fn unsafe_slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubSequence {
        StringSlice {
            start_address: self.start_address,
            start_position: from,
            end_position: to,
        }
    }

    unsafe fn set_start(&mut self, p: Self::Position) {
        self.start_position = p
    }

    unsafe fn set_end(&mut self, p: Self::Position) {
        self.end_position = p
    }
}
