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

            type Whole = Self;

            fn start(&self) -> Self::Position {
                0
            }

            fn end(&self) -> Self::Position {
                5
            }

            fn after(&self, i: Self::Position) -> Self::Position {
                i + 1
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
        }

        impl BidirectionalCollection for NonJumpableCollection {
            fn before(&self, i: Self::Position) -> Self::Position {
                i - 1
            }
        }
    }

    use details::NonJumpableCollection;

    #[test]
    fn after_n() {
        let arr = NonJumpableCollection {
            data: [1, 2, 3, 4, 5],
        };
        let i = arr.after_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn before_n() {
        let arr = NonJumpableCollection {
            data: [1, 2, 3, 4, 5],
        };
        let i = arr.before_n(2, 2);
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
