// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn collection_of_one() {
        let mut c = CollectionOfOne::new(2);

        assert!(!c.start());
        assert!(c.end());
        assert_eq!(c.next(c.start()), c.end());
        assert_eq!(c.prior(c.end()), c.start());

        assert_eq!(*c.at(&c.start()), 2);
        *c.at_mut(&c.start()) = 3;
        assert_eq!(*c.at(&c.start()), 3);

        let c = CollectionOfOne::new(2);
        assert_eq!(c.iter().sum::<i32>(), 2);

        let mut s = c.all();
        s.drop_first();
        assert_eq!(s.iter().sum::<i32>(), 0);
    }

    #[test]
    fn empty_collection() {
        let c: EmptyCollection<i32> = EmptyCollection::new();
        assert_eq!(c.iter().sum::<i32>(), 0)
    }
}
