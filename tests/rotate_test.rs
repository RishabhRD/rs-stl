// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use stl::*;

struct ForwardArray<const N: usize> {
    arr: [i32; N],
}

impl<const N: usize> InputRange for ForwardArray<N> {
    type Element = i32;

    type Position = usize;

    fn start(&self) -> Self::Position {
        0
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn at(&self, i: &Self::Position) -> &Self::Element {
        &self.arr[*i]
    }

    fn is_end(&self, i: Self::Position) -> bool {
        i == N
    }
}

impl<const N: usize> ForwardRange for ForwardArray<N> {
    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }
}

impl<const N: usize> SemiOutputRange for ForwardArray<N> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        self.arr.swap_at(i, j)
    }
}

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    use crate::ForwardArray;

    #[test]
    fn rotate_forward() {
        let mut arr = ForwardArray {
            arr: [0, 1, 2, 3, 4],
        };
        let i = algo::rotate(&mut arr, 0, 2, 5);
        assert_eq!(i, 3);
        assert!(arr.equals(&[2, 3, 4, 0, 1]));

        let mut arr = ForwardArray {
            arr: [0, 1, 2, 3, 4],
        };
        let i = algo::rotate(&mut arr, 0, 0, 5);
        assert_eq!(i, 5);
        assert!(arr.equals(&[0, 1, 2, 3, 4]));

        let mut arr = ForwardArray {
            arr: [0, 1, 2, 3, 4],
        };
        let i = algo::rotate(&mut arr, 0, 5, 5);
        assert_eq!(i, 0);
        assert!(arr.equals(&[0, 1, 2, 3, 4]));

        let mut arr = ForwardArray {
            arr: [0, 1, 2, 3, 4],
        };
        let i = rng::rotate(&mut arr, 2);
        assert_eq!(i, 3);
        assert!(arr.equals(&[2, 3, 4, 0, 1]));

        let mut arr = ForwardArray {
            arr: [0, 1, 2, 3, 4],
        };
        let i = arr.rotate(2);
        assert_eq!(i, 3);
        assert!(arr.equals(&[2, 3, 4, 0, 1]));
    }

    #[test]
    fn rotate_copy() {
        let arr = [0, 1, 2, 3, 4];

        let mut dest = [0, 0, 0, 0, 0];
        let i = algo::rotate_copy(&arr, 0, 2, 5, &mut dest, 0);
        assert_eq!(i, 5);
        assert!(dest.equals(&[2, 3, 4, 0, 1]));

        let mut dest = [0, 0, 0, 0, 0];
        let i = rng::rotate_copy(&arr, 2, &mut dest);
        assert_eq!(i, 5);
        assert!(dest.equals(&[2, 3, 4, 0, 1]));

        let mut dest = [0, 0, 0, 0, 0];
        let i = algo::rotate_copy(&arr, 0, 0, 0, &mut dest, 0);
        assert_eq!(i, 0);
        assert!(dest.equals(&[0, 0, 0, 0, 0]));

        let mut dest = [0, 0, 0, 0, 0];
        let i = algo::rotate_copy(&arr, 0, 0, 5, &mut dest, 0);
        assert_eq!(i, 5);
        assert!(dest.equals(&[0, 1, 2, 3, 4]));

        let mut dest = [0, 0, 0, 0, 0];
        let i = algo::rotate_copy(&arr, 0, 5, 5, &mut dest, 0);
        assert_eq!(i, 5);
        assert!(dest.equals(&[0, 1, 2, 3, 4]));
    }
}
