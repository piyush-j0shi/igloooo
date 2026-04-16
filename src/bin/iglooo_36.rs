use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Delay {
    remaining: u8,
}

struct LoggingFuture<F> {
    inner: F,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready("done")
        } else {
            self.remaining -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl<F: Future> Future for LoggingFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner_data = unsafe { self.map_unchecked_mut(|s| &mut s.inner) };

        match inner_data.poll(cx) {
            Poll::Pending => {
                println!("Polling");
                Poll::Pending
            }
            Poll::Ready(val) => {
                println!("Completed");
                Poll::Ready(val)
            }
        }
    }
}

fn main() {
    trpl::block_on(async {
        let delay = Delay { remaining: 3 };
        let logging_future = LoggingFuture { inner: delay };

        println!("{}", logging_future.await);
    });
}
