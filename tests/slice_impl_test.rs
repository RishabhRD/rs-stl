// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        assert_eq!(arr.start(), 0);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        assert_eq!(arr.end(), 3);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn at() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        assert_eq!(arr.at(&0), &1);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        assert_eq!(arr.at(&0), &1);
    }

    #[test]
    fn at_mut() {
        let mut array = [1, 2, 3];
        let mut arr: &mut [i32] = &mut array;
        *arr.at_mut(&0) = 2;
        assert_eq!(arr.at(&0), &2);
    }

    #[test]
    fn after() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        let i = arr.after(0);
        assert_eq!(i, 1);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        let i = arr.after(0);
        assert_eq!(i, 1);
    }

    #[test]
    fn after_n() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        let i = arr.after_n(0, 2);
        assert_eq!(i, 2);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        let i = arr.after_n(0, 2);
        assert_eq!(i, 2);
    }

    #[test]
    fn before() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        let i = arr.before(1);
        assert_eq!(i, 0);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        let i = arr.before(1);
        assert_eq!(i, 0);
    }

    #[test]
    fn before_n() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        let i = arr.before_n(3, 2);
        assert_eq!(i, 1);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        let i = arr.before_n(3, 2);
        assert_eq!(i, 1);
    }

    #[test]
    fn distance() {
        let array = [1, 2, 3];
        let arr: &[i32] = &array;
        let n = arr.distance(arr.start(), arr.end());
        assert_eq!(n, 3);

        let mut array = [1, 2, 3];
        let arr: &mut [i32] = &mut array;
        let n = arr.distance(arr.start(), arr.end());
        assert_eq!(n, 3);
    }

    #[test]
    fn slice() {
        let array = [1, 2, 3, 4, 5];
        let arr: &[i32] = &array;
        let s = arr.slice(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        assert_eq!(*s.at(&2), 3);

        let mut array = [1, 2, 3, 4, 5];
        let arr: &mut [i32] = &mut array;
        let s = arr.slice(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        assert_eq!(*s.at(&2), 3);
    }

    #[test]
    fn slice_mut() {
        let mut array = [1, 2, 3, 4, 5];
        let mut arr: &mut [i32] = &mut array;
        let mut s = arr.slice_mut(2, 4);
        assert_eq!(s.start(), 2);
        assert_eq!(s.end(), 4);
        *s.at_mut(&2) = 5;
        assert_eq!(*s.at(&2), 5);
    }

    #[test]
    fn swap_at() {
        let mut array = [1, 2, 3, 4];
        let mut arr: &mut [i32] = &mut array;
        arr.swap_at(&0, &2);
        assert_eq!(arr, [3, 2, 1, 4]);
    }
}
