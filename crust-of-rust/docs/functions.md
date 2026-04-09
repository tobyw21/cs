# Functions and closures

## Function item and function pointer
Function item does not have size, it is a unique marker to the function

```Rust
fn foo() {}

let f = foo;

```

function item can be coerced to function pointers, but not the other way around. 

```Rust
f();

fn baz(f: fn()) {}

baz(f);
```

## Traits: Fn(), FnMut() and FnOnce()

`FnOnce()` can only be called once, the trait signature is like `fn call(self)`, which takes the ownership of caller and it can't be happening twice.

`FnMut()` can be called multiple times and can mutate the data.

`Fn()` can be called multiple times and can't mutate anything.

`fn()` the function pointer, impls all 3 traits above.

for closures,

anything impls `Fn()` then automatically impls `FnMut()` and `FnOnce()`
because an exclusive reference and owned data can always produce a shared reference, which `Fn()` requires.

anything impls `FnMut()` then automatically impls `FnOnce()`, a owned data can produce an exclusive reference.

all closures impls `FnOnce()` 

a special move closure, `move || {}`, moves variables captured into closure, it doesn't make closure FnOnce, only act on variables.

## dyn Fn()
functions and closures can be used with dynamic dispatch, Fn* family are traits after all. Used to have `FnBox` trait but it is deprecated.

by using this, if requires `Fn()` bound, a `&dyn Fn()` is ok, if requires `FnMut()` bound, a `&mut dyn FnMut()` is needed, if `FnOnce()` is needed, must use `Box<dyn FnOnce()>`.

## const Fn()
const functions are evaluated during compile time.

main function is not requiring anything to be const in its body, but const fns would need it, an nightly feature using `~const FnOnce()` syntax indicating function foo is const if F's trait bound is satisfied, else it is not const,

```Rust
const fn foo<F: ~const FnOnce()>(f: F) {
    f();
}
```

however, in const function, const fn require everything calling inside its body to be const, so
```Rust
const fn test_foo() {
    let f = || {
        let _ = String::new();
    };

    foo(f);
}
```
is **NOT** working. Ideally it works like this but this is a very experimental feature.

above example is nightly feature `#![feature(const_trait_impl)]` using `miri`.

## for Bounds
for bounds are rarely used, they are used to specify, for example, lifetimes, 
```Rust
fn foo(f: F) where F: for<'a> Fn(&'a str) -> 'a str
```
but usually compiler can guess the lifetime like this easily, not very useful.