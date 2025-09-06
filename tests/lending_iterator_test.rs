// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn for_each() {
        let arr = [1, 2, 3, 4, 5];
        let mut sum = 0;
        arr.iter().for_each(|e| sum += e);
        assert_eq!(sum, 15);
    }

    #[test]
    fn for_each_until() {
        let arr = [1, 2, 3, 4, 5, 6];
        let mut sum = 0;
        arr.iter().for_each_until(|e| sum += e, |e| **e == 4);
        assert_eq!(sum, 6);
    }
}
