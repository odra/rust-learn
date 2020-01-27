#[derive(Debug)]
struct Person {
    pub name: String
}

impl Person {
    pub fn new(name: &str) -> Self  {
        Person {name: name.to_owned()}
    }
}

macro_rules! person {
    ($name:expr) => {
        Person::new($name)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_person() {
        let p = person!("myself");
        assert_eq!("Person { name: \"myself\" }", format!("{:?}", p));

        let name = "myname";
        let p = person!(name);
        assert_eq!("Person { name: \"myname\" }", format!("{:?}", p));
    }
}