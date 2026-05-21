#[macro_export]
macro_rules! avec {
    // () => {
    //     Vec::new()
    // };
    // Allowing repetition, similar to regex, `*`, `+` and `?`
    // here it means, allowing a pattern with `expr,`*
    // in words an expr separated by comma, occurring more than 0 time(s).
    // The comma after the whole pattern means allowing 0 or 1 number of trailing comma.
    ($($element: expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut v = Vec::new();
        // Execute pattern many times that number of pattern == expansion time.
        $(v.push($element);)*
        v
    }};
    ($element: expr; $count: expr) => {{
        let count = $count;
        let mut v = Vec::with_capacity(count);
        // Sometimes element pass in is not a literal value,
        // it might be `something.take()`, if this is the case,
        // v.push() will not work properly because take() can only execute once.
        // it will expand to
        // for _ in 0..2 {
        //     v.push(x.take().unwrap());
        // }
        // which is 'taking' x for multiple times, that doesn't work, take can only
        // be called once.
        // let e = $element.clone();
        // for _ in 0..count {
        //     v.push(e);
        // }


        // v.resize(count, $element);

        v.extend(::std::iter::repeat($element).take(count));
        v
    }};
}

// Macros can be expanded recursively, but with limit or 128 times.
macro_rules! count {
    () => {
        0
    };
    ($head: tt, $($tail: tt),* $(,)*) => {
        1 + count!($($tail,)*)
    }
}

#[test]
fn count_num() {
    assert_eq!(count!(1, 2, 3, 4, 5,), 5);
}

/// ```compile_fail
///  println!(
///     "{}",
///     count!(
///         1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
///         2, 3, 4, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 11, 1, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
///         2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
///         1, 11, 1, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
///         3, 4, 1, 2, 3, 4, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 11, 1, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
///         3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 1, 1, 1, 1, 1,
///         1, 1, 1, 1, 11, 1, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3,
///         4, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 11, 1, 1, 2, 3, 4, 1, 2, 3,
///         4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 1, 1,
///         1, 1, 1, 1, 1, 1, 1, 11, 1, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4,
///         1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 11, 1, 1, 2, 3, 4,
///         1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0,
///         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 11, 1, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
///         2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 11, 1
///     )
/// );
/// ```


#[test]
fn empty() {
    let v: Vec<i32> = avec![];
    assert!(v.is_empty());
}

#[test]
fn one() {
    let v: Vec<i32> = avec![1];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 1);
}

#[test]
fn multiple() {
    let v: Vec<i32> = avec![1, 2, 3];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
}

#[test]
fn multiple_lines() {
    let v: Vec<_> = avec![
        "dsadjahdkjahkjdhakjdhkjahbfkjabjkafkahfkjahfkjabgakjbgakj",
        "dsadjahdkjahkjdhakjdhkjahbfkjabjkafkahfkjahfkjabgakjbgakj",
        "dsadjahdkjahkjdhakjdhkjahbfkjabjkafkahfkjahfkjabgakjbgakj",
        "dsadjahdkjahkjdhakjdhkjahbfkjabjkafkahfkjahfkjabgakjbgakj",
        "dsadjahdkjahkjdhakjdhkjahbfkjabjkafkahfkjahfkjabgakjbgakj",
    ];
}

#[test]
fn clone_2() {
    let v: Vec<i32> = avec![1; 2];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 1);
}

#[test]
fn clone_2_non_literal() {
    let mut x = Some(42);
    let v: Vec<i32> = avec![x.take().unwrap(); 2];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 42);
}

// Use `compile_fail` attribute in doctest to get an intentional compile fail test.

/// ```compile_fail
/// let v = avec![1; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFail;
