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

    #[test]
    fn minmax_element() {
        let arr = [2, 1, 1, 2, 3, 3, 2];

        let (i, j) =
            algo::minmax_element_by(&arr, arr.start(), arr.end(), |x, y| x < y);
        assert_eq!(i, 1);
        assert_eq!(j, 5);

        let (i, j) =
            algo::minmax_element_by(&arr, arr.start(), arr.start(), |x, y| {
                x < y
            });
        assert_eq!(i, 0);
        assert_eq!(j, 0);

        let (i, j) = algo::minmax_element_by(
            &arr,
            arr.start(),
            arr.after(arr.start()),
            |x, y| x < y,
        );
        assert_eq!(i, 0);
        assert_eq!(j, 0);

        let (i, j) = rng::minmax_element_by(&arr, |x, y| x < y);
        assert_eq!(i, 1);
        assert_eq!(j, 5);

        let (i, j) = arr.minmax_element_by(|x, y| x < y);
        assert_eq!(i, 1);
        assert_eq!(j, 5);

        let (i, j) = algo::minmax_element(&arr, arr.start(), arr.end());
        assert_eq!(i, 1);
        assert_eq!(j, 5);

        let (i, j) = algo::minmax_element(&arr, arr.start(), arr.start());
        assert_eq!(i, 0);
        assert_eq!(j, 0);

        let (i, j) = rng::minmax_element(&arr);
        assert_eq!(i, 1);
        assert_eq!(j, 5);

        let (i, j) = arr.minmax_element();
        assert_eq!(i, 1);
        assert_eq!(j, 5);

        let arr = [1, 1];
        let (i, j) = arr.minmax_element();
        assert_eq!(i, 0);
        assert_eq!(j, 1);

        let arr = [2, 1];
        let (i, j) = arr.minmax_element();
        assert_eq!(i, 1);
        assert_eq!(j, 0);
    }
}
