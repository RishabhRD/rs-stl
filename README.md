# rs-stl

Porting STL to rust.

Rust has great set of algorithms based on `rust iterators`. However, many
algorithms like `std::rotate` is still missing. This is because we don't have
concept of `C++-like iterators`.

C++ STL is a brilliant piece of work by Alex Stepanov and provides highly
composable algorithms over C++ iterators.

rs-stl ports C++ STL algorithms to rust by using concepts of Positions instead
of Iterators to support rust borrow rules.

## Basic Idea

- STL provides generic algorithms with use of iterators. Iterators can be seen
  as generalization of pointers.
- With pointers one can traverse an array with accessing elements, that iterators
  generalized for any linear sequence of elements.
- However, using indexes in array one can achieve the same.
- Porting iterators to rust is not possible, as iterators borrows from range.
  And most algorithms requires 2 iterators. If mutation is required,
  rust borrow checker would not allow it.
- So, rs-stl generalizes indexes with Positions. And as index doesn't borrow
  from array, thus Position doesn't need to borrow from range.
- Then, STL algorithms accepting 2 iterators can be replaced with rs-stl
  algorithms accepting a range and 2 positions.

## End Goal

- rs-stl is not there to replace rust iterators. It is there to complement the same.
- rs-stl aims to be a go to library for algorithmic intensive programming problems.
- rs-stl focuses on inplace mutations with good ergonomics to achieve good performance easily with reasonable code.
- rs-stl aims to be inter-operable with rust iterators, so that one can go from one world to other easily.

## API Documentation

View detailed documentation at: [rs-stl docs](https://rishabhrd.github.io/rs-stl/).

## Design

View design at: [rs-stl design](./docs/design.md).

## Sample Usage

For detailed usage see: [rs-stl how to use](./docs/how_to_use.md).

### Algorithms

Using `algo` module:

```rust
use stl::*;

let arr = [1, 2, 3];
let cnt = algo::count_if(&arr, arr.start(), arr.end(), |x| x % 2 == 1);
assert_eq!(cnt, 2);
```

Using `rng` module:

```rust
use stl::*;

let arr = [1, 2, 3];
let cnt = rng::count_if(&arr, |x| x % 2 == 1);
assert_eq!(cnt, 2);
```

Using infix style of `rng` module:

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3];
let cnt = arr.count_if(|x| x % 2 == 1);
assert_eq!(cnt, 2);
```

See API documentation for support for all styles.

### Views

Views can be lazily composed to form new view. One can form

immutable view to range:

```rust
use stl::*;
use rng::infix::*;

let arr = [1, 2, 3];
let cnt = arr
           .view()
           .map(|x| *x + 1)
           .count_if(|x| x % 2 == 1);
assert_eq!(cnt, 1);
```

mutable view to range:

```rust
use stl::*;
use rng::infix::*;

let mut arr = [(1, 2), (2, 1)];
arr
  .view()
  .map(|x| x.1)
  .sort_range();
assert_eq!(arr, [(2, 1), (1, 2)]);
```

### As Iterators

InputRanges can also be used as Iterators. This is useful, for traversing any
InputRange using for loop.

```rust
use stl::*;
use rng::infix::*;

let mut sum = 0;
for e in view::single(2).iter() {
    sum += e;
}
assert_eq!(sum, 2);
```

If Iterators are not an option, use `for_each` algorithm for general traversal.

## Support for standard library

Currently range concepts have been implemented for:

- \[T, N\]
- \[T\]
- Vec<T>
- str
- String

In future, we plan to support more data structures from standard library.

## Contribution

The scope of project is quite large and surely need contributions from other
experienced programmers. Feel free to contribute in terms of:

- Bugs
- Design
- Documents
- Feature Request

[Goals](./docs/goals.md) lists future goals for project. The list might not
cover all use-case, feel free to raise feature request with supporting use-case.

## Motivations

The idea of generalization of indexes are not new and motivated from:

- [Hylo Programming Language](https://github.com/hylo-lang/hylo)
- [Flux](https://github.com/tcbrindle/flux)

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
- [x] equal -> equals, equals_by, equals_prefix, equals_prefix_by
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
- [x] for_each -> for_each, for_each_mut
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
- [x] stable_partition -> stable_partition, stable_partition_no_alloc
- [x] partition_point

### Sorting Operations

- [x] sort -> sort_range, sort_range_by
- [x] stable_sort -> stable_sort, stable_sort_by, stable_sort_no_alloc, stable_sort_by_no_alloc
- [x] partial_sort -> partial_sort, partial_sort_by
- [x] partial_sort_copy -> partial_sort_copy, partial_sort_copy_by
- [x] is_sorted -> is_sorted, is_sorted_by
- [x] is_sorted_until -> is_sorted_until, is_sorted_until_by
- [x] nth_element -> nth_element, nth_element_by

### Binary Search Operations

- [x] partition_point (Same as above partition_point)
- [x] lower_bound -> lower_bound, lower_bound_by
- [x] upper_bound -> upper_bound, upper_bound_by
- [x] equal_range -> equal_range, equal_range_by
- [x] binary_search -> binary_search, binary_search_by

### Merge Operations

- [x] merge -> merge, merge_by
- [x] inplace_merge -> merge_inplace, merge_inplace_by, merge_inplace_no_alloc, merge_inplace_by_no_alloc

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

Most of numeric algorithms can be implemented using views and copy operations.
Hence, it is not on priority list currently. Contributions are most welcome
for any of these algorithms.

- [ ] ~~iota~~ -> use view::ints
- [x] accumulate/reduce -> fold_left, fold_right
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

## Views to implement

NOTE: This is not an exhaustive list. If any view is not listed here, consider
raising a feature request for the same, or contribute the same :)

### View Factories

- [x] ints
- [x] generate
- [x] repeat
- [x] single
- [x] empty
- [x] maybe

### View adaptors

- [x] take
- [x] take_while
- [x] drop
- [x] drop_while
- [x] filter
- [x] map
- [x] subrange
- [x] prefix
- [x] suffix
- [x] as_reversed
- [x] cycle
- [ ] zip
- [ ] zip_map
- [ ] slide
- [ ] group_by
- [ ] split
- [ ] join
- [ ] cartesian_product
- [ ] scan
- [ ] scan_first
