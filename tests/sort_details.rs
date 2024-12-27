// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::algo::sort::sort_details::*;
    use stl::*;

    #[test]
    fn insertion_sort_non_empty_range() {
        let mut arr = [(2, 4), (2, 3), (1, 2), (1, 3)];
        let start = arr.start();
        let end = arr.end();
        insertion_sort(&mut arr, start, end, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 2), (1, 3), (2, 4), (2, 3)]));
    }

    #[test]
    fn insertion_sort_empty_range() {
        let mut arr: [i32; 0] = [];
        let start = arr.start();
        let end = arr.end();
        insertion_sort(&mut arr, start, end, |x, y| x < y);
        assert!(arr.equals(&[]));
    }

    #[test]
    fn quick_sort_till_depth_test() {
        let mut arr = [3, 1, 2];
        let start = arr.start();
        let end = arr.end();
        let res = quick_sort_till_depth(&mut arr, start, end, |x, y| x < y, 9);
        assert!(res);
        assert!(arr.equals(&[1, 2, 3]));

        let mut arr: [i32; 0] = [];
        let start = arr.start();
        let end = arr.end();
        let res = quick_sort_till_depth(&mut arr, start, end, |x, y| x < y, 0);
        assert!(res);
        assert!(arr.equals(&[]));

        let mut arr = [5, 4, 3, 2, 1];
        let start = arr.start();
        let end = arr.end();
        let res = quick_sort_till_depth(&mut arr, start, end, |x, y| x < y, 1);
        assert!(!res);
    }

    #[test]
    fn heap_sort_test() {
        let mut arr = [3, 1, 2];
        let start = arr.start();
        let end = arr.end();
        heap_sort(&mut arr, start, end, |x, y| x < y);
        assert!(arr.equals(&[1, 2, 3]));

        let mut arr = [3, 1, 2];
        let start = arr.start();
        let end = arr.end();
        heap_sort(&mut arr, start, end, |x, y| x > y);
        assert!(arr.equals(&[3, 2, 1]));

        let mut arr: [i32; 0] = [];
        let start = arr.start();
        let end = arr.end();
        heap_sort(&mut arr, start, end, |x, y| x < y);
        assert!(arr.equals(&[]));
    }
}
