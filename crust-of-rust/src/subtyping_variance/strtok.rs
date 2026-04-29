// What happens if the function signature is this?
// strtok<'a>(s: &'a mut &'a str, delim: char) -> Option<&'a str>
// this is taking a `&str` with a lifetime as same as the pointer,
// but what if we have a `&'static str`?

pub fn strtok<'a, 'b>(s: &'a mut &'b str, delim: char) -> Option<&'b str>
// It is even better to write like this, let compiler guess what's the lifetime of the reference.
// fn strtok<'s>(s: &'_ mut &'s str, delim: char) -> Option<&'s str>
// where 'b: 'a
//  ^ this is not needed because it doesn't matter what relationship of `'a` and `'b`
//    they are following the rule of `&'a mut T`
//    that covariant in `'a` and invariant in `T`, there is no association between 'a and 'b.
{
    // Implementation doesn't matter...
    if let Some(i) = s.find(delim) {
        let prefix = &s[..i];
        let suffix = &s[(i + delim.len_utf8())..];
        *s = suffix;
        Some(prefix)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut s: &'static str = "hello world";
        let x = strtok(&mut s, ' ');
        // strtok<'a, 'b>(s: &'a mut &'b str, delim: char) -> &'b str
        // what we pass in
        // strtok<'s, 'static>(&'s mut &'static, char) -> &'static str
        // which is totally fine, compiler can choose which to use, the
        // reference to `s` with a lifetime 's and the type is &'static str
        // that satisfy the variance in `&'a mut T` where it is covariant in 'a
        // but invariant in `T`.
        let _z = &mut s;  // z: &'z mut &'static str ---
        // Why does this ^^^^ still works?                                |
        // because of the covariance of `&'a mut T` in `'a`,              |
        // of course you can have a pointer with a shorter lifetime       |
        //                                      z's lifetime ends here <---
        // and z is not used below, but if...
        assert_eq!(x, Some("hello"));
        assert_eq!(s, "world");
        // printing z here is not allowed because there are overlap of a mut pointer
        // z's lifetime has been dragged longer because of this print
        // println!("{}", _z);  // This will not work!
    }

    #[test]
    fn long() {
        let mut s = "hello world my name is Ferris!";
        let x = strtok(&mut s, ' ');
        assert_eq!(x, Some("hello"));
        assert_eq!(s, "world my name is Ferris!");
        while let Some(_) = strtok(&mut s, ' ') {}
        assert_eq!(s, "Ferris!");
    }
}
