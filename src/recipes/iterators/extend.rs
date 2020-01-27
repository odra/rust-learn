struct SkipIterator<I: Iterator> {
    inner: I
}

impl<I, T> Iterator for SkipIterator<I> where I: Iterator<Item = T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()?;
        self.inner.next()
    }
}

//adds a skip_half function to all iterators
trait IterComb: Iterator + Sized {
    fn skip_half(self) -> SkipIterator<Self> {
        SkipIterator{inner: self}
    }
}
impl<I: Iterator + Sized> IterComb for I {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_skip() {
        let v: i32 = (0..10).skip_half().sum();
        assert_eq!(25, v);
    }
}