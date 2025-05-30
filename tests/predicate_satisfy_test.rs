// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn all_satisfy() {
        let arr = [1, 3, 5];
        assert!(arr.all_satisfy(|x| x % 2 == 1));

        let arr = [1, 2, 5];
        assert!(!arr.all_satisfy(|x| x % 2 == 1));

        let arr = [];
        assert!(arr.all_satisfy(|x| x % 2 == 1));
    }

    #[test]
    fn any_satisfy() {
        let arr = [1, 2, 5];
        assert!(arr.any_satisfy(|x| x % 2 == 1));

        let arr = [2, 4, 6];
        assert!(!arr.any_satisfy(|x| x % 2 == 1));

        let arr = [];
        assert!(!arr.any_satisfy(|x| x % 2 == 1));
    }

    #[test]
    fn none_satisfy() {
        let arr = [2, 4, 6];
        assert!(arr.none_satisfy(|x| x % 2 == 1));

        let arr = [2, 1, 6];
        assert!(!arr.none_satisfy(|x| x % 2 == 1));

        let arr = [];
        assert!(arr.none_satisfy(|x| x % 2 == 1));
    }
}
