use std::thread;
use std::time::Duration;

pub struct Timer<T> {
    delay : Duration,
    callback : T,
}

impl<T> Timer<T>
    where T: Fn() + Send + Sync + 'static {
    
    pub fn new(delay: Duration, callback: T) -> Self {
        Timer {
            delay: delay,
            callback: callback,
        }
    }

    pub fn start(self) {
        self.start_delayed(Duration::new(0, 0));
    }

    pub fn start_delayed(self, delay: Duration) {
        thread::spawn(move || {
            if delay.as_secs() != 0 {
                thread::sleep(delay);
            }

            loop {
                (self.callback)();
                thread::sleep(self.delay);
            }
        });
    }
}