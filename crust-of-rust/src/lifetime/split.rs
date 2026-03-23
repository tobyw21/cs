// str split with &str
// "a, b, c, d" => split & collect => ["a", "b", "c", "d"]

// a split with generic delimiter.
pub struct Split<'s, D> {
    s: Option<&'s str>,
    delim: D,
}

impl<'s, D> Split<'s, D> {
    pub fn new(s: &'s str, delim: D) -> Self {
        Self { s: Some(s), delim }
    }
}

trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// implement a string delimiter
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // Use string find the delimiter location, return 
        // the start and end position of delimiter.
        s.find(self).map(|start| {
            (start, start + self.len())
        })
    }
}


// or we can even have a char delimiter implemented
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(start, _)| {
            (start, start + self.len_utf8())
        })
    }
}


// The idea of split is to abstract away the type of delimiter, not matter if its a 
// string, char or anything else, Delimiter trait only returns the start and end
// index of a delimiter, it doesn't care what type it is.

// Even split src can be a generic type in this way 
// as long as it can be splitted, but it is harder
// to implement.
impl<'s, D> Iterator for Split<'s, D> where D: Delimiter {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        // Need to modify remainder later, need a mutable reference to s
        // s: &'s str
        // need: &mut &'s str, is a pointer points to &str but mutable and can 
        // point to other &str's.

        let remainder = self.s.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delim.find_next(remainder) {
            // The actual string splited from original s.
            let until_delim = &remainder[..delim_start];

            // Point remainder to new remainder position
            *remainder = &remainder[delim_end..];
            Some(until_delim)
        } else {
            self.s.take()
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str() {
        let sp = Split::new("a,b,c,d", ",");
        let v: Vec<&str> = sp.collect();
        assert_eq!(v, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_char() {
        let sp = Split::new("a,b,c,d", ',');
        let v: Vec<&str> = sp.collect();
        assert_eq!(v, vec!["a", "b", "c", "d"]);
    }
}