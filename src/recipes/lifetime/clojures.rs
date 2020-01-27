#[derive(Debug)]
struct Hider {
    pub public: String,
    hidden: String,
    hidden_count: i32
}

impl Hider {
    pub fn new(public: String, hidden: String) -> Self {
        Hider {
            public,
            hidden,
            hidden_count: 0
        }
    }

    pub fn edit<F>(&mut self, f: F) where F:FnOnce(&mut String) {
        f(&mut self.hidden);
        self.hidden_count += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_hider_edit() {
        let mut h = Hider::new("showme".to_owned(), "hideme".to_owned());
        assert_eq!("hideme", h.hidden);
        assert_eq!(0, h.hidden_count);
        
        h.edit(|s| s.push_str("now"));
        assert_eq!("hidemenow", h.hidden);
        assert_eq!(1, h.hidden_count);
    }
}