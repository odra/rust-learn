#[link(name="libmath", kind="static")]
extern "C" {
    fn add(n1: i32, n2: i32) -> i32;
}

fn rust_add(n1: i32, n2: i32) -> i32 {
    unsafe {
        add(n1, n2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        unsafe {
            assert_eq!(2, add(1, 1));
        }
    }
}