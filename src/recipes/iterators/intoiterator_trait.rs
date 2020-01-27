struct Company {
    ceo: String,
    cfo: String,
    coo: String
}

struct CompanyIter<'a> {
    c: &'a Company,
    n: i32
}

impl<'a> Iterator for CompanyIter<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            self.n += 1;
            match self.n {
                1 => Some(&self.c.ceo),
                2 => Some(&self.c.cfo),
                3 => Some(&self.c.coo),
                _ => None
            }
        }
}

impl<'a> IntoIterator for &'a Company {
    type Item = &'a str;
    type IntoIter = CompanyIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CompanyIter{c: &self, n: 0}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intoiter() {
        let c = Company{
            ceo: "the ceo".to_owned(),
            cfo: "the cfo".to_owned(),
            coo: "the coo".to_owned()
        };
        
        let mut res = String::new();
        for m in &c {
            res.push_str(m);
        }
        assert_eq!("the ceothe cfothe coo", res);

        let mut res = String::new();
        for m in c.into_iter() {
            res.push_str(m);
        }
        assert_eq!("the ceothe cfothe coo", res);  
    }
}