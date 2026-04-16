use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Delay {
    remaining: u8,
    name: &'static str,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            println!("{} ready", self.name);
            Poll::Ready(self.name)
        } else {
            println!("{} pending {} left", self.name, self.remaining);
            self.remaining -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

struct JoinTwo<A, B>
where
    A: Future,
    B: Future,
{
    future_a: A,
    future_b: B,
    a_output: Option<A::Output>,
    b_output: Option<B::Output>,
}

impl<A: Future, B: Future> Future for JoinTwo<A, B> {
    type Output = (A::Output, B::Output);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.a_output.is_none() {
            let future_a = unsafe { Pin::new_unchecked(&mut this.future_a) };
            match future_a.poll(cx) {
                Poll::Pending => (),
                Poll::Ready(val) => this.a_output = Some(val),
            };
        }

        if this.b_output.is_none() {
            let future_b = unsafe { Pin::new_unchecked(&mut this.future_b) };
            match future_b.poll(cx) {
                Poll::Pending => (),
                Poll::Ready(val) => this.b_output = Some(val),
            };
        }

        if this.a_output.is_some() && this.b_output.is_some() {
            return Poll::Ready((this.a_output.take().unwrap(), this.b_output.take().unwrap()));
        }

        Poll::Pending
    }
}

fn main() {
    trpl::block_on(async {
        let future1 = Delay {
            remaining: 2,
            name: "A",
        };
        let future2 = Delay {
            remaining: 4,
            name: "B",
        };

        let joined = JoinTwo {
            future_a: future1,
            future_b: future2,
            a_output: None,
            b_output: None,
        };

        let result = joined.await;
        println!("result: {:?}", result);
    });
}
