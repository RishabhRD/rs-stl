// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn range() {
        let arr = 1..5;
        assert_eq!(Collection::start(&arr), 1);
        assert_eq!(Collection::end(&arr), 5);
        assert_eq!(arr.next(2), 3);
        assert_eq!(arr.next_n(2, 2), 4);
        assert_eq!(arr.prior(2), 1);
        assert_eq!(arr.prior_n(2, 2), 0);
        assert_eq!(arr.compute_at(&2), 2);
        assert_eq!(*(&arr.at(&2) as &i32), 2);
        assert!(arr.all().equals(&[1, 2, 3, 4]));

        let arr = 1_i32..3;
        let mut sum = 0;
        for i in arr.iter() {
            sum += *i;
        }
        assert_eq!(sum, 3);
        assert_eq!(arr.lazy_iter().sum::<i32>(), 3);
    }

    #[test]
    fn range_inclusive() {
        let arr = 1..=5;
        assert_eq!(Collection::start(&arr), 1);
        assert_eq!(Collection::end(&arr), 6);
        assert_eq!(arr.next(2), 3);
        assert_eq!(arr.next_n(2, 2), 4);
        assert_eq!(arr.prior(2), 1);
        assert_eq!(arr.prior_n(2, 2), 0);
        assert_eq!(arr.compute_at(&2), 2);
        assert_eq!(*(&arr.at(&2) as &i32), 2);
        assert!(arr.all().equals(&[1, 2, 3, 4, 5]));

        let arr = 1_i32..=3;
        let mut sum = 0;
        for i in arr.iter() {
            sum += *i;
        }
        assert_eq!(sum, 6);
        assert_eq!(arr.lazy_iter().sum::<i32>(), 6);
    }
}
