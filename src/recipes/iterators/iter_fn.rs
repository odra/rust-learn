fn print_iter<S: AsRef<str>, I:IntoIterator<Item=S>>(v: I) -> String {
    let mut s = String::new();

    for (i, v) in v.into_iter().enumerate() {
        s.push_str(format!("|{}.{}|", i, v.as_ref()).as_str());
    }

    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print_iter() {
        let v = vec!["zero".to_owned(), "one".to_owned(), "two".to_owned()];
        assert_eq!("|0.zero||1.one||2.two|", print_iter(v.into_iter()));

        let v = vec!["zero", "one", "two"];
        assert_eq!("|0.zero||1.one||2.two|", print_iter(v.into_iter()));

        let v = &["zero", "one", "two"];
        assert_eq!("|0.zero||1.one||2.two|", print_iter(v.into_iter()));
    }
}