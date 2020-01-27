//simple iterator
struct RangeIterator {
    current: i32,
    stop: i32,
    step: i32
}

impl RangeIterator {
    pub fn new(start: i32, stop: i32, step: i32) -> Self {
        RangeIterator {
            current: start,
            stop,
            step
        }
    }
}

impl Iterator for RangeIterator {
    type Item = i32;

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
    fn test_rangeiterator() {
        let mut sum: i32 = 0;
        let it = RangeIterator::new(5, 12, 3);
        for item in it {
            sum += item;
        }

        assert_eq!(24, sum, "Test RangeIterator");
    }
}