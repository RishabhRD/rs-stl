// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

mod __details {
    use crate::InputRange;
    pub struct RangeIterator<'a, R>
    where
        R: InputRange + ?Sized,
    {
        pub range: &'a R,
        pub pos: Option<R::Position>,
    }

    impl<'a, R> Iterator for RangeIterator<'a, R>
    where
        R: InputRange + ?Sized,
    {
        type Item = R::ElementRef<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.range.is_end(self.pos.as_ref().unwrap()) {
                None
            } else {
                let element = self.range.at(self.pos.as_ref().unwrap());
                let cur_pos = self.pos.take();
                let next = self.range.after(cur_pos.unwrap());
                self.pos = Some(next);
                Some(element)
            }
        }
    }
}

pub mod infix {
    use crate::InputRange;

    /// `iter`.
    pub trait STLIterExt: InputRange {
        /// Returns iterator for given range.
        fn iter(&self) -> super::__details::RangeIterator<Self>
where {
            super::__details::RangeIterator {
                range: self,
                pos: Some(self.start()),
            }
        }
    }

    impl<R> STLIterExt for R where R: InputRange + ?Sized {}
}
