fn foobar() -> Result<i32, failure::Error> {
    let lib = libloading::Library::new("libmath/libmath.so")?;

    unsafe {
        let f: libloading::Symbol<unsafe extern "C" fn(i32, i32)->i32> = lib.get(b"add")?;
        Ok(f(2, 2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foobar() {
        assert_eq!(4, foobar().unwrap());
    }
}