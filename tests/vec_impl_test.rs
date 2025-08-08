// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let arr = vec![1, 2, 3];
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let arr = vec![1, 2, 3];
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn at() {
        let arr = vec![1, 2, 3];
        assert_eq!(arr.at(&0), &1);
    }

    #[test]
    fn at_mut() {
        let mut arr = vec![1, 2, 3];
        *arr.at_mut(&0) = 2;
        assert_eq!(arr.at(&0), &2);
    }

    #[test]
    fn next() {
        let arr = vec![1, 2, 3];
        let i = arr.next(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn next_n() {
        let arr = vec![1, 2, 3];
        let i = arr.next_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn form_next_n_limited_by() {
        let arr = [1, 2, 3];

        let mut i = 0;
        let succ = arr.form_next_n_limited_by(&mut i, 0, 0);
        assert_eq!(i, 0);
        assert!(succ);

        let mut i = 0;
        let succ = arr.form_next_n_limited_by(&mut i, 1, 0);
        assert_eq!(i, 0);
        assert!(!succ);

        let mut i = 0;
        let succ = arr.form_next_n_limited_by(&mut i, 5, 3);
        assert_eq!(i, 3);
        assert!(!succ);
    }

    #[test]
    fn prior() {
        let arr = vec![1, 2, 3];
        let i = arr.prior(1);
        assert_eq!(i, 0);
    }

    #[test]
    fn prior_n() {
        let arr = vec![1, 2, 3];
        let i = arr.prior_n(3, 2);
        assert_eq!(i, 1);
    }

    #[test]
    fn form_prior_n_limited_by() {
        let arr = [1, 2, 3];

        let mut i = 3;
        let succ = arr.form_prior_n_limited_by(&mut i, 0, 0);
        assert_eq!(i, 3);
        assert!(succ);

        let mut i = 3;
        let succ = arr.form_prior_n_limited_by(&mut i, 1, 3);
        assert_eq!(i, 3);
        assert!(!succ);

        let mut i = 3;
        let succ = arr.form_prior_n_limited_by(&mut i, 1, 2);
        assert_eq!(i, 2);
        assert!(succ);
    }

    #[test]
    fn distance() {
        let arr = vec![1, 2, 3];
        let n = arr.distance(arr.start(), arr.end());
        assert_eq!(n, 3);
    }

    #[test]
    fn slice() {
        let arr = vec![1, 2, 3, 4, 5];
        let s = arr.slice(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        assert_eq!(*s.at(&2), 3);
        assert_eq!(s.iter().sum::<i32>(), 7)
    }

    #[test]
    fn slice_mut() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let mut s = arr.slice_mut(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        *s.at_mut(&2) = 5;
        assert_eq!(*s.at(&2), 5);
    }

    #[test]
    fn swap_at() {
        let mut array = vec![1, 2, 3, 4];
        array.swap_at(&0, &2);
        assert_eq!(array, [3, 2, 1, 4]);
    }
}
