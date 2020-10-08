use std::io;
use std::io::Write;

struct Person {}

enum Command {}

fn get_cmd() -> Command {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    // TODO parse line and create Command
    panic!("Not implemented");
}

fn cmd_help() {
    println!("The following commands are available:");
    println!("  add <name>");
    println!("  age <name> <age>");
    println!("  rem <name>");
    println!("  show");
    println!("  help");
    println!("  quit");
}

fn main() {
    let mut people = Vec::<Person>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();

        // TODO handle command
    }
}
