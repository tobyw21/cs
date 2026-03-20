// str split with &str
// "a, b, c, d" => ["a", "b", "c", "d"]

pub struct Split<'s, 'd> {
    s: &'s str,
    delim: &'d str,
}

impl<'s, 'd> Split<'s, 'd> {
    pub fn new(s: &'s str, delim: &'d str) -> Self {
        Self { s, delim }
    }

    pub fn split(&self) -> Vec<char> {
        todo!()
    }

}