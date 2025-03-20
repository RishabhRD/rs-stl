// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn at() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        assert_eq!(arr.at(&0), &1);
    }

    #[test]
    fn after() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.after(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn after_n() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.after_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn before() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.before(1);
        assert_eq!(i, 0);
    }

    #[test]
    fn before_n() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.before_n(3, 2);
        assert_eq!(i, 1);
    }

    #[test]
    fn distance() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let n = arr.distance(arr.start(), arr.end());
        assert_eq!(n, 3);
    }

    #[test]
    fn slice() {
        let array = [1, 2, 3, 4, 5];
        let arr = array.slice(array.start(), array.end());
        let s = arr.slice(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        assert_eq!(*s.at(&2), 3);
    }

    #[test]
    fn swap_at() {
        let mut array = [1, 2, 3, 4];
        let mut slice = array.all_mut();
        slice.swap_at(&0, &2);
        assert_eq!(array, [3, 2, 1, 4]);
    }

    #[test]
    fn prefix() {
        let arr = [1, 2, 3];
        assert!(arr.prefix(1).equals(&[1]));
    }

    #[test]
    fn suffix() {
        let arr = [1, 2, 3];
        assert!(arr.suffix(1).equals(&[2, 3]));
    }
}
