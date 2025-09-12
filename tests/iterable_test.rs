// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn sum_by_iteration() {
        let arr = [1, 2, 3];
        assert_eq!(arr.iter().sum::<i32>(), 6);
    }

    #[test]
    fn sum_by_lazy_iteration() {
        let arr = 1..=3;
        assert_eq!(arr.lazy_iter().sum::<i32>(), 6);
    }
}
