// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn zip_map_test() {
        let mut elements = [1, 2, 3];
        let mut weights = [2, 1, 3];

        view::zip_map(elements.view_mut(), weights.view_mut(), |_, y| *y)
            .sort_range();
        assert_eq!(elements, [2, 1, 3]);
        assert_eq!(weights, [1, 2, 3]);

        let mut elements = [1, 2, 3];
        let mut weights = [2, 1, 3];

        let v =
            view::zip_map(elements.view_mut(), weights.view_mut(), |x, y| {
                *x * *y
            })
            .as_reversed();
        assert!(v.equals(&[9, 2, 2]));

        let mut elements = [1, 2, 3];
        let mut weights = [2, 3, 3];
        view::zip_map(elements.view_mut(), weights.view_mut(), |x, y| *x * *y)
            .as_reversed()
            .sort_range();
        assert_eq!(elements, [3, 2, 1]);
        assert_eq!(weights, [3, 3, 2]);
    }
}
