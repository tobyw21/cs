# Lifetime Annotations

## Anonymous lifetime
an anonymous lifetime can only occur when circumstance like
`fn get_ref(&self) -> &str`, lifetime of returned `&str` is tied to `&self`,
which equals to `fn get_ref<'a>(&'a self) -> &'a str` or explicitly can be written as `fn get_ref<'a>(&'a self) -> &'_ str`.

## Static lifetime
any literal `&str`, is `'static` and it outlives all lifetimes. You can assign `'static` lifetime strings anywhere, even in local variable, eg 
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