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
    }
}
