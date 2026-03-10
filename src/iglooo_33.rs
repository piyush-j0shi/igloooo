use std::pin::Pin;
use std::task::{Context, Poll};

enum State {
    Start,
    Ready,
    Done,
}

struct ToggleFuture {
    state: State,
}

impl Future for ToggleFuture {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this.state {
            State::Start => {
                this.state = State::Ready;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::Ready => Poll::Ready("Ok"),
            State::Done => panic!("Future can not be polled once it is already polled"),
        }
    }
}

fn main() {
    trpl::block_on(async {
        let result = ToggleFuture {
            state: State::Start,
        }
        .await;

        println!("Completed: {}", result);
    });
}
