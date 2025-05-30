// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn count_if() {
        let arr = [1, 2, 3];
        let n = arr.count_if(|x| x % 2 == 1);
        assert_eq!(n, 2);

        let arr = [];
        let n = arr.count_if(|x| x % 2 == 1);
        assert_eq!(n, 0);
    }

    #[test]
    fn count_element() {
        let arr = [3, 2, 3];
        let n = arr.count_element(&3);
        assert_eq!(n, 2);

        let arr = [];
        let n = arr.count_element(&3);
        assert_eq!(n, 0);
    }
}
