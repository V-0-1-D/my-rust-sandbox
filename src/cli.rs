use {
    crate::{SandBox, Worker},
    std::io::stdin,
};

pub(crate) struct Cli<'a> {
    sand_box: &'a mut SandBox,
    cmd: String,
    args: Vec<String>,
}

impl Cli<'_> {
    pub fn new(sand_box: &mut SandBox) -> Cli {
        let cmd = String::new();
        let args = Vec::new();
        Cli {
            sand_box,
            cmd,
            args,
        }
    }

    pub fn listen(&mut self) {
        let mut buffer = String::new();
        println!("Press 'help' to list commands for or 'q' for quit");

        loop {
            stdin().read_line(&mut buffer).unwrap();
            match &buffer[..buffer.len() - 1] {
                "q" => return,
                cmd => self.handle_root(cmd),
            }
            buffer.clear();
        }
    }

    fn print_command_not_found(&self) {
        println!("Command \"{}\" wasn't recognized.", self.cmd)
    }

    fn handle_root(&mut self, cmd: &str) {
        self.cmd = cmd.to_owned();

        self.args = cmd
            .split_whitespace()
            .map(|arg| arg.to_owned())
            .collect::<Vec<String>>();

        self.args.reverse();
        let arg1 = self.args.pop();

        let arg1 = match arg1 {
            Some(word) => word,
            None => return,
        };
        match &arg1[..] {
            "help" => print_help(),
            "thread" => self.handle_thread(),
            _ => self.print_command_not_found(),
        }
    }

    fn handle_thread(&mut self) {
        let arg2 = self.args.pop();

        let arg2 = match arg2 {
            Some(word) => word,
            None => return,
        };
        match &arg2[..] {
            "spawn" => self.handle_thread_spawn(),
            "info" => self.handle_thread_info(),
            _ => self.print_command_not_found(),
        }
    }

    fn handle_thread_spawn(&mut self) {
        let thread_name = self.args.pop();

        let thread_name = match thread_name {
            Some(word) => word,
            None => {
                println!("Expected thread name");
                return;
            }
        };
        self.sand_box.create_worker(thread_name.to_owned());
        println!("Thread {} created successfully", thread_name);
    }

    fn handle_thread_info(&mut self) {
        let thread_name = self.args.pop();

        if let Some(thread_name) = thread_name {
            let worker = self.sand_box.workers.get(&thread_name);
            if let Some(worker) = worker {
                Cli::print_thread_info(&worker);
            }
            return;
        }
        for (_, worker) in &self.sand_box.workers {
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

fn print_help() {
    println!(
        "
COMMANDS:

thread spawn <name>
thread info [<name>]
"
    );
}
