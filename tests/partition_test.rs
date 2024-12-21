// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn is_partitioned() {
        let arr = [1, 3, 5, 2, 4];
        assert!(algo::is_partitioned(&arr, arr.start(), arr.end(), |x| x
            % 2
            == 1));
        assert!(rng::is_partitioned(&arr, |x| x % 2 == 1));
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [2, 4];
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [1, 3];
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [];
        assert!(arr.is_partitioned(|x| x % 2 == 1));

        let arr = [1, 2, 3];
        assert!(!(arr.is_partitioned(|x| x % 2 == 1)));
    }
}
