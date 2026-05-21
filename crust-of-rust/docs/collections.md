# `std::collections`
not so many can be noted on this topic, they are just data structures, read [collections API doc](https://doc.rust-lang.org/std/collections/index.html).

## `Vec`
Things to note here is vec is reallocating memory if elements are constantly pushed into it, try use `with_capacity`, `swap_remove` etc to manipulate vector, they guarantee the performance from avoid realloc and memcpy.

## `VecDeque`
Pros:
- easy to push/pop front/back, very efficient, O(1) complexity.
- can act as a stack or queue

Cons:
- can't deref as a slice easily
- overhead when calculating index if start pointer and end pointer are wrapped around
- low CPU cache hit when it tries to read forward, high fragmentation

## `LinkedList`
Don't know why do we use it? read [this](https://rust-unofficial.github.io/too-many-lists/)

## Sets
Fundamentally, sets are using map with T as key and ZST as value.

Note that `OR, AND, XOR` operators are overloaded for getting intersection, diff and symmetric diff.

## Maps
`HashMap` has `entry` API that allows to do a check on key based and return Enum based on if the key in map is empty or occupied, then allow user to do their operations like insert, replace.

check [hashbrown](https://github.com/rust-lang/hashbrown) for a more powerful and performance implementation of hash map.

`BTreeMap`, Btree root stays in cache because it is hot, the higher the node is, the more efficient lookup is, or bsearch is.

## `BinaryHeap`
Just a binary heap implemented as a max-heap but can use `Reverse()` to achieve min-heap.