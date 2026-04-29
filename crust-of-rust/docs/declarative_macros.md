# Declarative Marcos
`macro_rules!`

Detailed explanation is in the [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)

## Common tricks
## extra `{}`
When trying to return value from macro, try use extra layer of `{}` to wrap it, when it expands without extra braces, it will look like...
```Rust

($($element: expr),+ $(,)?) => {{
    let mut v = Vec::new();
    $(v.push($element);)*
    v
}};
let v = let v = Vec::new(); $(v.push($element);)* v
```
which doesn't not compile.

## use `#[doc(hidden)]`
To measure the length of a macro expr is not easy, using hidden macro is a way to do it.


## Doctest
Can generate doc test by using
```Rust
/// ```
///  let x = 1;
/// ```
```
there are many attributes can be used in doc test and ways to use doc test is [here](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).
