// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn type_check() {
        let a = [1, 2, 3, 4, 5];
        let b: Slice<'_, ArraySlice<i32>> = a.full();
        let _: Slice<'_, ArraySlice<i32>> = b.full();

        let mut a = [1, 2, 3, 4, 5];
        let mut b: SliceMut<'_, ArraySlice<i32>> = a.full_mut();
        let _: SliceMut<'_, ArraySlice<i32>> = b.full_mut();

        let mut a = [1, 2, 3, 4, 5];
        let b: SliceMut<'_, ArraySlice<i32>> = a.full_mut();
        let _: Slice<'_, ArraySlice<i32>> = b.full();
    }
}
