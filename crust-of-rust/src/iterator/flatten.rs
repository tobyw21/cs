// Flatten takes an iterable with iterables as elements
// and flat them
/*
    vec![vec![...], vec![...]]
    Iterator[IntoIterator[], IntoIterator[]]
*/
trait IteratorExt: Iterator {
    // For ergonomics, an extension iterator trait can be added on top of Iterator trait
    // to add a flatten function on top of it
    fn my_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
        Self: Sized;
}

// impl IteratorExt for all Iterators
impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn my_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        Flatten::new(self)
    }
}

fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    // Because we need to track both front and back of an iterator
    // in order to implement next and next_back
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

// Just a simple set up for struct.
impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Self {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // If front iterator has more elements
            if let Some(ref mut front_iter) = self.front_iter {
                // Get next element of front iter
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                // Front iterator exhaust elements
                self.front_iter = None;
            }
            // Current front iter doesn't have more elements.
            // If there is any outer iterator, assign it to front iter
            if let Some(outer_next) = self.outer.next() {
                self.front_iter = Some(outer_next.into_iter());
            } else {
                // Use back iterator's front elements
                // when front iter is all exhausted and back iterator is
                // not yet consumed
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator, // O has to be a DoubleEndedIterator to have next_back
    O::Item: IntoIterator,  // O's items should be able to be turned into iterators
    // O's items iterable should implement DoubleEndedIterator to be able to have next_back
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            // If back_iter is not None
            if let Some(ref mut back_iter) = self.back_iter {
                // If back iterator still has elements
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                // Back iterator exhaust elements
                self.back_iter = None;
            }

            // back_iter doesn't have elements, need to go to next_back in outer iterator
            if let Some(outer_next) = self.outer.next_back() {
                self.back_iter = Some(outer_next.into_iter());
            } else {
                // Use front iterator's back elements
                // when back iter is all exhausted and front iterator is
                // not yet consumed
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).next(), Some("a"));
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn slice() {
        let v: Vec<&[i32]> = vec![&[0, 1], &[2, 3], &[1]];
        assert_eq!(flatten(v).count(), 5);
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![1,2]].into_iter().my_flatten().count(), 2);
    }
}
