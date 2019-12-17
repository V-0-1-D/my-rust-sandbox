// use {
//     std::{
//         future::Future,
//         pin::Pin,
//         sync::{Arc, Mutex},
//         task::{Context, Poll, Waker},
//         thread,
//         time::Duration,
//     },
// };

// struct SharedState {
//     completed: bool,
//     waker: Option<Waker>,
// }

// pub struct TimerFuture {
//     shared_state: Arc<Mutex<SharedState>>,
// }

// fn main() {}

use std::boxed::Box;

fn main() {
    let b = Box::new(666);
    println!("{}", b);
}