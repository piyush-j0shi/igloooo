use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Delay {
    remaining: u8,
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

struct MapFuture<F, Func> {
    inner: F,
    func: Option<Func>,
}

impl<F: Future, Func, Output> Future for MapFuture<F, Func>
where
    Func: FnOnce(F::Output) -> Output,
{
    type Output = Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.as_mut().map_unchecked_mut(|s| &mut s.inner) };

        match this.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(val) => {
                let this_func = unsafe { self.as_mut().get_unchecked_mut().func.take().unwrap() };
                Poll::Ready(this_func(val))
            }
        }
    }
}

fn main() {
    trpl::block_on(async {
        let delay = Delay { remaining: 3 };
        let map_fut = MapFuture {
            inner: delay,
            func: Some(|s: &str| format!("Transformed result: {}", s)),
        };

        let result = map_fut.await;
        println!("Final result: {}", result);
    });
}
