// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn is_heap_until() {
        let arr = [9, 5, 4, 1, 1, 3, 2, 6];

        let i =
            algo::is_heap_until_by(&arr, arr.start(), arr.end(), |x, y| x < y);
        assert_eq!(i, 7);

        let i = rng::is_heap_until_by(&arr, |x, y| x < y);
        assert_eq!(i, 7);

        let i = arr.is_heap_until_by(|x, y| x < y);
        assert_eq!(i, 7);

        let i = algo::is_heap_until(&arr, arr.start(), arr.end());
        assert_eq!(i, 7);

        let i = rng::is_heap_until(&arr);
        assert_eq!(i, 7);

        let i = arr.is_heap_until();
        assert_eq!(i, 7);

        let arr = [9, 5, 4];
        let i = arr.is_heap_until();
        assert_eq!(i, 3);

        let arr = [9, 5, 10];
        let i = arr.is_heap_until();
        assert_eq!(i, 2);

        let arr = [9, 10, 5];
        let i = arr.is_heap_until();
        assert_eq!(i, 1);

        let arr = [10, 9];
        let i = arr.is_heap_until();
        assert_eq!(i, 2);

        let arr = [9, 10];
        let i = arr.is_heap_until();
        assert_eq!(i, 1);

        let arr = [9];
        let i = arr.is_heap_until();
        assert_eq!(i, 1);

        let arr: [i32; 0] = [];
        let i = arr.is_heap_until();
        assert_eq!(i, 0);
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

    #[test]
    fn push_heap() {
        let mut arr = [8, 2, 9];
        let start = arr.start();
        let end = arr.end();
        algo::push_heap_by(&mut arr, start, end, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = [8, 2, 9];
        rng::push_heap_by(&mut arr, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = [8, 2, 9];
        arr.push_heap_by(|x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = [8, 2, 9];
        let start = arr.start();
        let end = arr.end();
        algo::push_heap(&mut arr, start, end);
        assert!(arr.is_heap());

        let mut arr = [8, 2, 9];
        rng::push_heap(&mut arr);
        assert!(arr.is_heap());

        let mut arr = [8, 2, 9];
        arr.push_heap();
        assert!(arr.is_heap());

        let mut arr = [8];
        arr.push_heap();
        assert!(arr.is_heap());

        let mut arr: [i32; 0] = [];
        arr.push_heap();
        assert!(arr.is_heap());

        let mut arr = [8, 2];
        arr.push_heap();
        assert!(arr.is_heap());

        let mut arr = [2, 8];
        arr.push_heap();
        assert!(arr.is_heap());
    }

    #[test]
    fn pop_heap() {
        let mut arr = [9, 8, 7];
        let start = arr.start();
        let end = arr.end();
        algo::pop_heap_by(&mut arr, start, end, |x, y| x < y);
        assert!(&arr[0..2].is_heap_by(|x, y| x < y));
        assert!(arr.equals(&[8, 7, 9]));

        let mut arr = [9, 8, 7];
        rng::pop_heap_by(&mut arr, |x, y| x < y);
        assert!(&arr[0..2].is_heap_by(|x, y| x < y));
        assert!(arr.equals(&[8, 7, 9]));

        let mut arr = [9, 8, 7];
        arr.pop_heap_by(|x, y| x < y);
        assert!(&arr[0..2].is_heap_by(|x, y| x < y));
        assert!(arr.equals(&[8, 7, 9]));

        let mut arr = [9, 8, 7];
        let start = arr.start();
        let end = arr.end();
        algo::pop_heap(&mut arr, start, end);
        assert!(&arr[0..2].is_heap());
        assert!(arr.equals(&[8, 7, 9]));

        let mut arr = [9, 8, 7];
        rng::pop_heap(&mut arr);
        assert!(&arr[0..2].is_heap());
        assert!(arr.equals(&[8, 7, 9]));

        let mut arr = [9, 8, 7];
        arr.pop_heap();
        assert!(&arr[0..2].is_heap());
        assert!(arr.equals(&[8, 7, 9]));

        let mut arr = [9, 8];
        arr.pop_heap();
        assert!(arr.equals(&[8, 9]));

        let mut arr = [9];
        arr.pop_heap();
        assert!(arr.equals(&[9]));

        let mut arr: [i32; 0] = [];
        arr.pop_heap();
        assert!(arr.equals(&[]));

        let mut arr = [9, 5, 4, 1, 1, 3, 2];
        arr.pop_heap();
        assert!(arr[0..6].is_heap());
    }

    #[test]
    fn sort_heap() {
        let mut arr = [9, 8, 7];
        let start = arr.start();
        let end = arr.end();
        algo::sort_heap_by(&mut arr, start, end, |x, y| x < y);
        assert!(arr.equals(&[7, 8, 9]));

        let mut arr = [9, 8, 7];
        rng::sort_heap_by(&mut arr, |x, y| x < y);
        assert!(arr.equals(&[7, 8, 9]));

        let mut arr = [9, 8, 7];
        arr.sort_heap_by(|x, y| x < y);
        assert!(arr.equals(&[7, 8, 9]));

        let mut arr = [9, 8, 7];
        let start = arr.start();
        let end = arr.end();
        algo::sort_heap(&mut arr, start, end);
        assert!(arr.equals(&[7, 8, 9]));

        let mut arr = [9, 8, 7];
        rng::sort_heap(&mut arr);
        assert!(arr.equals(&[7, 8, 9]));

        let mut arr = [9, 8, 7];
        arr.sort_heap();
        assert!(arr.equals(&[7, 8, 9]));

        let mut arr = [9, 8];
        arr.sort_heap();
        assert!(arr.equals(&[8, 9]));

        let mut arr = [9];
        arr.sort_heap();
        assert!(arr.equals(&[9]));

        let mut arr: [i32; 0] = [];
        arr.sort_heap();
        assert!(arr.equals(&[]));

        let mut arr = [9, 5, 4, 1, 1, 3, 2];
        arr.sort_heap();
        assert!(arr.equals(&[1, 1, 2, 3, 4, 5, 9]));
    }

    #[test]
    fn make_heap() {
        let mut arr = [5, 1, 1, 2, 9, 4];
        let start = arr.start();
        let end = arr.end();
        algo::make_heap_by(&mut arr, start, end, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = [5, 1, 1, 2, 9, 4];
        rng::make_heap_by(&mut arr, |x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = [5, 1, 1, 2, 9, 4];
        arr.make_heap_by(|x, y| x < y);
        assert!(arr.is_heap_by(|x, y| x < y));

        let mut arr = [5, 1, 1, 2, 9, 4];
        let start = arr.start();
        let end = arr.end();
        algo::make_heap(&mut arr, start, end);
        assert!(arr.is_heap());

        let mut arr = [5, 1, 1, 2, 9, 4];
        rng::make_heap(&mut arr);
        assert!(arr.is_heap());

        let mut arr = [5, 1, 1, 2, 9, 4];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr = [1, 1, 2, 4, 5, 9];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr = [9, 5, 4, 1, 1, 3, 2];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr = [1, 1, 2, 4, 5];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr = [1, 1, 2, 4];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr = [1, 1, 2];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr = [1, 1];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr = [1];
        arr.make_heap();
        assert!(arr.is_heap());

        let mut arr: [i32; 0] = [];
        arr.make_heap();
        assert!(arr.is_heap());
    }
}
