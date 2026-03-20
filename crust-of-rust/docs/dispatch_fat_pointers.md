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
this could lead to binary bloat but fast, zero cost abstration.


## Static dispatch
Because of monomorphization, type T can have a trait bound
```Rust
fn do_something<T>(data: T) -> T
    where T: std::fmt::Display {

}

fn do_something(data: impl std::fmt::Display) -> impl std::fmt::Display {}

```
but it can only return take and return the same type and the type must be known at compile time

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
because all is determined at runtime which compiler doesn't know the size, it must be put on heap by a box or use as a immutable reference, a fat pointer.

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

## vtable
