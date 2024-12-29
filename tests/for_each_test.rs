// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn for_each() {
        let arr = [1, 2, 3];

        let mut sum = 0;
        algo::for_each(&arr, arr.start(), arr.end(), |x| sum += *x);
        assert_eq!(sum, 6);

        let mut sum = 0;
        rng::for_each(&arr, |x| sum += *x);
        assert_eq!(sum, 6);

        let mut sum = 0;
        arr.for_each(|x| sum += *x);
        assert_eq!(sum, 6);

        let arr: [i32; 0] = [];
        let mut sum = 0;
        arr.for_each(|x| sum += *x);
        assert_eq!(sum, 0);
    }

    #[test]
    fn for_each_mut() {
        let mut arr = [1, 2, 3];
        let start = arr.start();
        let end = arr.end();
        algo::for_each_mut(&mut arr, start, end, |x| *x += 1);
        assert_eq!(arr, [2, 3, 4]);

        let mut arr = [1, 2, 3];
        rng::for_each_mut(&mut arr, |x| *x += 1);
        assert_eq!(arr, [2, 3, 4]);

        let mut arr = [1, 2, 3];
        arr.for_each_mut(|x| *x += 1);
        assert_eq!(arr, [2, 3, 4]);

        let mut arr: [i32; 0] = [];
        arr.for_each_mut(|x| *x += 1);
        assert_eq!(arr, []);
    }
}
