
trait Greetable: std::fmt::Debug { //requires each implementation to implement the Debug trait
    fn greet(&self) -> String;
}

#[derive(Debug)]
struct English {}

impl Default for English {
    fn default() -> Self {
        English{}
    }
}

impl Greetable for English {
    fn greet(&self) -> String {
        "hello".to_owned()
    }
}

fn factory(indiom: &str) -> Option<impl Greetable> {
    match indiom {
        "english" => Some(English::default()),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factory() {
        let greet = factory("english").unwrap();
        assert_eq!("hello", greet.greet());
    }

    #[test]
    fn test_factory_notfound() {
        let greet = factory("portuguese");
        assert_eq!(true, greet.is_none());
    }
}