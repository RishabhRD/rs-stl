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
    fn next() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.next(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn next_n() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.next_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn advance() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let mut i = 0;
        arr.form_next(&mut i);
        assert_eq!(i, 1);
    }

    #[test]
    fn advance_n() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let mut i = 0;
        arr.form_next_n(&mut i, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn prior() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.prior(1);
        assert_eq!(i, 0);
    }

    #[test]
    fn prior_n() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let i = arr.prior_n(3, 2);
        assert_eq!(i, 1);
    }

    #[test]
    fn backstep() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let mut i = 1;
        arr.form_prior(&mut i);
        assert_eq!(i, 0);
    }

    #[test]
    fn backstep_n() {
        let array = [1, 2, 3];
        let arr = array.slice(array.start(), array.end());
        let mut i = 3;
        arr.form_prior_n(&mut i, 2);
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

    #[test]
    fn pop_elements() {
        let arr = [1, 2, 3];
        let mut s = arr.all();
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

    #[test]
    fn compute_at() {
        let arr = 0..=3;
        assert_eq!(arr.all().compute_at(&0), 0);
    }
}
