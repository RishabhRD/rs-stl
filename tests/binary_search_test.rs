// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    pub fn lower_bound() {
        let arr = [2, 1, 4, 8, 7];
        let i =
            algo::lower_bound_by(&arr, arr.start(), arr.end(), &4, |x, y| {
                x < y
            });
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = rng::lower_bound_by(&arr, &4, |x, y| x < y);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = arr.lower_bound_by(&4, |x, y| x < y);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = algo::lower_bound(&arr, arr.start(), arr.end(), &4);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = rng::lower_bound(&arr, &4);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [2, 1, 4, 4, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [2, 1, 5, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [2, 1];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 2);

        let arr = [5, 8, 7];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 0);

        let arr = [];
        let i = arr.lower_bound(&4);
        assert_eq!(i, 0);
    }
}
