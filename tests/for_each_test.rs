// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    fn assert_for_each<const N: usize>(arr: [i32; N], expected: i32) {
        let mut sum = 0;
        algo::for_each(&arr, arr.start(), arr.end(), |x| sum += *x);
        assert_eq!(sum, expected);

        let mut sum = 0;
        rng::for_each(&arr, |x| sum += *x);
        assert_eq!(sum, expected);

        let mut sum = 0;
        arr.for_each(|x| sum += *x);
        assert_eq!(sum, expected);
    }

    fn assert_for_each_mut<const N: usize>(rng: [i32; N], expected: [i32; N]) {
        let mut arr = rng;
        let start = arr.start();
        let end = arr.end();
        algo::for_each_mut(&mut arr, start, end, |x| *x += 1);
        assert_eq!(arr, expected);

        let mut arr = rng;
        rng::for_each_mut(&mut arr, |x| *x += 1);
        assert_eq!(arr, expected);

        let mut arr = rng;
        arr.for_each_mut(|x| *x += 1);
        assert_eq!(arr, expected);
    }

    #[test]
    fn for_each() {
        let arr = [1, 2, 3];
        assert_for_each(arr, 6);

        let arr = [0];
        assert_for_each(arr, 0);
    }

    #[test]
    fn for_each_mut() {
        let arr = [1, 2, 3];
        assert_for_each_mut(arr, [2, 3, 4]);

        let arr = [];
        assert_for_each_mut(arr, []);
    }
}
