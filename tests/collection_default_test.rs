// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    mod details {
        use stl::{BidirectionalCollection, Collection, Slice};

        #[derive(Clone, PartialEq, Eq)]
        pub struct NonJumpableCollection {
            pub data: [i32; 5],
        }

        impl Collection for NonJumpableCollection {
            type Position = usize;

            type Element = i32;

            type ElementRef<'a>
                = &'a i32
            where
                Self: 'a;

            type Whole = Self;

            type Iter<'a>
                = std::slice::Iter<'a, i32>
            where
                Self: 'a;

            fn start(&self) -> Self::Position {
                0
            }

            fn end(&self) -> Self::Position {
                5
            }

            fn at(&self, i: &Self::Position) -> &Self::Element {
                &self.data[*i]
            }

            fn slice(
                &self,
                from: Self::Position,
                to: Self::Position,
            ) -> stl::Slice<Self::Whole> {
                Slice::new(self, from, to)
            }

            fn form_next(&self, i: &mut Self::Position) {
                *i += 1
            }

            fn iter_within(
                &self,
                from: Self::Position,
                to: Self::Position,
            ) -> Self::Iter<'_> {
                <[i32]>::iter(&self.data[from..to])
            }
        }

        impl BidirectionalCollection for NonJumpableCollection {
            fn form_prior(&self, i: &mut Self::Position) {
                *i -= 1
            }
        }
    }

    use details::NonJumpableCollection;

    #[test]
    fn next_n() {
        let arr = NonJumpableCollection {
            data: [1, 2, 3, 4, 5],
        };
        let i = arr.next_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn prior() {
        let arr = NonJumpableCollection {
            data: [1, 2, 3, 4, 5],
        };
        let i = arr.prior_n(2, 2);
        assert_eq!(i, 0);
    }

    #[test]
    fn distance() {
        let arr = NonJumpableCollection {
            data: [1, 2, 3, 4, 5],
        };
        let i = arr.distance(arr.start(), arr.end());
        assert_eq!(i, 5);
    }
}
