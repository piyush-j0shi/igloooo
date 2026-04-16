use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ThreePollResult {
    count: u8,
}

impl Future for ThreePollResult {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.count == 0 {
            Poll::Ready("Done")
        } else {
            this.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    trpl::block_on(async {
        let some_result = ThreePollResult { count: 2 }.await;
        println!("Completed : {}", some_result);
    });
}
