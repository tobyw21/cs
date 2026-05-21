# Static, Dynamic Dispatch and Fat Pointers

## Monomorphization

Rust creates multiple functions for the type T if it is used, for a function,
```Rust
fn do_something<T>(data: T) {...}
```
it will create stubs like
```Rust
fn do_something_i32(data: i32) {...}

fn do_something_str(data: &str) {...}

...

```
this could lead to binary bloat but fast, zero cost abstraction. 

Use `nm` to check symbols, can see something like
```
0000000000015a10 t _ZN7generic3foo17hb1fa428c062d660cE
0000000000015a90 t _ZN7generic3foo17hf4ace8e481d6aaadE
```


## Static dispatch
Because of monomorphization, type T can have a trait bound
```Rust
fn do_something<T>(data: T) -> T
    where T: std::fmt::Display {

}

fn do_something(data: impl std::fmt::Display) -> impl std::fmt::Display {}

```
but it can only take and return the same type and the type must be known at compile time

## Sized
Almost all types are sized, they implement the marker trait `std::marker::Sized`. 

Trait objects erased the concrete type, the size is unknown at compile time so they are not sized, hence they can't exist in the form of `dyn Trait`, it must be a pointer, either `&dyn Trait` or `Box<dyn Trait>`, when code is compiled, compiler needs to allocate correct stack size to the types, but Trait doesn't have a size, trait object has a dynamic size, it can be a byte or it can be a few Gigabytes. 

(almost?) under all circumstances, it is impossible to rebuild the concrete type from a trait object.

## Unsized
Represent by `?Sized`.

Types like `str`, `[u8]` are not sized, they can only occur with a reference, and the reference is a fat pointer.


## Dynamic dispatch
Dynamic dispatch allows different types to be stored in a container or be passed into functions, but the size is unknown, by using trait objects, 

```Rust
struct Person;
struct Cow;

trait Animal {}

impl Animal for Person {}
impl Animal for Cow {}

-----

let v: Vec<&dyn Animal> = ...


fn do_something(data: Box<dyn Animal>) {}

```
because all is determined at runtime which compiler doesn't know the size, it must be put on heap by a box or use as a reference, a fat pointer.

The scenario that can use dynamic dispatch is like a database record,
```SQL
CREATE TABLE Users (
    id int,
    user_name text,
    ...
)
```

```Rust
// assume Int, Text impl DataTypes
let record: Vec<Box<DataTypes>> = vec![Int{...}, Text{...}, ...];
```

## Fat pointer
Fat pointer is not only a pointer holds an address of a variable, it also contains some meta data like length, eg, `&str` is a fat pointer, it contains the actual string it points to and the length of the string, because the length must known at otherwise the pointer doesn't make sense.

```Rust
&dyn Trait
    // contains 2 pointers
    // 1. the concrete type it points to, the data.
    // 2. the vtable address compiler constructed.
```

## vtable
vtable is a table compiler construct during compile time, something like 
```Rust
// Those are not compilable Rust, they are for illustration.
struct TraitVtable {
    memberfunc: *mut Fn(*mut ()),
}

// if &str impls Trait
// the vtable would look like
struct TraitVtable {
    &<str as Trait>::memberfunc
}

```

every vtable contains concrete types's `Drop` implicitly.


## Limitations
- Multiple traits
    > The upper traits must be object safe.
- Associate types
    > Need to specify the associate type in the trait object `&dyn Trait<Target = i32>`.
- Generic trait
    > Need to specify the type generics in the trait Object `&dyn Trait<i32>`
- Static trait member
    > trait member doesn't have `&self` in signature, can use `Self: Sized` to be excluded by trait object.
- Generic method
    > Not object safe, it will create layers of layers generics and vtable will grow exponentially which is impossible to be constructed, example: `FromIterator<A>`.
- No non-receiver Self
    > trait object receiver must take `&self` cannot be `self` which is taking the ownership and not returning `Self`, it must know the size of concrete type in order to generate correct code.
- Disallowing trait objects
    + Disallow a trait to be trait object
    + Disallow member to be constructed within a trait object

