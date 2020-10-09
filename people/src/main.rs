use std::fmt;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

struct Person {
    name: String,
    age: Option<i32>,
}

impl Person {
    fn new(name: String) -> Person {
        Person {
            name,
            age: None,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person({}", self.name)?;
        if let Some(n) = self.age {
            write!(f, ", {} years", n)?;
        }
        write!(f, ")")
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

#[derive(Debug)]
enum Error {
    MissingArg,
    UnknownCmd,
    ReadFailed,
    ParseFailed,
    NotFound,
    Exists,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::ParseFailed
    }
}

fn parse(cmd: String) -> Result<Command, Error> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() == 0 {
        return Err(Error::MissingArg);
    }

    match parts[0] {
        "add"    => {
            if parts.len() != 2 {
                return Err(Error::MissingArg);
            }
            Ok(Command::Add(parts[1].to_string()))
        }

        "age"    => {
            if parts.len() != 3 {
                return Err(Error::MissingArg);
            }
            Ok(Command::Age(parts[1].to_string(), parts[2].to_string()))
        }

        "rem"    => {
            if parts.len() != 2 {
                return Err(Error::MissingArg);
            }
            Ok(Command::Remove(parts[1].to_string()))
        }

        "show"   => Ok(Command::Show),
        "help"   => Ok(Command::Help),
        "quit"   => Ok(Command::Quit),

        _        => Err(Error::UnknownCmd),
    }
}

fn get_cmd() -> Result<Command, Error> {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    match io::stdin().read_line(&mut command) {
        Ok(_)   => parse(command.to_lowercase()),
        Err(_)  => Err(Error::ReadFailed),
    }
}

fn cmd_add(people: &mut Vec<Person>, name: String) -> Result<(), Error> {
    if people.iter().find(|p| p.name == name).is_some() {
        return Err(Error::Exists);
    }

    people.push(Person::new(name));
    Ok(())
}

fn cmd_age(people: &mut Vec<Person>, name: String, age: String) -> Result<(), Error> {
    let mut p = people.iter_mut().find(|p| p.name == name).ok_or(Error::NotFound)?;
    p.age = Some(age.parse::<i32>()?);
    Ok(())
}

fn cmd_remove(people: &mut Vec<Person>, name: String) -> Result<(), Error> {
    people.iter().find(|p| p.name == name).ok_or(Error::NotFound)?;
    people.retain(|p| p.name != name);
    Ok(())
}

fn cmd_show(people: &[Person]) -> Result<(), Error> {
    println!("[");
    for p in people {
        println!("  {}", p);
    }
    println!("]");
    Ok(())
}

fn cmd_help() -> Result<(), Error> {
    println!("The following commands are available:");
    println!("  add <name>");
    println!("  age <name> <age>");
    println!("  rem <name>");
    println!("  show");
    println!("  help");
    println!("  quit");
    Ok(())
}

fn main() {
    let mut people = Vec::<Person>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();

        let res = match command {
            Ok(Command::Add(n))         => cmd_add(&mut people, n),
            Ok(Command::Age(n, a))      => cmd_age(&mut people, n, a),
            Ok(Command::Remove(n))      => cmd_remove(&mut people, n),
            Ok(Command::Show)           => cmd_show(&people),
            Ok(Command::Help)           => cmd_help(),
            Ok(Command::Quit)           => break,
            Err(e)                      => Err(e),
        };

        if let Err(e) = res {
            println!("An error occurred: {:?}", e)
        }
    }
}
