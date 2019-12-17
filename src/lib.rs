use std::{
    collections::HashMap,
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

mod cli;

#[derive(Debug)]
struct Worker {
    pub join_handle: JoinHandle<()>,
    pub sender: mpsc::Sender<String>,
    pub receiver: mpsc::Receiver<String>,
}

impl Worker {
    fn new(name: String) -> Worker {
        let join_handle = thread::Builder::new()
            .name(name)
            .spawn(move || loop {
                thread::sleep(Duration::from_secs(1));
            })
            .unwrap();

        let (sender, receiver) = mpsc::channel();

        Worker {
            join_handle,
            sender,
            receiver,
        }
    }
}

#[derive(Debug)]
pub struct SandBox {
    workers: HashMap<String, Worker>,
}

impl SandBox {
    pub fn new() -> SandBox {
        SandBox {
            workers: HashMap::new(),
        }
    }

    pub fn create_worker(&mut self, name: String) {
        let worker = Worker::new(name.clone());
        self.workers.insert(name, worker);
    }

    pub fn run(&mut self) {
        let mut cli = cli::Cli::new(self);
        cli.listen();
    }
}
