#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    //adds itetools methods and tooling to all iterators in scope
    use itertools::Itertools;

    #[test]
    fn test_step_by() {
        let v: i32 = (0..10).step_by(3).sum();
        assert_eq!(18, v);
    }
}