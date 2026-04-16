use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct TimeOut<F> {
    inner: F,
    remaining: u8,
}

impl<F: Future> Future for TimeOut<F> {
    type Output = Result<F::Output, &'static str>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        let inner = unsafe { Pin::new_unchecked(&mut this.inner) };

        if let Poll::Ready(val) = inner.poll(cx) {
            return Poll::Ready(Ok(val));
        }

        if this.remaining == 0 {
            return Poll::Ready(Err("timeout"));
        }
        this.remaining -= 1;
        Poll::Pending
    }
}

fn main() {
    let success = TimeOut {
        inner: async { 42u32 },
        remaining: 5,
    };

    trpl::run(async {
        match success.await {
            Ok(val) => println!("Got: {val}"),
            Err(e) => println!("Error: {e}"),
        }
    });

    let timed_out = TimeOut {
        inner: std::future::pending::<u32>(),
        remaining: 0,
    };

    trpl::run(async {
        match timed_out.await {
            Ok(val) => println!("Got: {val}"),
            Err(e) => println!("Error: {e}"),
        }
    });
}
