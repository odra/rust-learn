use std::ops::AddAssign;

trait Rangeable:PartialOrd + Copy + AddAssign {} //wrap required traits into one
impl<T:PartialOrd + Copy + AddAssign> Rangeable for T {} //make it work for all T impl 

#[derive(Debug)]
struct GenericRangeIterator<T> {
    current: T,
    stop: T,
    step: T
}

impl<T> GenericRangeIterator<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        GenericRangeIterator {
            current: start,
            stop,
            step
        }
    }
}

impl<T:Rangeable> Iterator for GenericRangeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.stop {
            return None;
        }

        let res = self.current;
        self.current += self.step;

        Some(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_genericrangeiterator_int() {
        let mut sum: i32 = 0;
        let it = GenericRangeIterator::new(5, 12, 3);
        for item in it {
            sum += item;
        }

        assert_eq!(24, sum, "Test RangeIterator");
    }

    #[test]
    fn test_genericrangeiterator_float() {
        let mut sum: f32 = 0.;
        let it = GenericRangeIterator::new(5.2, 12.3, 3.5);
        for item in it {
            sum += item;
        }

        assert_eq!(26.099998, sum, "Test GenericRangeIterator");
    }

    #[test]
    fn test_filter() {
        let sum: i32 = GenericRangeIterator::new(5, 12, 3).filter(|n| n % 2 == 0).sum();
        assert_eq!(8, sum);
    }
}