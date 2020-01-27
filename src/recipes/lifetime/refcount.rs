use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct WithLife<'a> {
    s: &'a String
}

#[derive(Debug)]
struct NoLifeRO {
    s: Rc<String>,
}

#[derive(Debug)]
struct NoLifeRW {
    s: Rc<RefCell<String>>,
}

//this function doesn't work because it returns a reference that goes away within the function
// fn make_with_life<'a>(path: &str)  -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
//     let s = std::fs::read_to_string(path)?;
//     Ok((WithLife{s: &s}, WithLife{s: &s}))
// }

//a RC (RefCount) is a read only pointer
fn make_no_life(path: &str)-> Result<(NoLifeRO, NoLifeRO), std::io::Error> {
    let s = std::fs::read_to_string(path)?;
    let r = Rc::new(s); // creates a pointer
    //each clone creates a pointer to the same data
    Ok((NoLifeRO{s: r.clone()}, NoLifeRO{s: r.clone()}))
}

//a RefCell allows the pointer data to be changed (one borrow at a time)
fn make_no_life_rw(path: &str)-> Result<(NoLifeRW, NoLifeRW), std::io::Error> {
    let s = std::fs::read_to_string(path)?;
    let r = Rc::new(RefCell::new(s));// creates a writable pointer
    Ok((NoLifeRW{s: r.clone()}, NoLifeRW{s: r.clone()})) //each clone increments the pointer counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_no_life() {
        let (n1, n2) = make_no_life("test_data/data.txt").unwrap();
        assert_eq!("NoLifeRO { s: \"foobar\" }", format!("{:?}", n1));
        assert_eq!("NoLifeRO { s: \"foobar\" }", format!("{:?}", n2));
    }

    #[test]
    fn test_make_no_life_rw() {
        let (n1, n2) = make_no_life_rw("test_data/data.txt").unwrap();
        assert_eq!("NoLifeRW { s: RefCell { value: \"foobar\" } }", format!("{:?}", n1));
        assert_eq!("NoLifeRW { s: RefCell { value: \"foobar\" } }", format!("{:?}", n2));

        //borrows and modifies string from n1
        let mut s1 = n1.s.borrow_mut();
        //this does not work because a mutable (writable) borrow is happening
        //let s2 = n2.s.borrow();
        s1.push_str("stuff");
        drop(s1); // needs to be dropped (unborrow)

        //n2 changes as well since it points to the same data
        assert_eq!("NoLifeRW { s: RefCell { value: \"foobarstuff\" } }", format!("{:?}", n2));
    }
}