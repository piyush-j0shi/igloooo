use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

enum State {
    Start,
    Waiting,
    Done,
}

struct Data {
    state: State,
}

impl Future for Data {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match this.state {
            State::Start => {
                println!("starting");
                this.state = State::Waiting;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::Waiting => {
                println!("Waiting");
                this.state = State::Done;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::Done => Poll::Ready("Done"),
        }
    }
}

fn main() {
    trpl::block_on(async {
        let some_date = Data {
            state: State::Start,
        }
        .await;
        println!("{}", some_date);
    })
}
