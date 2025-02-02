// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn after() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(arr.after(0), 1);
    }

    #[test]
    fn after_n() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(arr.after_n(0, 2), 2);
    }

    #[test]
    fn before() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(arr.before(1), 0);
    }

    #[test]
    fn before_n() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(arr.before_n(2, 2), 0);
    }

    #[test]
    fn distance() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(arr.distance(0, 2), 2);
    }

    #[test]
    fn at_as_deref() {
        let array = [10, 20, 30];
        let arr = array.slice();
        assert_eq!(*arr.at_ref(&0), 10);
    }

    #[test]
    fn at() {
        let array = [10, 20, 30];
        let arr = Slice::new(&array, array.start(), array.end());
        assert_eq!(*arr.at(&0), 10);
    }

    #[test]
    fn slice() {
        let array = [10, 20, 30];
        let arr = array.slice().slice();
        assert_eq!(arr.start(), 0);
        assert_eq!(arr.end(), 3);
        assert_eq!(*arr.at(&0), 10);
        assert_eq!(*arr.at(&1), 20);
        assert_eq!(*arr.at(&2), 30);
    }

    #[test]
    fn prefix() {
        let array = [10, 20, 30];
        let arr = array.slice().prefix(2);
        assert_eq!(arr.start(), 0);
        assert_eq!(arr.end(), 2);
        assert_eq!(*arr.at(&0), 10);
        assert_eq!(*arr.at(&1), 20);
    }

    #[test]
    fn suffix() {
        let array = [10, 20, 30];
        let arr = array.slice().suffix(1);
        assert_eq!(arr.start(), 1);
        assert_eq!(arr.end(), 3);
        assert_eq!(*arr.at(&1), 20);
        assert_eq!(*arr.at(&2), 30);
    }

    #[test]
    fn subrange() {
        let array = [10, 20, 30];
        let arr = array.slice().subrange(1, 2);
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
