use std::process::Command;
use std::io::{stdout, Write};

fn main() {
    loop{
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();
        child.wait().unwrap();
    }
}
