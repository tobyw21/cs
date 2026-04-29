# Subtyping and Variance

## Subtyping
A definition of subtype is if there is a type T and type U, T is the subtype of U if T is at least as useful of U.

`'static` is a subtype of any `'a`, because `'static` lives as long as `'a`.

## Covariance
Most things are covariant.
```Rust
fn foo(&'a str) {...}

foo(&'a str);       // valid
foo(&'static str);  // valid, 'static is a subtype of 'a (outlives 'a)
foo(&'b str);       // maybe invalid, if 'b is shorter than 'a (not a subtype of 'a)
```

Note that `foo` is not covariant, the parameter type is covariant.

Anything requires `&'a T`, a `&'static T` can be provided. In other words, provided reference can be longer than required reference's lifetime. Compiler is smart enough to coerce the longer lifetime to a shorter lifetime in this context.

This idea is not limited to lifetimes but mostly for lifetimes in Rust, in Java an example like, 
```Java
List<T> list = new ArrayList<>();
```
`ArrayList` is a subtype of `List` and at least as useful as `List`.

## Contravariance
Contravariance is only used on function pointers and closures arguments.

```Rust
// Those are not compilable Rust.
fn foo(f: Fn(&'static str)) {}

foo(fn(&'a str) {});    // valid, foo is expecting a function with parameter is less useful than &'static.
```

but if,
```Rust
fn foo(f: Fn(&'a str)) {}

foo(fn(&'static str) {});   // invalid, foo is expecting a less restrictive parameter but 'static is a subtype of 'a, which is more strict!
```

In other words, `Fn(&'static str)` requires a strict long lifetime string to be passed in. 
`Fn(&'a str)` requires a less strict lifetime parameter, it can be long, or can be short, doesn't matter as long as in the lifetime scope of `'a`. Thus, the function takes a shorter lifetime string is more useful. `Fn(&'a str)` is a subtype of `Fn(&'static str)`


## Invariance
`&'a mut T` invariant on `T`, look an example,
```Rust
fn foo(s: &mut &'a str, x: &'a str) {
    *s = x;
}

let z = String::new();
let mut x: &'static str = "hello world";
foo(&mut x, &z);
drop(z);
// Will this even print? (suppose the code is compilable)
println!("{}", x);

```

If foo make x points to z's address, and z later is free'd, then x points to a region of free'd memory. This is incorrect.

This is why `&mut T` is invariant, anything passes in must be the exact same as specified. No more no less.

as for why it is covariant in lifetime,
```Rust
fn bar() {
    let mut y: bool = true;
    let mut z /* : &'y mut bool */= &mut y;

    let mut x = Box::new(true);
    let mut x: &'static mut bool = Box::leak(x);

    z = x;  // let &'y mut bool = &'static mut bool, which is fine to shorten the lifetime of a mutable borrow.
}

```

When it is requiring a `&mut T` like `&mut &'static mut i32`, this `T = &'static mut i32`a pointer points to a reference with static lifetime, you must pass in exactly the same what it wants, because otherwise it may cause dangling pointer.

The lifetime is covariant is because compiler can shorten the lifetime, it is the same in covariance, longer lifetime is a subtype of a shorter lifetime, so it is always ok to do so.

In crates like `Serde`, a serializer can be implemented using `PhantomData` trick, and apply variance on them.
```Rust
struct Serializer<T> {
    // some fields...
    _t: PhantomData<T>, // look into `The Drop Check` chapter.
}

struct Serializer<T> {
    // some fields...
    _t: PhantomData<fn() -> T>, // This is covariant.
    // or a const raw pointer which is also covariant
    // _t: PhantomData<*const T>,
}

struct Serializer<T> {
    // some fields...
    _t: PhantomData<fn(T)>,     // This will make serializer contravariant, a longer lifetime can't be casted into shorter.
}

struct Serializer<T> {
    // some fields...
    _t1: PhantomData<fn(T)>,      
    _t2: PhantomData<fn() -> T>,    // This will make serializer invariant, no type can be both covariant and contravariant, variance is only 1 of 3.
    // or combine them
    // _t: PhantomData<fn(T) -> T>
    // or a lifetime can be introduced
    // _t: PhantomData<&'a mut T>
    // or use a raw pointer, but lose Send + Sync
    // _t: PhantomData<*mut T>
}

```

have a look with `#[may_dangle]` [here](https://std-dev-guide.rust-lang.org/tricky/may-dangle.html)