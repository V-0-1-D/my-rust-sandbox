use {
    clap::{App, Arg, SubCommand},
    cli::Cli,
    std::{
        collections::HashMap,
        io::stdin,
        sync::mpsc,
        thread::{self, JoinHandle},
    },
};

mod cli;

enum Message {
    Text(String),
    Terminate,
}

#[derive(Debug)]
struct Worker {
    pub join_handle: JoinHandle<()>,
    pub sender: mpsc::Sender<Message>,
}

impl Worker {
    fn new(name: String) -> Worker {
        let (sender, receiver) = mpsc::channel();

        let join_handle = thread::Builder::new()
            .name(name.clone())
            .spawn(move || loop {
                let message = receiver.recv().unwrap();
                match message {
                    Message::Text(message) => {
                        println!("Thread '{}' got a message '{}'", name, message)
                    }
                    Message::Terminate => return,
                }
            })
            .unwrap();

        Worker {
            join_handle,
            sender,
        }
    }
}

pub struct SandBox {
    workers: HashMap<String, Worker>,
    pub(crate) app: App<'static, 'static>,
}

impl SandBox {
    pub fn new() -> SandBox {
        SandBox {
            app: SandBox::make_app(),
            workers: HashMap::new(),
        }
    }

    fn make_app() -> App<'static, 'static> {
        App::new("my_rust_sandbox")
            .version("0.0.1")
            .subcommand(SubCommand::with_name("quit").about("Quit the program"))
            .subcommand(
                SubCommand::with_name("thread")
                    .about("So stuff with threads")
                    .subcommand(
                        SubCommand::with_name("spawn")
                            .about("Spawn thread")
                            .arg(Arg::with_name("name").required(true)),
                    )
                    .subcommand(
                        SubCommand::with_name("info")
                            .about("Print thread info")
                            .arg(Arg::with_name("name")),
                    )
                    .subcommand(
                        SubCommand::with_name("terminate")
                            .about("Terminate thread")
                            .arg(Arg::with_name("name").required(true)),
                    )
                    .subcommand(
                        SubCommand::with_name("send")
                            .about("Send message to thread")
                            .arg(
                                Arg::with_name("name")
                                    .long("name")
                                    .help("Name of the thread")
                                    .required(true),
                            )
                            .arg(Arg::with_name("message").required(true)),
                    ),
            )
    }

    pub(crate) fn create_worker(&mut self, name: String) {
        let worker = Worker::new(name.clone());
        self.workers.insert(name, worker);
    }

    pub fn run(&mut self, buffer: &str) {
        let buffer = format!("my_rust_sandbox {}", buffer);
        let query = buffer.split_whitespace();

        let matches = self.app.clone().get_matches_from_safe(query);

        let matches = match matches {
            Err(x) => {
                println!("{}", x);
                return;
            }
            Ok(x) => x,
        };

        let (cmd_name, cmd) = matches.subcommand();
        let cmd = match cmd {
            None => {
                // got empty line
                return;
            }
            Some(x) => x.clone(),
        };

        let mut cli = Cli::new(self, cmd);
        cli.run(&cmd_name);
    }
}

impl Drop for SandBox {
    fn drop(&mut self) {
        for (_, worker) in &mut self.workers {
            worker.sender.send(Message::Terminate).unwrap();
        }
        for (_, worker) in self.workers.drain() {
            worker.join_handle.join().unwrap();
        }
    }
}

pub fn run() {
    println!("Press 'help' to list commands or 'quit' for quit");
    let mut sand_box = SandBox::new();

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        if buffer == "quit\n" {
            return;
        }
        sand_box.run(&mut buffer);
    }
}

#[cfg(test)]
mod tests {
    use crate::SandBox;

    const THREAD_NAME: &str = "foo";

    #[test]
    fn spawn_thread() {
        let mut sand_box = SandBox::new();
        sand_box.run(&format!("thread spawn {}\n", THREAD_NAME));
    }

    #[test]
    fn send_message_to_thread() {
        let mut sand_box = SandBox::new();
        sand_box.run(&format!("thread spawn {}\n", THREAD_NAME));
        sand_box.run(&format!("thread send test --name={}\n", THREAD_NAME));
    }
}
