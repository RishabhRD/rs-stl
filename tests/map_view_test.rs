// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    pub fn map_test() {
        let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
        view::map(arr.view_mut(), |x| x.0).sort_range();
        assert!(arr.is_sorted_by(|x, y| x.0 < y.0));

        let mut arr = [(3, 1), (1, 2), (4, 4), (1, 1), (5, 5)];
        arr.view_mut().map(|x| x.0).sort_range();
        assert!(arr.is_sorted_by(|x, y| x.0 < y.0));
    }
}
