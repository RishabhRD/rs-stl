// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let arr = [1, 2, 3];
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let arr = [1, 2, 3];
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn at() {
        let arr = [1, 2, 3];
        assert_eq!(arr.at(&0), &1);
    }

    #[test]
    fn at_mut() {
        let mut arr = [1, 2, 3];
        *arr.at_mut(&0) = 2;
        assert_eq!(arr.at(&0), &2);
    }

    #[test]
    fn next() {
        let arr = [1, 2, 3];
        let i = arr.next(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn next_n() {
        let arr = [1, 2, 3];
        let i = arr.next_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn prior() {
        let arr = [1, 2, 3];
        let i = arr.prior(1);
        assert_eq!(i, 0);
    }

    #[test]
    fn prior_n() {
        let arr = [1, 2, 3];
        let i = arr.prior_n(3, 2);
        assert_eq!(i, 1);
    }

    #[test]
    fn distance() {
        let arr = [1, 2, 3];
        let n = arr.distance(arr.start(), arr.end());
        assert_eq!(n, 3);
    }

    #[test]
    fn slice() {
        let arr = [1, 2, 3, 4, 5];
        let s = arr.slice(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        assert_eq!(*s.at(&2), 3);
    }

    #[test]
    fn slice_mut() {
        let mut arr = [1, 2, 3, 4, 5];
        let mut s = arr.slice_mut(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        *s.at_mut(&2) = 5;
        assert_eq!(*s.at(&2), 5);
    }

    #[test]
    fn swap_at() {
        let mut array = [1, 2, 3, 4];
        array.swap_at(&0, &2);
        assert_eq!(array, [3, 2, 1, 4]);
    }

    #[test]
    fn count() {
        let arr = [1, 2, 3, 4];
        assert_eq!(arr.count(), 4);
    }
}
