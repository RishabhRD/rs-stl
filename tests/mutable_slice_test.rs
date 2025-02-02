// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn after() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(arr.after(0), 1);
    }

    #[test]
    fn after_n() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(arr.after_n(0, 2), 2);
    }

    #[test]
    fn before() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(arr.before(1), 0);
    }

    #[test]
    fn before_n() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(arr.before_n(2, 2), 0);
    }

    #[test]
    fn distance() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(arr.distance(0, 2), 2);
    }

    #[test]
    fn at_as_deref() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(*arr.at_ref(&0), 10);
    }

    #[test]
    fn at() {
        let mut array = [10, 20, 30];
        let arr = array.slice_mut();
        assert_eq!(*arr.at(&0), 10);
    }

    #[test]
    fn slice() {
        let mut array = [10, 20, 30];
        let v = array.slice_mut();
        let arr = v.slice();
        assert_eq!(arr.start(), 0);
        assert_eq!(arr.end(), 3);
        assert_eq!(*arr.at(&0), 10);
        assert_eq!(*arr.at(&1), 20);
        assert_eq!(*arr.at(&2), 30);
    }

    #[test]
    fn slice_mut() {
        let mut array = [10, 20, 30];
        let mut v = array.slice_mut();
        let arr = v.slice_mut();
        assert_eq!(arr.start(), 0);
        assert_eq!(arr.end(), 3);
        assert_eq!(*arr.at(&0), 10);
        assert_eq!(*arr.at(&1), 20);
        assert_eq!(*arr.at(&2), 30);
    }

    #[test]
    fn clone() {
        let mut array = [10, 20, 30];
        let mut v = array.slice_mut();
        let arr = v.clone();
        assert_eq!(arr.start(), 0);
        assert_eq!(arr.end(), 3);
        assert_eq!(*arr.at(&0), 10);
        assert_eq!(*arr.at(&1), 20);
        assert_eq!(*arr.at(&2), 30);
    }

    #[test]
    fn prefix() {
        let mut array = [10, 20, 30];
        let v = array.slice_mut(); // TODO: make array.slice_mut().prefix(2) possible to use.
        let arr = v.prefix(2);
        assert_eq!(arr.start(), 0);
        assert_eq!(arr.end(), 2);
        assert_eq!(*arr.at(&0), 10);
        assert_eq!(*arr.at(&1), 20);
    }

    #[test]
    fn suffix() {
        let mut array = [10, 20, 30];
        let v = array.slice_mut();
        let arr = v.suffix(1);
        assert_eq!(arr.start(), 1);
        assert_eq!(arr.end(), 3);
        assert_eq!(*arr.at(&1), 20);
        assert_eq!(*arr.at(&2), 30);
    }

    #[test]
    fn subrange() {
        let mut array = [10, 20, 30];
        let v = array.slice_mut();
        let arr = v.subrange(1, 2);
        assert_eq!(arr.start(), 1);
        assert_eq!(arr.end(), 2);
        assert_eq!(*arr.at(&1), 20);
    }

    #[test]
    fn prefix_mut() {
        let mut array = [10, 20, 30];
        let mut v = array.slice_mut(); // TODO: make array.slice_mut().prefix(2) possible to use.
        let arr = v.prefix_mut(2);
        assert_eq!(arr.start(), 0);
        assert_eq!(arr.end(), 2);
        assert_eq!(*arr.at(&0), 10);
        assert_eq!(*arr.at(&1), 20);
    }

    #[test]
    fn suffix_mut() {
        let mut array = [10, 20, 30];
        let mut v = array.slice_mut();
        let arr = v.suffix_mut(1);
        assert_eq!(arr.start(), 1);
        assert_eq!(arr.end(), 3);
        assert_eq!(*arr.at(&1), 20);
        assert_eq!(*arr.at(&2), 30);
    }

    #[test]
    fn subrange_mut() {
        let mut array = [10, 20, 30];
        let mut v = array.slice_mut();
        let arr = v.subrange_mut(1, 2);
        assert_eq!(arr.start(), 1);
        assert_eq!(arr.end(), 2);
        assert_eq!(*arr.at(&1), 20);
    }

    #[test]
    fn swap_at() {
        let mut arr = [1, 2, 3];
        arr.slice_mut().swap_at(&0, &1);
        assert_eq!(arr, [2, 1, 3])
    }

    #[test]
    fn at_mut() {
        let mut arr = [1, 2, 3];
        *arr.slice_mut().at_mut(&0) = 2;
        assert_eq!(arr, [2, 2, 3])
    }

    // TODO: add test: at, at_mut for lazy collection.
}
