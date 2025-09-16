use std::process::{Child, Command, Stdio};
use std::path::Path;
use std::io::{stdout, Write};

fn main() {
    loop{
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command :Option<Child> = None;
        while let Some(command) = commands.next() {
            let mut parts = command.split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = std::env::set_current_dir(&root) {
                        eprintln!("cd: {}: {}", e, new_dir);
                    }
                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            eprintln!("{}", e);
                            previous_command = None;
                        },
                    }
                }

            }
        }

    }
}
