// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use rng::infix::*;
use stl::*;

#[test]
fn find_if_when_element_is_present() {
    let vec = vec![1, 2, 3];
    let p1 =
        algo::find_if(&vec, vec.start_position(), vec.end_position(), |x| {
            x % 2 == 0
        });
    let p2 = rng::find_if(&vec, |x| x % 2 == 0);
    let p3 = vec.find_if(|x| x % 2 == 0);

    assert_eq!(p1, 1);
    assert_eq!(p2, 1);
    assert_eq!(p3, 1);
}

#[test]
fn find_if_when_element_is_not_present() {
    let vec = vec![1, 3, 5];
    let p1 =
        algo::find_if(&vec, vec.start_position(), vec.end_position(), |x| {
            x % 2 == 0
        });
    let p2 = rng::find_if(&vec, |x| x % 2 == 0);
    let p3 = vec.find_if(|x| x % 2 == 0);

    assert_eq!(p1, 3);
    assert_eq!(p2, 3);
    assert_eq!(p3, 3);
}

#[test]
fn find_if_when_range_is_empty() {
    let vec = vec![];
    let p1 =
        algo::find_if(&vec, vec.start_position(), vec.end_position(), |x| {
            x % 2 == 0
        });
    let p2 = rng::find_if(&vec, |x| x % 2 == 0);
    let p3 = vec.find_if(|x| x % 2 == 0);

    assert_eq!(p1, 0);
    assert_eq!(p2, 0);
    assert_eq!(p3, 0);

    let vec2 = vec![1, 3, 5];
    let p = algo::find_if(
        &vec2,
        vec2.position_after(vec.start_position()),
        vec2.position_after(vec.start_position()),
        |x| x % 2 == 0,
    );
    assert_eq!(p, 1);
}
