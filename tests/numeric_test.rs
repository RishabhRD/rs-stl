// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    fn concat_str(mut x: String, y: &&str) -> String {
        x.push_str(y);
        x
    }

    fn concat_str_1(x: &&str, mut y: String) -> String {
        y.push_str(x);
        y
    }

    #[test]
    fn fold_left() {
        let arr = ["Hello", ", ", "World!"];

        let res = algo::fold_left(
            &arr,
            arr.start(),
            arr.end(),
            String::from(""),
            concat_str,
        );
        assert_eq!(res, "Hello, World!");

        let res = rng::fold_left(&arr, String::from(""), concat_str);
        assert_eq!(res, "Hello, World!");

        let res = arr.fold_left(String::from(""), concat_str);
        assert_eq!(res, "Hello, World!");

        let res = arr.fold_left(String::from(" "), concat_str);
        assert_eq!(res, " Hello, World!");

        let arr: [&str; 0] = [];
        let res = arr.fold_left(String::from(" "), concat_str);
        assert_eq!(res, " ");
    }

    #[test]
    fn fold_right() {
        let arr = ["Hello", ", ", "World!"];

        let res = algo::fold_right(
            &arr,
            arr.start(),
            arr.end(),
            String::from(""),
            concat_str_1,
        );
        assert_eq!(res, "World!, Hello");

        let res = rng::fold_right(&arr, String::from(""), concat_str_1);
        assert_eq!(res, "World!, Hello");

        let res = arr.fold_right(String::from(""), concat_str_1);
        assert_eq!(res, "World!, Hello");

        let res = arr.fold_right(String::from(" "), concat_str_1);
        assert_eq!(res, " World!, Hello");

        let arr: [&str; 0] = [];
        let res = arr.fold_right(String::from(" "), concat_str_1);
        assert_eq!(res, " ");
    }

    #[test]
    fn inclusive_scan_inplace() {
        let mut arr = [1, 2, 3];
        let start = arr.start();
        let end = arr.end();
        algo::inclusive_scan_inplace(&mut arr, start, end, |x, y| x + y);
        assert!(arr.equals(&[1, 3, 6]));

        let mut arr = [1];
        let start = arr.start();
        let end = arr.end();
        algo::inclusive_scan_inplace(&mut arr, start, end, |x, y| x + y);
        assert!(arr.equals(&[1]));

        let mut arr = [1, 2, 3];
        rng::inclusive_scan_inplace(&mut arr, |x, y| x + y);
        assert!(arr.equals(&[1, 3, 6]));

        let mut arr = [1];
        rng::inclusive_scan_inplace(&mut arr, |x, y| x + y);
        assert!(arr.equals(&[1]));

        let mut arr = [1, 2, 3];
        arr.inclusive_scan_inplace(|x, y| x + y);
        assert!(arr.equals(&[1, 3, 6]));

        let mut arr = [1];
        arr.inclusive_scan_inplace(|x, y| x + y);
        assert!(arr.equals(&[1]));

        let mut arr: [i32; 0] = [];
        let start = arr.start();
        let end = arr.end();
        algo::inclusive_scan_inplace(&mut arr, start, end, |x, y| x + y);
        assert!(arr.equals(&[]));

        let mut arr: [i32; 0] = [];
        rng::inclusive_scan_inplace(&mut arr, |x, y| x + y);
        assert!(arr.equals(&[]));

        let mut arr: [i32; 0] = [];
        arr.inclusive_scan_inplace(|x, y| x + y);
        assert!(arr.equals(&[]));

        let mut arr = [1, 2, 3];
        let start = arr.start();
        let end = arr.end();
        algo::inclusive_scan_inplace(&mut arr, start, end, |x, y| x - y);
        assert!(arr.equals(&[1, -1, -4]));

        let mut arr = [1, 2, 3];
        rng::inclusive_scan_inplace(&mut arr, |x, y| x - y);
        assert!(arr.equals(&[1, -1, -4]));

        let mut arr = [1, 2, 3];
        arr.inclusive_scan_inplace(|x, y| x - y);
        assert!(arr.equals(&[1, -1, -4]));
    }

    #[test]
    fn inclusive_scan() {
        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let out = dest.start();
        let i = algo::inclusive_scan(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            out,
            |x, y| x + y,
        );
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 3, 6]));

        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let i = rng::inclusive_scan(&src, &mut dest, |x, y| x + y);
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 3, 6]));

        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let out = dest.start();
        let i = algo::inclusive_scan(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            out,
            |x, y| x - y,
        );
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, -1, -4]));

        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let i = rng::inclusive_scan(&src, &mut dest, |x, y| x - y);
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, -1, -4]));

        let src: [i32; 0] = [];
        let mut dest = [];
        let out = dest.start();
        let i = algo::inclusive_scan(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            out,
            |x, y| x + y,
        );
        assert_eq!(i, 0);
        assert!(dest.equals(&[]));

        let src: [i32; 0] = [];
        let mut dest = [];
        let i = rng::inclusive_scan(&src, &mut dest, |x, y| x + y);
        assert_eq!(i, 0);
        assert!(dest.equals(&[]));
    }

    #[test]
    fn exclusive_scan_inplace() {
        let mut arr = [2, 3, 7];
        let start = arr.start();
        let end = arr.end();
        algo::exclusive_scan_inplace(&mut arr, start, end, 0, |x, y| x + y);
        assert!(arr.equals(&[0, 2, 5]));

        let mut arr = [2, 3, 7];
        rng::exclusive_scan_inplace(&mut arr, 0, |x, y| x + y);
        assert!(arr.equals(&[0, 2, 5]));

        let mut arr = [2, 3, 7];
        arr.exclusive_scan_inplace(0, |x, y| x + y);
        assert!(arr.equals(&[0, 2, 5]));

        let mut arr = [2, 3, 7];
        let start = arr.start();
        let end = arr.end();
        algo::exclusive_scan_inplace(&mut arr, start, end, 0, |x, y| x - y);
        assert!(arr.equals(&[0, -2, -5]));

        let mut arr = [2, 3, 7];
        rng::exclusive_scan_inplace(&mut arr, 0, |x, y| x - y);
        assert!(arr.equals(&[0, -2, -5]));

        let mut arr = [2, 3, 7];
        arr.exclusive_scan_inplace(0, |x, y| x - y);
        assert!(arr.equals(&[0, -2, -5]));

        let mut arr = [2];
        let start = arr.start();
        let end = arr.end();
        algo::exclusive_scan_inplace(&mut arr, start, end, 0, |x, y| x + y);
        assert!(arr.equals(&[0]));

        let mut arr = [2];
        rng::exclusive_scan_inplace(&mut arr, 0, |x, y| x + y);
        assert!(arr.equals(&[0]));

        let mut arr = [2];
        arr.exclusive_scan_inplace(0, |x, y| x + y);
        assert!(arr.equals(&[0]));

        let mut arr: [i32; 0] = [];
        let start = arr.start();
        let end = arr.end();
        algo::exclusive_scan_inplace(&mut arr, start, end, 0, |x, y| x + y);
        assert!(arr.equals(&[]));

        let mut arr: [i32; 0] = [];
        rng::exclusive_scan_inplace(&mut arr, 0, |x, y| x + y);
        assert!(arr.equals(&[]));

        let mut arr: [i32; 0] = [];
        arr.exclusive_scan_inplace(0, |x, y| x + y);
        assert!(arr.equals(&[]));
    }

    #[test]
    fn exclusive_scan() {
        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let out = dest.start();
        let i = algo::exclusive_scan(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            out,
            1,
            |x, y| x + y,
        );
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 2, 4]));

        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let i = rng::exclusive_scan(&src, &mut dest, 1, |x, y| x + y);
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 2, 4]));

        let src: [i32; 0] = [];
        let mut dest = [];
        let out = dest.start();
        let i = algo::exclusive_scan(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            out,
            1,
            |x, y| x + y,
        );
        assert_eq!(i, 0);
        assert!(dest.equals(&[]));

        let src: [i32; 0] = [];
        let mut dest = [];
        let i = rng::exclusive_scan(&src, &mut dest, 1, |x, y| x + y);
        assert_eq!(i, 0);
        assert!(dest.equals(&[]));

        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let out = dest.start();
        let i = algo::exclusive_scan(
            &src,
            src.start(),
            src.end(),
            &mut dest,
            out,
            1,
            |x, y| x - y,
        );
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 0, -2]));

        let src = [1, 2, 3];
        let mut dest = [0, 0, 0];
        let i = rng::exclusive_scan(&src, &mut dest, 1, |x, y| x - y);
        assert_eq!(i, 3);
        assert!(dest.equals(&[1, 0, -2]));
    }
}
