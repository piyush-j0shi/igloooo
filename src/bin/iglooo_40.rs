use std::task::{Context, Poll, Waker};
use std::pin::Pin;
use std::future::Future;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct Delay {
    completed : Arc<Mutex<bool>>,
    waker_stored : Arc<Mutex<Option<Waker>>>,
    duration : Duration,
    started : bool,
}

impl Delay {
    fn new(duration : Duration) -> Self {
        Delay {
            completed: Arc::new(Mutex::new(false)),
            waker_stored: Arc::new(Mutex::new(None)),
            duration : duration,
            started: false,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if *self.completed.lock().unwrap() {
            return Poll::Ready(());
        }

        *self.waker_stored.lock().unwrap() = Some(cx.waker().clone());

        if !self.started {
            self.started = true;
            let completed = Arc::clone(&self.completed);
            let waker = Arc::clone(&self.waker_stored);
            let duration = self.duration;

            thread::spawn(move || {
                thread::sleep(duration);
                *completed.lock().unwrap() = true;

                if let Some(w) = waker.lock().unwrap().take() {
                    w.wake(); 
                }
            });
        }

        Poll::Pending 
    }
}
fn main() {}