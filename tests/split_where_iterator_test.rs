// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn split() {
        let arr = [1, 3, 5, 2, 2, 3, 4, 5, 5];
        let mut res = vec![];
        arr.splitting_where(|x| x % 2 == 0)
            .for_each(|s| res.push(s.iter().sum::<i32>()));
        assert_eq!(res, vec![9, 0, 3, 10]);
    }

    #[test]
    fn split_mut() {
        let mut arr = [1, 3, 5, 2, 2, 3, 4, 5, 7];

        {
            let mut res = vec![];
            arr.splitting_where_mut(|x| x % 2 == 0)
                .for_each(|s| res.push(s.iter().sum::<i32>()));
            assert_eq!(res, vec![9, 0, 3, 12]);
        }

        {
            arr.splitting_where_mut(|x| x % 2 == 0)
                .for_each(|mut s| s.reverse());
            assert_eq!(arr, [5, 3, 1, 2, 2, 3, 4, 7, 5]);
        }
    }
}
