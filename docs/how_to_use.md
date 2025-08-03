# How to use

## Traversal

### Forward Traversal

It requires data structure to implement `Collection` trait.

Let's first understand, how to traverse collection with accessing elements:

immutable:

```rust
use stl::*;

let arr = [1, 2, 3];

let mut start = arr.start();
let end = arr.end();
while start != end {
    println!("{}", arr.at(&start));
    start = arr.next(start);
}
```

mutable:

```rust
use stl::*;

let mut arr = [1, 2, 3];

let mut start = arr.start();
let end = arr.end();
while start != end {
    *arr.at_mut(&start) = 4;
    start = arr.next(start);
}
```

### Backward traversal

It requires data structure to implement `BidirectionalCollection` trait.

```rust
use stl::*;

let arr = [1, 2, 3];

let start = arr.start();
let mut end = arr.end();
while start != end {
    end = arr.prior(end);
    println!("{}", arr.at(&end));
}
```

### Jumping from one position to other

```rust
use stl::*;

let arr = [1, 2, 3, 4, 5];

let mut i = arr.start(); // 0
let j = arr.next(i, 2); // 2
i = arr.prior_n(j, 2); // 0

let dist = arr.distance(i, j); // 2
```

If collection implements `RandomAccessCollection` trait, then `next_n`,
`prior_n` and `distance` are guaranteed to be O(1), otherwise it would be O(n).

## Algorithms

Algorithms are exposed as methods using extension traits. Thus `use stl::*` is
necessary to bring all extension traits to context.

Example of Non-Mutating algorithm:

```rust
use stl::*;

let mut sum = 0;
let arr = [1, 2, 3];
arr.for_each(|e| sum += e);
assert_eq!(sum, 6);
```

Example of Mutating algorithm:

```rust
use stl::*;

let mut arr = [1, 2, 3];
arr.for_each_mut(|e| *e += 1);
assert_eq!(arr, [2, 3, 4]);
```

### Slicing

Algorithms are there to work over collections. But many times it is important
to work over part of collections rather than full collections e.g, quick sort.

For tackling the same, every collection has slicing ability with `slice` method.
If collections supports mutation of any kind it also has `slice_mut` method.

For convenience `prefix_upto`, `suffix`, `full`, `prefix_upto_mut`, `suffix_mut`, `full_mut`
method is also exposed.

```rust
use stl::*;

let arr = [1, 2, 3, 4, 5];
let slice = arr.prefix_upto(3);
let odd_count = slice.count_if(|x| x % 2 == 1);
assert_eq!(odd_count, 2);
```

Slices can be used for basis of algorithm composition:

```rust
pub trait ReorderableCollectionExt: ReorderableCollection
where
    Self::Whole: ReorderableCollection,
{
    fn quick_sort(&mut self)
    where
        Self::Element: Ord,
    {
        if self.start() != self.end() {
            let p = self.partition_on_pos(&self.start(), |x, y| x < y); // assume a partition method
            self.prefix_upto_mut(p.clone()).quick_sort();
            self.suffix_mut(self.next(p)).quick_sort();
        }
    }
}
```
