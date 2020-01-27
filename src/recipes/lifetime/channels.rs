use std::time::Duration;
use std::sync::{Arc, Mutex};

fn with_arc() -> String {
    let content = Arc::new(Mutex::new(String::new()));
    let cloned = content.clone();
    
    std::thread::spawn(move || {
        let mut s = cloned.lock().unwrap();
        s.push_str("thread");
    });

    std::thread::sleep(Duration::from_millis(1000));

    let mut s = content.lock().unwrap();
    s.push_str("hello");
    
    String::from(s.as_str())
}

fn with_channels() {
    let (sender, receiver) = std::sync::mpsc::channel::<Box<dyn Fn(&mut String) + Send>>();
    let (done_s, done_r) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || {
        let mut hidden = String::new();
        loop {
            match receiver.recv() {
                Ok(f) => {
                    f(&mut hidden);
                    println!("{}", hidden);
                },
                Err(_) => {
                    done_s.send(()).unwrap();
                    return;
                }
            };
        }
    });

    sender.send(Box::new(|s: &mut String| {
        s.push_str("foo");
    })).unwrap();

    let sender2 = sender.clone();
    sender2.send(Box::new(|s: &mut String| {
        s.push_str("bar");
    })).unwrap();

    drop(sender);
    drop(sender2);

    done_r.recv().ok();
}

#[cfg(test)]
mod test {
    use super::*;

    struct Writer {
        s: String
    }

    unsafe impl Send for Writer {}

    impl Default for Writer {
        fn default() -> Self {
            Writer {s: String::new()}
        }
    }

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result { 
           self.s.push_str(_s);
           Ok(())
        }
    }

    #[test]
    fn test_with_arc() {
        assert_eq!("threadhello", with_arc());
    }

    #[test]
    fn test_with_channels() {
        //TODO: test something
        with_channels();
    }
}