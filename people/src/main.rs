use std::io;
use std::io::Write;

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String) -> Person {
        Person {
            name,
            age: 0,
        }
    }
}

enum Command {
    Add(String),
    Age(String, String),
    Remove(String),
    Show,
    Help,
    Quit,
}

fn parse(cmd: String) -> Command {
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.len() == 0 {
        panic!("Unknown command");
    }

    match parts[0] {
        "add"    => {
            assert!(parts.len() == 2);
            Command::Add(parts[1].to_string())
        }

        "age"    => {
            assert!(parts.len() == 3);
            Command::Age(parts[1].to_string(), parts[2].to_string())
        }

        "rem"    => {
            assert!(parts.len() == 2);
            Command::Remove(parts[1].to_string())
        }

        "show"   => Command::Show,
        "help"   => Command::Help,
        "quit"   => Command::Quit,

        _        => panic!("Unknown command"),
    }
}

fn get_cmd() -> Command {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    parse(command)
}

fn cmd_add(people: &mut Vec<Person>, name: String) {
    people.push(Person::new(name));
}

fn cmd_age(people: &mut Vec<Person>, name: String, age: String) {
    let mut p = people.iter_mut().find(|p| p.name == name).unwrap();
    p.age = age.parse::<i32>().unwrap();
}

fn cmd_remove(people: &mut Vec<Person>, name: String) {
    people.retain(|p| p.name != name);
}

fn cmd_show(people: &Vec<Person>) {
    println!("[");
    for p in people {
        println!("  {} (Age {})", p.name, p.age);
    }
    println!("]");
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

        match command {
            Command::Add(n)         => cmd_add(&mut people, n),
            Command::Age(n, a)      => cmd_age(&mut people, n, a),
            Command::Remove(n)      => cmd_remove(&mut people, n),
            Command::Show           => cmd_show(&people),
            Command::Help           => cmd_help(),
            Command::Quit           => break,
        };
    }
}
