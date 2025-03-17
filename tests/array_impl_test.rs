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
    fn after() {
        let arr = [1, 2, 3];
        let i = arr.after(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn after_n() {
        let arr = [1, 2, 3];
        let i = arr.after_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn before() {
        let arr = [1, 2, 3];
        let i = arr.before(1);
        assert_eq!(i, 0);
    }

    #[test]
    fn before_n() {
        let arr = [1, 2, 3];
        let i = arr.before_n(3, 2);
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
}
