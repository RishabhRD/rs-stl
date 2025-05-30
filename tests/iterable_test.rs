// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn sum_by_iteration() {
        let arr = [1, 2, 3];
        let mut sum = 0;
        for i in CollectionExt::iter(&arr) {
            sum += i;
        }
        assert_eq!(sum, 6);
    }

    #[test]
    fn sum_by_lazy_iteration() {
        let arr = 1..=3;
        let mut sum = 0;
        for i in arr.lazy_iter() {
            sum += i;
        }
        assert_eq!(sum, 6);
    }
}
