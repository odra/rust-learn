#[derive(Debug)]
struct Person<'a> {
    name: &'a str
}

fn make_person<'a>(name: &'a str) -> Person<'a> {
    Person {name}
}

fn make_static_str() -> &'static str {
    "hello"
}

fn make_ref_str<'a>() -> &'a str {
    "hello"
}

fn modify_string(s: &mut String) {
    s.push_str(" world");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_static_str() {
        assert_eq!("hello", make_static_str());
    }

    #[test]
    fn test_make_ref_str() {
        assert_eq!("hello", make_ref_str());
    }

    #[test]
    fn test_modify_string() {
        let mut s = String::from("hello");
        modify_string(&mut s);
        assert_eq!("hello world", s);

        modify_string(&mut s);
        assert_eq!("hello world world", s);
    }

    #[test]
    fn test_make_person() {
        let name = "unnamed";
        let person = make_person(name);
        assert_eq!("Person { name: \"unnamed\" }", format!("{:?}", person))
    }
}