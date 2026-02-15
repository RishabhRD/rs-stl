// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::{collections::EmptyCollection, *};

    #[test]
    fn empty_collection() {
        let c: EmptyCollection<i32> = EmptyCollection::new();

        let mut i = ();
        let succ = c.form_next_n_limited_by(&mut i, 0, ());
        assert!(succ);

        let mut i = ();
        let succ = c.form_next_n_limited_by(&mut i, 2, ());
        assert!(!succ);

        let mut i = ();
        let succ = c.form_prior_n_limited_by(&mut i, 0, ());
        assert!(succ);

        let mut i = ();
        let succ = c.form_prior_n_limited_by(&mut i, 2, ());
        assert!(!succ);
    }
}
