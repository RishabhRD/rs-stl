// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use details::Basic;
    use stl::*;

    mod details {
        use stl::{Collection, ReorderableCollection, Slice, SliceMut};

        #[derive(Clone, PartialEq, Eq)]
        pub struct Basic {
            pub data: [i32; 5],
        }

        impl Collection for Basic {
            type Position = usize;

            type Element = i32;

            type ElementRef<'a>
                = &'a i32
            where
                Self: 'a;

            type Whole = Self;

            fn start(&self) -> Self::Position {
                0
            }

            fn end(&self) -> Self::Position {
                5
            }

            fn form_next(&self, i: &mut Self::Position) {
                *i += 1
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

        impl ReorderableCollection for Basic {
            fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
                self.data.swap(*i, *j)
            }

            fn slice_mut(
                &mut self,
                from: Self::Position,
                to: Self::Position,
            ) -> stl::SliceMut<Self::Whole> {
                SliceMut::new(self, from, to)
            }
        }
    }

    #[test]
    fn type_check() {
        let arr = Basic {
            data: [1, 2, 3, 4, 5],
        };
        let slice: Slice<Basic> = arr.full();
        assert!(slice == Slice::<Basic>::new(&arr, arr.start(), arr.end()));
    }

    #[test]
    fn should_not_nest() {
        let arr = Basic {
            data: [1, 2, 3, 4, 5],
        };
        let slice1: Slice<Basic> = arr.full();
        let slice2: Slice<Basic> = slice1.full();
        assert!(slice1 == slice2);
    }

    #[test]
    fn type_check_mut() {
        let mut arr = Basic {
            data: [1, 2, 3, 4, 5],
        };
        let _: SliceMut<Basic> = arr.full_mut();
        // Can't assert equality as 2 mutable references are not possible at a time.
    }

    #[test]
    fn should_not_nest_mut() {
        let mut arr = Basic {
            data: [1, 2, 3, 4, 5],
        };
        let mut slice1: SliceMut<Basic> = arr.full_mut();
        let _: SliceMut<Basic> = slice1.full_mut();
        // Can't assert equality as 2 mutable references are not possible at a time.
    }
}
