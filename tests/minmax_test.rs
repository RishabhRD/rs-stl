// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn min_element() {
        let arr = [2, 1, 3, 1];

        let i =
            algo::min_element_by(&arr, arr.start(), arr.end(), |x, y| x < y);
        assert_eq!(i, 1);

        let i =
            algo::min_element_by(&arr, arr.start(), arr.start(), |x, y| x < y);
        assert_eq!(i, 0);

        let i = rng::min_element_by(&arr, |x, y| x < y);
        assert_eq!(i, 1);

        let i = arr.min_element_by(|x, y| x < y);
        assert_eq!(i, 1);

        let i = algo::min_element(&arr, arr.start(), arr.end());
        assert_eq!(i, 1);

        let i = algo::min_element(&arr, arr.start(), arr.start());
        assert_eq!(i, 0);

        let i = rng::min_element(&arr);
        assert_eq!(i, 1);

        let i = arr.min_element();
        assert_eq!(i, 1);
    }

    #[test]
    fn max_element() {
        let arr = [1, 4, 3, 4, 2];

        let i =
            algo::max_element_by(&arr, arr.start(), arr.end(), |x, y| x < y);
        assert_eq!(i, 3);

        let i =
            algo::max_element_by(&arr, arr.start(), arr.start(), |x, y| x < y);
        assert_eq!(i, 0);

        let i = rng::max_element_by(&arr, |x, y| x < y);
        assert_eq!(i, 3);

        let i = arr.max_element_by(|x, y| x < y);
        assert_eq!(i, 3);

        let i = algo::max_element(&arr, arr.start(), arr.end());
        assert_eq!(i, 3);

        let i = algo::max_element(&arr, arr.start(), arr.start());
        assert_eq!(i, 0);

        let i = rng::max_element(&arr);
        assert_eq!(i, 3);

        let i = arr.max_element();
        assert_eq!(i, 3);
    }
}
