// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn join() {
        static ARR: [[i32; 3]; 3] = [[2, 1, 2], [2, 9, 4], [3, 1, 2]];

        let v = view::join(ARR.view());
        assert_eq!(v.count(&2), 4);

        let v = view::join(ARR.view()).as_reversed();
        assert_eq!(v.count(&2), 4);

        let v = ARR.view().join();
        assert_eq!(v.count(&2), 4);

        let v = ARR.view().join().as_reversed();
        assert_eq!(v.count(&2), 4);

        // TODO: add tests for the case, where a row is empty. currently not
        // possible because of static requirement.
    }
}
