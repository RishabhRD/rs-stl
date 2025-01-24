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

    #[test]
    fn test_inner_product_with_sum_and_multiplication() {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            0,
            |x, y| x * y,
            |a, b| a + b,
        );
        assert_eq!(result, 32); // 0 + (1*4) + (2*5) + (3*6) = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_inner_product_with_subtraction_and_multiplication() {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            0,
            |x, y| x * y,
            |a, b| a - b,
        );
        assert_eq!(result, -32); // 0 - (1*4) - (2*5) - (3*6) = -4 - 10 - 18 = -32
    }

    #[test]
    fn test_inner_product_with_different_initial_value() {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            10,
            |x, y| x * y,
            |a, b| a + b,
        );
        assert_eq!(result, 42); // 10 + (1*4) + (2*5) + (3*6) = 10 + 4 + 10 + 18 = 42
    }

    #[test]
    fn test_inner_product_with_empty_ranges() {
        let rng1: [i32; 0] = [];
        let rng2: [i32; 0] = [];
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            0,
            |x, y| x * y,
            |a, b| a + b,
        );
        assert_eq!(result, 0); // No elements, so result is the initial value
    }

    #[test]
    fn test_inner_product_with_single_element_ranges() {
        let rng1 = [3];
        let rng2 = [7];
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            1,
            |x, y| x * y,
            |a, b| a + b,
        );
        assert_eq!(result, 22); // 1 + (3*7) = 1 + 21 = 22
    }
    #[test]
    fn test_inner_product_with_mixed_operations() {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            10,
            |x, y| x * y,
            |a, b| a - b,
        );
        assert_eq!(result, -22); // 10 - (1*4) - (2*5) - (3*6) = 10 - 4 - 10 - 18 = -22
    }

    #[test]
    fn test_inner_product_with_rang1_is_shorter() {
        let rng1 = [1, 2, 3]; // Shorter range
        let rng2 = [4, 5, 6, 7]; // Longer range
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            0,
            |x, y| x * y,
            |a, b| a + b,
        );
        assert_eq!(result, 32); // 0 + (1*4) + (2*5) + (3*6) = 32
    }

    #[test]
    fn test_inner_product_with_rang2_is_shorter() {
        let rng1 = [1, 2, 3, 4]; // Longer range
        let rng2 = [5, 6, 7]; // Shorter range
        let start1 = rng1.start();
        let end1 = rng1.end();
        let start2 = rng2.start();

        let result = algo::inner_product(
            &rng1,
            start1,
            end1,
            &rng2,
            start2,
            0,
            |x, y| x * y,
            |a, b| a + b,
        );
        assert_eq!(result, 38); // 0 + (1*5) + (2*6) + (3*7) = 38
    }

    #[test]
    fn test_inner_product_with_sum_and_multiplication_for_rng() {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];

        let result =
            rng::inner_product(&rng1, &rng2, 0, |x, y| x * y, |a, b| a + b);
        assert_eq!(result, 32); // 0 + (1*4) + (2*5) + (3*6) = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_inner_product_with_subtraction_and_multiplication_with_subtraction()
    {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];

        let result =
            rng::inner_product(&rng1, &rng2, 0, |x, y| x * y, |a, b| a - b);
        assert_eq!(result, -32); // 0 - (1*4) - (2*5) - (3*6) = -4 - 10 - 18 = -32
    }

    #[test]
    fn test_inner_product_with_different_initial_value_and_multiplication() {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];

        let result =
            rng::inner_product(&rng1, &rng2, 10, |x, y| x * y, |a, b| a + b);
        assert_eq!(result, 42); // 10 + (1*4) + (2*5) + (3*6) = 10 + 4 + 10 + 18 = 42
    }

    #[test]
    fn test_inner_product_with_empty_ranges_and_mixed_operations() {
        let rng1: [i32; 0] = [];
        let rng2: [i32; 0] = [];

        let result =
            rng::inner_product(&rng1, &rng2, 0, |x, y| x * y, |a, b| a + b);
        assert_eq!(result, 0); // No elements, so result is the initial value
    }

    #[test]
    fn test_inner_product_with_single_element_ranges_and_multiplication() {
        let rng1 = [3];
        let rng2 = [7];

        let result =
            rng::inner_product(&rng1, &rng2, 1, |x, y| x * y, |a, b| a + b);
        assert_eq!(result, 22); // 1 + (3*7) = 1 + 21 = 22
    }
    #[test]
    fn test_inner_product_with_mixed_operations_rng() {
        let rng1 = [1, 2, 3];
        let rng2 = [4, 5, 6];

        let result =
            rng::inner_product(&rng1, &rng2, 10, |x, y| x * y, |a, b| a - b);
        assert_eq!(result, -22); // 10 - (1*4) - (2*5) - (3*6) = 10 - 4 - 10 - 18 = -22
    }

    #[test]
    fn test_inner_product_with_rang1_is_shorter_for_rng() {
        let rng1 = [1, 2, 3]; // Shorter range
        let rng2 = [4, 5, 6, 7]; // Longer range

        let result =
            rng::inner_product(&rng1, &rng2, 0, |x, y| x * y, |a, b| a + b);
        assert_eq!(result, 32); // 0 + (1*4) + (2*5) + (3*6) = 32
    }

    #[test]
    fn test_inner_product_with_rng2_is_shorter() {
        let rng1 = &[1, 2, 3, 4];
        let rng2 = &[5, 6, 7];

        let result =
            rng::inner_product(rng1, rng2, 0, |x, y| x * y, |a, b| a + b);

        assert_eq!(result, 38); // 0 + (1*5) + (2*6) + (3*7) = 38
    }
}
