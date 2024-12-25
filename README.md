# rs-stl

Porting STL to rust.

Rust has great set of algorithms based on `rust iterators`. However, many
algorithms like `std::rotate` is still missing. This is because we don't have
concept of `C++-like iterators`.

C++ STL is a brilliant piece of work by Alex Stepanov and provides highly
composable algorithms over C++ iterators.

rs-stl ports C++ STL algorithms to rust by using concepts of Positions instead
of Iterators to support rust borrow rules.

## Documentation

View detailed documentation at: [rs-stl docs](https://rishabhrd.github.io/rs-stl/).

## Design

rs-stl is port of C++ STL to rust. C++ STL works over abstraction of iterators.
Iterators in C++ are generalization of pointers. However, pointers have
reference semantics so does iterators. Most STL algorithms need 2 iterators
to work upon. However, this model can't be adopted to rust. As an example,
in reverse example algorithm needs 2 mutable iterators to data structure.
However, then there would 2 mutable borrows from same data structure at
same time. That is not possible in rust.

Thus rs-stl works with idea of `Positions`. Positions are generalization of
indexes as iterators were generalization of pointers.

Considering example of array. In C++ there are 2 ways to traverse array `arr`:

1. Pointer -> \*arr, ++arr
2. Indexes -> arr[i], ++i;

As iterators are all about abstraction to traverse linear range, a similar
alternative is required. The above array example suggests, indexes are the
one. Which is generalized and is called `Position` in rs-stl. Position
type doesn't need to be integers and can be any type that follows below
trait requirements.

As from above example, its clear that Position doesn't have reference semantics.
Position doesn't borrow from data structure. Thus multiple positions can
be passed to algorithms. Also data structures are required to access
element at any position. Thus, this solution would not have problem of
dangling iterator.

rs-stl works with **ranges**. Range models linear sequence of elements.

```
  _ _ _ _ _

  ^          ^
  |          |
start       end
```

Every range has a `start` position, that is position of first element in range,
and an `end` position, that is position just after last element in range.

To get start and end position in range `rng`, `rng.start()`, `rng.end()`
can be used.

To access any element at position `i` in `rng` do:

- `rng.at(&i)` -> for immutable access
- `rng.at_mut(&i)` -> for mutable access

NOTE: end position can not be accessed for element with above methods.

To get to next position from current position `i` in range `rng`,
`rng.after(i)` can be used.

See the trait docs for more information.

## How to use

rs-stl supports following operation with ranges:

1. Algorithms (algo and rng module)

Please look at module docs for more information for the same.

#### Algorithms

Let's take an example of `std::count_if` in rs-stl, how to use this algorithm.

1. algo module
2. rng module

algo module contains algorithms which require position start and end to
be passed explicitly. Using that:

```rust
use stl::*;

let arr = [1, 2, 3];
let cnt = algo::count_if(&arr, arr.start(), arr.end(), |x| x % 2 == 1);
assert_eq!(cnt, 2);
```

rng module contains algo module algorithms overload, which doesn't need
start and end positions to be passed explicitly.

```rust
use stl::*;

let arr = [1, 2, 3];
let cnt = rng::count_if(&arr, |x| x % 2 == 1);
assert_eq!(cnt, 2);
```

For many algorithms rng module also provides infix overload inside rng::infix
module.

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3];
let cnt = arr.count_if(|x| x % 2 == 1);
assert_eq!(cnt, 2);
```

## Support for standard library

Currently range concepts have been implemented for:

- \[T, N\]
- \[T\]
- Vec<T>
  In future, we plan to support more data structures from standard library.

TODO: Not sure, how to deal with

- str
- String

correctly?

## Algorithms Implemented / Planned to Implement

The names mentioned here are C++ standard library names. If any algorithm is
implemented with other name as other name would be more suitable
then, that name would be mentioned explicitly in front of C++ name as well as
required description would be provided.

### Search Operations

- [x] find_if
- [x] find_if_not
- [x] find
- [x] count_if
- [x] count
- [x] all_of
- [x] any_of
- [x] none_of
- [x] mismatch -> mismatch_unbounded, mismatch_unbounded_by, mismatch, mismatch_by
- [ ] find_end
- [x] adjacent_find -> adjacent_find_if
- [x] equal -> equals, equals_by, equals_unbounded, equals_unbounded_by
- [ ] search
- [ ] search_n

### Copy Operations

- [x] copy
- [x] copy_if
- [x] copy_n
- [ ] ~~copy_backward~~ -> **NOT PLANNED:** Unlikely to be useful in rust?
- [ ] ~~move~~ -> **NOT POSSIBLE:** Due to rust ownership model.
- [ ] ~~move_backward~~ -> **NOT POSSIBLE:** Due to rust ownership model.

### Swap Operations

- [x] swap_ranges

### Transformation Operations

- [x] transform -> transform, zip_transform
- [x] replace
- [x] replace_if
- [x] replace_copy
- [x] replace_copy_if

### Generation Operations

- [x] fill -> fill_value
- [ ] ~~fill_n~~ -> **NOT PLANNED:** Use fill_value with slices.
- [x] generate -> fill_by
- [ ] ~~generate_n~~ -> **NOT PLANNED:** Use fill_by with slices.

### Removing Operations

- [x] remove
- [x] remove_if
- [x] remove_copy
- [x] remove_copy_if
- [x] unique -> unique, unique_by
- [x] unique_copy

### Order Changing Operations

- [x] reverse
- [x] reverse_copy
- [x] rotate
- [x] rotate_copy

### Partitioning Operations

- [x] is_partitioned
- [x] partition
- [ ] ~~partition_copy~~ -> Do we need that (copy_if is enough)? If yes, contribute with reason.
- [x] stable_partition
- [x] partition_point

### Sorting Operations

- [ ] sort
- [ ] stable_sort
- [ ] partial_sort
- [ ] partial_sort_copy
- [ ] is_sorted
- [ ] is_sorted_until
- [ ] nth_element

### Binary Search Operations

- [x] partition_point (Same as above partition_point)
- [x] lower_bound -> lower_bound, lower_bound_by
- [x] upper_bound -> upper_bound, upper_bound_by
- [x] equal_range -> equal_range, equal_range_by
- [ ] binary_search

### Merge Operations

- [x] merge -> merge, merge_by
- [x] inplace_merge -> merge_inplace, merge_inplace_by

### Heap Operations

- [x] push_heap -> push_heap, push_heap_by
- [x] pop_heap -> pop_heap, pop_heap_by
- [x] make_heap -> make_heap, make_heap_by
- [x] sort_heap -> sort_heap, sort_heap_by
- [x] is_heap -> is_heap, is_heap_by
- [x] is_heap_until -> is_heap_until, is_heap_until_by

### Minimum/Maximum Operations

- [x] min_element -> min_element, min_element_by
- [x] max_element -> max_element, max_element_by
- [x] minmax_element -> minmax_element, minmax_element_by

### Numeric Operations

- [ ] iota
- [ ] accumulate/reduce
- [ ] inner_product
- [ ] adjacent_difference
- [ ] partial_sum
- [ ] exclusive_scan
- [ ] inclusive_scan
- [ ] transform_reduce
- [ ] transform_exclusive_scan
- [ ] transform_inclusive_scan

### Permutation Operations

- [ ] shift_left
- [ ] shift_right
- [ ] next_permutation
- [ ] prev_permutation
- [ ] is_permutation

## Motivations

The idea of generalization of indexes are not new and motivated from:

- [Hylo Programming Language](https://github.com/hylo-lang/hylo)
- [Flux](https://github.com/tcbrindle/flux)
