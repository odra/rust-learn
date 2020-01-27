use std::sync::{mpsc, Arc, Mutex};

struct Pool {
    ch_s: Option<mpsc::Sender<Box<dyn Fn() + Send>>>,
    ch_done: mpsc::Receiver<()>,
    n: u32
}

impl Pool {
    pub fn new(n: u32) -> Self {
        let (ch_s, ch_r) = mpsc::channel();
        let (ch_done_s, ch_done_r) = mpsc::channel();
        let a = Arc::new(Mutex::new(ch_r));
        
        for _ in 0..n {
            let a2 = a.clone();
            let ch_done_s_clone = ch_done_s.clone();
            std::thread::spawn(move || loop {
                let m = a2.lock().unwrap();
                let f: Box<dyn Fn() + Send> = match m.recv() {
                    Ok(f) => f,
                    Err(_) => {
                        ch_done_s_clone.send(()).ok();
                        return;
                    },
                };
                drop(m);
                f();
            });
        }

        Pool {
            ch_s: Some(ch_s),
            ch_done: ch_done_r,
            n
        }
    }

    pub fn run<F:Fn() + Send + 'static>(&self, f: F) {
        if let Some(ref ch_s) = self.ch_s {
            ch_s.send(Box::new(f)).unwrap();
        }
    }

    pub fn wait(mut self) {
        self.ch_s.take();
        for _ in 0..self.n {
            self.ch_done.recv().unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pool() {
        let tp = Pool::new(10);
        for i in 0..10 {
            tp.run(move || {
                let data = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
                std::thread::sleep(std::time::Duration::from_millis(100));
                assert_eq!(true, data.contains(&i));
            });
        }
        tp.wait();
    }
}