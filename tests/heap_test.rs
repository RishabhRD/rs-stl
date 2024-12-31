// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    fn assert_is_heap_until<const N: usize>(arr: [i32; N], expected: usize) {
        let i =
            algo::is_heap_until_by(&arr, arr.start(), arr.end(), |x, y| x < y);
        assert_eq!(i, expected);

        let i = rng::is_heap_until_by(&arr, |x, y| x < y);
        assert_eq!(i, expected);

        let i = arr.is_heap_until_by(|x, y| x < y);
        assert_eq!(i, expected);

        let i = algo::is_heap_until(&arr, arr.start(), arr.end());
        assert_eq!(i, expected);

        let i = rng::is_heap_until(&arr);
        assert_eq!(i, expected);

        let i = arr.is_heap_until();
        assert_eq!(i, expected);
    }

    #[test]
    fn is_heap_until() {
        let arr = [9, 5, 4, 1, 1, 3, 2, 6];
        assert_is_heap_until(arr, 7);

        let arr = [9, 5, 4];
        assert_is_heap_until(arr, 3);

        let arr = [9, 5, 10];
        assert_is_heap_until(arr, 2);

        let arr = [9, 10, 5];
        assert_is_heap_until(arr, 1);

        let arr = [10, 9];
        assert_is_heap_until(arr, 2);

        let arr = [9, 10];
        assert_is_heap_until(arr, 1);

        let arr = [9];
        assert_is_heap_until(arr, 1);

        let arr: [i32; 0] = [];
        assert_is_heap_until(arr, 0);
    }

    #[test]
    fn is_heap() {
        let arr = [9, 5, 4, 1, 1, 3, 2];
        assert!(algo::is_heap_by(&arr, arr.start(), arr.end(), |x, y| x < y));
        assert!(rng::is_heap_by(&arr, |x, y| x < y));
        assert!(arr.is_heap_by(|x, y| x < y));

        let arr = [9, 5, 4, 1, 9, 3, 2];
        assert!(!algo::is_heap_by(&arr, arr.start(), arr.end(), |x, y| x < y));
        assert!(!rng::is_heap_by(&arr, |x, y| x < y));
        assert!(!arr.is_heap_by(|x, y| x < y));

        let arr: [i32; 0] = [];
        assert!(algo::is_heap_by(&arr, arr.start(), arr.end(), |x, y| x < y));
        assert!(rng::is_heap_by(&arr, |x, y| x < y));
        assert!(arr.is_heap_by(|x, y| x < y));
    }

    fn assert_push_heap<const N: usize>(rng: [i32; N]) {
        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        algo::push_heap_by(&mut arr, start, end, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = rng;
        rng::push_heap_by(&mut arr, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = rng;
        arr.push_heap_by(|x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        algo::push_heap(&mut arr, start, end);
        assert!(arr.is_heap());

        let mut arr = rng;
        rng::push_heap(&mut arr);
        assert!(arr.is_heap());

        let mut arr = rng;
        arr.push_heap();
        assert!(arr.is_heap());
    }

    #[test]
    fn push_heap() {
        assert_push_heap([8, 2, 9]);
        assert_push_heap([8]);
        assert_push_heap([]);
        assert_push_heap([8, 2]);
        assert_push_heap([2, 8]);
    }

    fn assert_pop_heap<const N: usize>(rng: [i32; N]) {
        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        if N != 0 {
            let first = *arr.at(&start);
            algo::pop_heap_by(&mut arr, start, end, |x, y| x < y);
            assert!(&arr[0..N - 1].is_heap_by(|x, y| x < y));
            assert_eq!(*arr.at(&(end - 1)), first);
        } else {
            algo::pop_heap_by(&mut arr, start, end, |x, y| x < y);
        }

        let mut arr = rng;
        if N != 0 {
            let first = *arr.at(&start);
            rng::pop_heap_by(&mut arr, |x, y| x < y);
            assert!(&arr[0..N - 1].is_heap_by(|x, y| x < y));
            assert_eq!(*arr.at(&(end - 1)), first);
        } else {
            rng::pop_heap_by(&mut arr, |x, y| x < y);
        }

        let mut arr = rng;
        if N != 0 {
            let first = *arr.at(&start);
            arr.pop_heap_by(|x, y| x < y);
            assert!(&arr[0..N - 1].is_heap_by(|x, y| x < y));
            assert_eq!(*arr.at(&(end - 1)), first);
        } else {
            arr.pop_heap_by(|x, y| x < y);
        }

        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        if N != 0 {
            let first = *arr.at(&start);
            algo::pop_heap(&mut arr, start, end);
            assert!(&arr[0..N - 1].is_heap());
            assert_eq!(*arr.at(&(end - 1)), first);
        } else {
            algo::pop_heap(&mut arr, start, end);
        }

        let mut arr = rng;
        if N != 0 {
            let first = *arr.at(&start);
            rng::pop_heap(&mut arr);
            assert!(&arr[0..N - 1].is_heap());
            assert_eq!(*arr.at(&(end - 1)), first);
        } else {
            rng::pop_heap(&mut arr);
        }

        let mut arr = rng;
        if N != 0 {
            let first = *arr.at(&start);
            arr.pop_heap();
            assert!(&arr[0..N - 1].is_heap());
            assert_eq!(*arr.at(&(end - 1)), first);
        } else {
            arr.pop_heap();
        }
    }

    #[test]
    fn pop_heap() {
        assert_pop_heap([9, 8, 7]);
        assert_pop_heap([9, 8]);
        assert_pop_heap([9]);
        assert_pop_heap([]);
        assert_pop_heap([9, 5, 4, 1, 1, 3, 2]);
    }

    fn assert_sort_heap<const N: usize>(rng: [i32; N]) {
        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        algo::sort_heap_by(&mut arr, start, end, |x, y| x < y);
        assert!(arr.is_sorted_by(|x, y| x < y));

        let mut arr = rng;
        rng::sort_heap_by(&mut arr, |x, y| x < y);
        assert!(arr.is_sorted_by(|x, y| x < y));

        let mut arr = rng;
        arr.sort_heap_by(|x, y| x < y);
        assert!(arr.is_sorted_by(|x, y| x < y));

        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        algo::sort_heap(&mut arr, start, end);
        assert!(arr.is_sorted());

        let mut arr = rng;
        rng::sort_heap(&mut arr);
        assert!(arr.is_sorted());

        let mut arr = rng;
        arr.sort_heap();
        assert!(arr.is_sorted());
    }

    #[test]
    fn sort_heap() {
        assert_sort_heap([9, 8, 7]);
        assert_sort_heap([9, 8]);
        assert_sort_heap([9]);
        assert_sort_heap([]);
        assert_sort_heap([9, 5, 4, 1, 1, 3, 2]);
    }

    fn assert_make_heap<const N: usize>(rng: [i32; N]) {
        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        algo::make_heap_by(&mut arr, start, end, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = rng;
        rng::make_heap_by(&mut arr, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = rng;
        arr.make_heap_by(|x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        algo::make_heap(&mut arr, start, end);
        assert!(arr.is_heap());

        let mut arr = rng;
        rng::make_heap(&mut arr);
        assert!(arr.is_heap());

        let mut arr = rng;
        arr.make_heap();
        assert!(arr.is_heap());
    }

    #[test]
    fn make_heap() {
        assert_make_heap([5, 1, 1, 2, 9, 4]);
        assert_make_heap([1, 1, 2, 4, 5, 9]);
        assert_make_heap([9, 5, 4, 1, 1, 3, 2]);
        assert_make_heap([1, 1, 2, 4, 5]);
        assert_make_heap([1, 1, 2]);
        assert_make_heap([1, 1]);
        assert_make_heap([1]);
        assert_make_heap([]);
    }
}
