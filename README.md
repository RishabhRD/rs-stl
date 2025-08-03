# rs-stl [WIP]

Generic Programming for Rust Programming Language.

Rust has good set of algorithms based on rust iterators requiring just single
pass iteration. However, many algorithms like `rotate`, `reverse`, `sort`
might require multiple passes, backward iteration or random access to given
sequence.

Currently Rust lacks abstractions/traits to represent these kind of linear
sequences and thus generic algorithms over them. As a workaround, rust
implements algorithms like `sort` on inbuilt slice type `[T]` and use implicit
conversion trick for maintaining ergonomics.

rs-stl removes the need of these tricks and provides ways to write true generic
algorithms by introducing `Collection` trait and its refinements to empower
generic programming.

## Basic Idea

- C++ is a great language for generic programming.
- C++ uses `iterators` as basis of iteration over a given linear sequence.
- In C++, `iterators` are generalization of pointers. However, pointers are
  are not good for memory safe languages like rust.
- Thus rs-stl generalizes indexes as `Positions` and uses it as basis of iteration.
- The idea is, you can iterate a C-array using pointers (iterators) or using
  indexes (Positions).
- Because Positions doesn't depend on lifetime of collection, they are suitable
  for memory safe programming languages like rust.
- Slices should be first class citizen, that would allow working on a part of
  sequence without explicitly passing 2 positions to represent them. We call it
  slice first design.

## API Documentation

View detailed API documentation at: [rs-stl docs](https://rishabhrd.github.io/rs-stl/).

## Design

View design of rs-stl at: [rs-stl design](docs/design.md).

## How to use

For detailed usage see: [rs-stl how to use](docs/how_to_use.md).

## Sample Usage

rs-stl exposes methods for collections in stl module.

```rust
use stl::*;

let arr = [1, 2, 3, 4, 5];
let odd_count = arr.count_if(|x| x % 2 == 1);
assert_eq!(odd_count, 3);
```

For working on a contiguous part of collection,
use `slice`, `prefix_upto`, `suffix_from` or `full` method to obtain slice of collection.

```rust
use stl::*;

let arr = [1, 2, 3, 4, 5];
let slice = arr.prefix_upto(3);
let odd_count = slice.count_if(|x| x % 2 == 1);
assert_eq!(odd_count, 2);
```

## Support for Standard Library

Currently collection traits have been implemented for:

- `[T; N]`
- `[T]`
- `Vec<T>`
- `Option<T>`
- `Range<T>` (`a..b`) where T is an signed/unsigned integer type.
- `RangeInclusive<T>` (`a..=b`) where T is an signed/unsigned integer type.

In future, we plan to support more stdlib data structures.

## Motivations

The idea of generalization of indexes are not new and motivated from:

- [Swift Standard Library](https://developer.apple.com/documentation/swift/)
- [Hylo Programming Language](https://github.com/hylo-lang/hylo)
- [Flux](https://github.com/tcbrindle/flux)

## Credits

- Dave Abrahams ([@dabrahams](https://github.com/dabrahams))
  Current design of library specially slice first design was his idea only and
  was never possible without him.

## Contributions

Since the scope of project is quite big, surely needs contributions from other
experienced programmers. Feel free to contribute in terms of:

- Bugs
- Design
- Documents
- Feature Request
