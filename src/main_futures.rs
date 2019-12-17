use {
    futures::{executor::block_on, join},
    std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};

struct Timeout {
    thread: thread::JoinHandle<()>,
    ready: bool,
    maybe_waker: Option<Waker>,
}

impl Timeout {
    fn new() -> Timeout {
        let _thread = thread::spawn(move || {
            
        });

        Timeout { thread: _thread, ready: false, maybe_waker: None }
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.ready {
            println!("Ready");
            Poll::Ready(())
        } else {
            println!("Pending");
            let waker = cx.waker().clone();
            self.get_mut().maybe_waker = Some(waker);
            Poll::Pending
        }
    }
}

// fn set_timeout<F>(duration: Duration, cb: F) -> thread::JoinHandle<()>
// where
//     F: FnOnce() + Send + 'static,
// {
//     thread::spawn(move || {
//         thread::sleep(duration);
//         cb();
//     })
// }

// async fn hello_world(id: usize) {
//     let t1 = println!("sleep... {}", id);
//     thread::sleep(Duration::from_secs(5));
//     let t2 = println!("Hello, world!");
// }

async fn async_main() {
    let t = Timeout::new();
    join!(t);
    // let t1 = set_timeout(
    //     Duration::from_secs(5),
    //     Box::new(|| {
    //         println!("a");
    //     }),
    // );
    // let t2 = set_timeout(
    //     Duration::from_secs(5),
    //     Box::new(|| {
    //         println!("b");
    //     }),
    // );


    // join!(t1, t2);
    // t1.join();
    // t2.join();

    // let future1 = hello_world(1);
    // let future2 = hello_world(2);
    // join!(future1, future2);
}

fn main() {
    block_on(async_main());
}

// use std::{thread,time::Duration};

// fn main() {
//     let handle = thread::spawn(move || {
//         println!("thread");
//     });

//     handle.join();
// }
