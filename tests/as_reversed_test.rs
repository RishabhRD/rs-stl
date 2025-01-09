// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn as_reversed_test() {
        let mut arr = [2, 1, 3];
        view::as_reversed(arr.view_mut()).sort_range();
        assert_eq!(arr, [3, 2, 1]);

        let mut arr = [2, 1, 3];
        arr.view_mut().as_reversed().sort_range();
        assert_eq!(arr, [3, 2, 1]);
    }
}
