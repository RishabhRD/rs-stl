// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn count_where() {
        let arr = [1, 2, 3];
        let n = arr.count_where(|x| x % 2 == 1);
        assert_eq!(n, 2);

        let arr = [];
        let n = arr.count_where(|x| x % 2 == 1);
        assert_eq!(n, 0);
    }

    #[test]
    fn count_of() {
        let arr = [3, 2, 3];
        let n = arr.count_of(&3);
        assert_eq!(n, 2);

        let arr = [];
        let n = arr.count_of(&3);
        assert_eq!(n, 0);
    }
}
