use {
    crate::{Message, SandBox, Worker},
    clap::ArgMatches,
};

pub(crate) struct Cli<'a, 'b> {
    sand_box: &'a mut SandBox,
    matches: ArgMatches<'b>,
}

impl<'a, 'b> Cli<'a, 'b> {
    pub fn new(sand_box: &'a mut SandBox, matches: ArgMatches<'b>) -> Cli<'a, 'b> {
        Cli { sand_box, matches }
    }

    pub fn run(&mut self, cmd_name: &str) {
        match cmd_name {
            "thread" => self.handle_thread(),
            _ => (),
        }
    }

    fn handle_thread(&mut self) {
        let (cmd_name, _) = self.matches.subcommand();

        match cmd_name {
            "spawn" => self.handle_thread_spawn(),
            "terminate" => self.handle_thread_terminate(),
            "info" => self.handle_thread_info(),
            "send" => self.handle_thread_message(),
            _ => (),
        }
    }

    fn handle_thread_terminate(&mut self) {
        let (_, cmd) = self.matches.subcommand();

        let name = cmd
            .unwrap()
            .args
            .get("name")
            .unwrap()
            .vals
            .get(0)
            .unwrap()
            .clone()
            .into_string()
            .unwrap();

        let worker = self.sand_box.workers.remove(&name);

        let worker = match worker {
            None => return println!("Thread '{}' wasn't found.", name),
            Some(worker) => worker,
        };
        worker.sender.send(Message::Terminate).unwrap();
        println!("Thread '{}' was terminated.", name);
    }

    fn handle_thread_message(&mut self) {
        let (_, cmd) = self.matches.subcommand();

        let name = cmd.unwrap().args.get("name");
        let message = cmd.unwrap().args.get("message");

        let message = match message {
            None => return,
            Some(message) => message,
        };

        let message = message.vals.get(0).unwrap().clone().into_string().unwrap();

        if let None = name {
            for (_, worker) in &self.sand_box.workers {
                let message = message.clone();
                worker.sender.send(Message::Text(message)).unwrap();
            }
            return;
        }
        let name = name
            .unwrap()
            .vals
            .get(0)
            .unwrap()
            .clone()
            .into_string()
            .unwrap();

        let worker = self.sand_box.workers.get(&name);

        let worker = match worker {
            None => return println!("Thread '{}' wasn't found.", name),
            Some(worker) => worker,
        };
        worker.sender.send(Message::Text(message)).unwrap();
    }

    fn handle_thread_spawn(&mut self) {
        let (_, cmd) = self.matches.subcommand();

        let name = cmd
            .unwrap()
            .args
            .get("name")
            .unwrap()
            .vals
            .get(0)
            .unwrap()
            .clone()
            .into_string()
            .unwrap();

        if self.sand_box.workers.contains_key(&name) {
            return println!("Thread with name '{}' already exist!", name);
        }

        self.sand_box.create_worker(name.clone());
        println!("Thread {} created successfully\n", name);
    }

    fn handle_thread_info(&mut self) {
        let (_, cmd) = self.matches.subcommand();

        let name = cmd.unwrap().args.get("name");

        if let None = name {
            for (_, worker) in &self.sand_box.workers {
                Cli::print_thread_info(&worker);
            }
            return;
        }
        let name = name
            .unwrap()
            .vals
            .get(0)
            .unwrap()
            .clone()
            .into_string()
            .unwrap();

        let worker = self.sand_box.workers.get(&name);

        if let Some(worker) = worker {
            Cli::print_thread_info(&worker);
        }
    }

    fn print_thread_info(worker: &Worker) {
        let thread = worker.join_handle.thread();
        println!("name: {:?}", thread.name().unwrap());
        println!("id: {:?}", thread.id());
        println!("\n");
    }
}
