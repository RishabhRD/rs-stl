// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::*;

    #[test]
    pub fn subrange_test() {
        let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
        view::subrange(arr.view_mut(), 1, 4).stable_sort_by(|x, y| x.0 < y.0);
        assert_eq!(arr, [(3, 1), (1, 2), (1, 1), (4, 4), (5, 5)]);

        let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
        arr.view_mut()
            .subrange(1, 4)
            .stable_sort_by(|x, y| x.0 < y.0);
        assert_eq!(arr, [(3, 1), (1, 2), (1, 1), (4, 4), (5, 5)]);
    }
}
