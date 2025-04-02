// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn at() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        assert_eq!(arr.at(&0), &1);
    }

    #[test]
    fn at_mut() {
        let mut array = [1, 2, 3];
        let mut arr = array.slice_mut(array.start(), array.end());
        *arr.at_mut(&0) = 2;
        assert_eq!(arr.at(&0), &2);
    }

    #[test]
    fn after() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        let i = arr.after(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn after_n() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        let i = arr.after_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn before() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        let i = arr.before(1);
        assert_eq!(i, 0);
    }

    #[test]
    fn before_n() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        let i = arr.before_n(3, 2);
        assert_eq!(i, 1);
    }

    #[test]
    fn distance() {
        let mut array = [1, 2, 3];
        let arr = array.slice_mut(array.start(), array.end());
        let n = arr.distance(arr.start(), arr.end());
        assert_eq!(n, 3);
    }

    #[test]
    fn slice() {
        let mut array = [1, 2, 3, 4, 5];
        let arr = array.slice_mut(array.start(), array.end());
        let s = arr.slice(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        assert_eq!(*s.at(&2), 3);
    }

    #[test]
    fn slice_mut() {
        let mut array = [1, 2, 3, 4, 5];
        let mut arr = array.slice_mut(array.start(), array.end());
        let mut s = arr.slice_mut(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        *s.at_mut(&2) = 5;
        assert_eq!(*s.at(&2), 5);
    }

    #[test]
    fn prefix_mut() {
        let mut arr = [1, 2, 3];
        assert!(arr.prefix_mut(1).equals(&[1]));
    }

    #[test]
    fn suffix_mut() {
        let mut arr = [1, 2, 3];
        assert!(arr.suffix_mut(1).equals(&[2, 3]));
    }

    #[test]
    fn pop_elements() {
        let mut arr = [1, 2, 3];
        let mut s = arr.all_mut();

        let e = s.pop_first();
        assert_eq!(e, Some(&1));
        assert!(s.equals(&[2, 3]));

        assert!(s.drop_first());
        assert!(s.equals(&[3]));
        assert!(s.drop_first());
        assert!(s.equals(&[]));
        assert!(!s.drop_first());
        assert!(s.equals(&[]));

        let e = s.pop_first();
        assert_eq!(e, None);
        assert!(s.equals(&[]));
    }
}
