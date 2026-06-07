# STL

Some interesting libraries:
- Abseil
- Catch2 (test framework)
- gsl-lite


## Containers
Similar to `Rust`'s [collections](../crust-of-rust/docs/collections.md).


## Iterators
Denote that `A <: B` B is a subtype of A.

C++ has following iterator types.

I/O <: Forward <: Bidirectional <: Random Access <: Contiguous

the subtype has stronger capabilities.

`const_iterator`
- use `cbegin`, `cend`
- read only iterators

`reverse_iterator`
- use `rbegin`, `rend`
- iterate from back to beginning

remove from iterator,
either do
```cpp
// using erase_if
auto pred = [](int x){ return x % 2 == 0; };

std::erase_if(vec, pred);

// in a loop
for (auto it = vec.begin(); it != vec.end(); /* Don't increment iter here! */) {
    if (pred) {
        it = vec.erase(it);
    } else {
        it++;
    }
}

// using erase + remove_if
vec.erase(std::remove_if(vec.begin(), vec.end(), pred), vec.end());
```

## Concepts
Similar to `Rust`'s trait bound



## Lambda
`[capture](parameter) -> optional return type {code}`

the type of lambda function is `std::function<ReturnType(arg1, arg2, ...)>`

```cpp
int a = 1;
auto func = [a](int b){ return a < b; };
```

This can be used in template and predicate
```cpp
template <typename Predicate>
int test(Predicate pred) {
    int a = 1;
    if (...) {
        a = pred(...);
    }
    return a;
}

```

## Algorithms
Just a bunch of template functions act on containers, look into [`<algorithm>`](https://en.cppreference.com/cpp/header/algorithm) and [example code](../src/stl/algos.cc)