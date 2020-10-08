use std::io;
use std::io::Write;

struct Book {}

enum Command {}

fn get_cmd() -> Command {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    // TODO parse line and create Command
    unimplemented!();
}

fn cmd_help() {
    println!("The following commands are available:");
    println!("  add <title>");
    println!("  year <title> <year>");
    println!("  rem <title>");
    println!("  show");
    println!("  help");
    println!("  quit");
}

fn main() {
    let mut books = Vec::<Book>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();

        // TODO handle command
    }
}
