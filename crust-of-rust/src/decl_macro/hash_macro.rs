use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $value: expr),* $(,)*) => {{
        const N: usize = $crate::count!(@COUNT; $($key),*);
        let mut hm = HashMap::with_capacity(N);
        $(hm.insert($key, $value);)*
        hm
    }};
    // This case doesn't make sense, it is just used as a practice.
    ($key: expr => $value: expr; $count: expr) => {{
        let count = $count;
        let mut hm = HashMap::with_capacity(count);
        hm.extend(::std::iter::repeat(($key, $value)).take(count));
        hm
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element: expr),*) => {
        // This is not alloc-ed on stack, `()` is a ZST.
        <[()]>::len(
            &[$($crate::count!(@SUBS; $element)),*]
        )
    };

    (@SUBS; $_element: expr) => {
        ()
    }
}

#[test]
fn empty() {
    let hm = hashmap! {"a" => 1};
    assert!(!hm.is_empty());
    assert_eq!(hm.len(), 1);
    assert_eq!(hm.get("a"), Some(&1));
}

#[test]
fn multiple() {
    let hm = hashmap! {"a" => 1, "b" => 2};
    assert!(!hm.is_empty());
    assert_eq!(hm.len(), 2);
    assert_eq!(hm.get("a"), Some(&1));
    assert_eq!(hm.get("b"), Some(&2));
}


#[test]
fn clone_2() {
    let hm = hashmap!["a" => 1; 2];
    assert!(!hm.is_empty());
    assert_eq!(hm.len(), 1);
    assert_eq!(hm.get("a"), Some(&1));
}