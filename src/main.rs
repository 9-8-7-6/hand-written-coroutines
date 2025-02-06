use std::time::Instant;
use std::{thread, time::Duration};

mod future;
mod http;
use crate::future::{Future, PollState};
use crate::http::Http;

enum State {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine {
    state: State,
}

impl Coroutine {
    fn new() -> Self {
        Self {
            state: State::Start,
        }
    }
}

impl Future for Coroutine {
    type Output = ();
    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("Program is starting");
                    let fut = Box::new(Http::get("/600/HelloWorld1"));
                }
                State::Wait1(ref mut fut) => match fut.poll() {
                    PollState::Ready(txt) => {
                        println!("{txt}");
                        let fut2 = Box::new(Http::get("/400/HelloWorld2"));
                        self.state = State::Wait2(fut2);
                    }
                    PollState::NotReady => break PollState::NotReady,
                },
                State::Wait2(ref mut fut2) => match fut2.poll() {
                    PollState::Ready(txt2) => {
                        println!("{txt2}");
                        self.state = State::Resolved;
                        break PollState::Ready(());
                    }
                    PollState::NotReady => break PollState::NotReady,
                },
                State::Resolved => panic!("Polled a resolved future"),
            }
        }
    }
}

// control state machine by ourself.
// fn async_main() -> impl Future<Output = ()> {
//     Coroutine::new()
// }

async fn async_main() {
    println!("Program starting");
    let txt = Http::get("/1000/HelloWorld").await;
    println!("{txt}");
    let txt2 = Http::get("500/HelloWorld2").await;
    println!("{txt2}");
}

fn main() {
    let mut future = async_main();
    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("Schedule other tasks");
            }
            PollState::Ready(_) => break,
        }
        thread::sleep(Duration::from_millis(100));
    }
}
