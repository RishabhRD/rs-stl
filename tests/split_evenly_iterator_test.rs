// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn when_max_splits_works() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let mut res = vec![];
        for s in arr.splitting_evenly_in_with_min_size(2, 3) {
            let mut cur = vec![];
            s.iter().for_each(|e| cur.push(*e));
            res.push(cur);
        }
        assert_eq!(res, vec![vec![1, 2, 3, 4], vec![5, 6, 7]]);
    }

    #[test]
    fn when_max_splits_doesnt_work() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let mut res = vec![];
        for s in arr.splitting_evenly_in_with_min_size(3, 4) {
            let mut cur = vec![];
            s.iter().for_each(|e| cur.push(*e));
            res.push(cur);
        }
        assert_eq!(res, vec![vec![1, 2, 3, 4, 5, 6, 7]]);
    }

    #[test]
    fn without_max_size_constraint() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let mut res = vec![];
        for s in arr.splitting_evenly_in(3) {
            let mut cur = vec![];
            s.iter().for_each(|e| cur.push(*e));
            res.push(cur);
        }
        assert_eq!(res, vec![vec![1, 2, 3], vec![4, 5], vec![6, 7]]);
    }

    #[test]
    fn when_max_splits_works_mut() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        let mut res = vec![];
        for s in arr.splitting_evenly_in_with_min_size_mut(2, 3) {
            let mut cur = vec![];
            s.iter().for_each(|e| cur.push(*e));
            res.push(cur);
        }
        assert_eq!(res, vec![vec![1, 2, 3, 4], vec![5, 6, 7]]);
    }

    #[test]
    fn when_max_splits_doesnt_work_mut() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        let mut res = vec![];
        for s in arr.splitting_evenly_in_with_min_size_mut(3, 4) {
            let mut cur = vec![];
            s.iter().for_each(|e| cur.push(*e));
            res.push(cur);
        }
        assert_eq!(res, vec![vec![1, 2, 3, 4, 5, 6, 7]]);
    }

    #[test]
    fn without_max_size_constraint_mut() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        let mut res = vec![];
        for s in arr.splitting_evenly_in_mut(3) {
            let mut cur = vec![];
            s.iter().for_each(|e| cur.push(*e));
            res.push(cur);
        }
        assert_eq!(res, vec![vec![1, 2, 3], vec![4, 5], vec![6, 7]]);
    }
}
