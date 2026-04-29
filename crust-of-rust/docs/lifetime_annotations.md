# Lifetime Annotations

## Anonymous lifetime
An anonymous lifetime can only occur when circumstance like
`fn get_ref(&self) -> &str`, lifetime of returned `&str` is tied to `&self`,
which equals to `fn get_ref<'a>(&'a self) -> &'a str` or explicitly can be written as `fn get_ref<'a>(&'a self) -> &'_ str`.

## Static lifetime
Any literal `&str`, is `'static` and it outlives all lifetimes. You can assign `'static` lifetime strings anywhere, even in local variable, eg 
```Rust

fn get_str<'a>() -> &'a str {
    let s = "abc";
    // looks odd, returning a local variable?
    s
}


fn main() {
    println!("{:?}", get_str());
}
```
, because it is put in text segment during compilation, same as in C where `char *s = "something";`.

## Lifetime restrictions
Lifetime has restrictions like follows, 
```Rust
'a: 'b
T: 'a
T: 'a + 'b
```
1st means that lifetime 'a outlives lifetime 'b.

2nd means type T lifetime outlives lifetime 'a.

3rd means type T lifetime must outlive both lifetime 'a and 'b. 

It can also be written with Traits like `fn test<'a: 'b, T: 'b + Display>(item: T) {...}` which is lifetime 'a outlives 'b, and type T must have Display trait impl-ed also live at least as long as within the function `test`.

`T: 'a` vs `&'a T`, the 1st one means everything in T (T as a whole) must outlive lifetime 'a, latter means the reference, the pointer only lives as long as 'a.
