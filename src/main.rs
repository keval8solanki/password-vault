use std::io;
use std::process;

const COMMANDS: [&str; 6] = ["Add", "List", "Search", "Update", "Delete", "Exit"];

struct Cred {
    title: String,
    id: String,
    password: String,
}

fn print_box(text: &str) {
    let len = text.len();
    let line = "-".repeat(len + 4);

    println!("{}", line);
    println!("| {} |", text);
    println!("{}", line);
}

fn print_menu() {
    for (idx, item) in COMMANDS.iter().enumerate() {
        println!("{}: {}", idx + 1, item);
    }
}

fn read_input(message: &str) -> String {
    print_box(message);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read");
    return input;
}

fn validate_menu(input: u8) -> bool {
    let min_allowed_command: u8 = 1;

    let max_allowed_command: u8 = match COMMANDS.len().try_into() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return input >= min_allowed_command && input <= max_allowed_command;
}

fn runner(mut vault: Vec<Cred>) -> Vec<Cred> {
    print_menu();

    let command = read_input("Enter command");

    let parsed_input = match command.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let is_menu_valid = validate_menu(parsed_input);

    println!("{}", parsed_input);

    // Add
    if parsed_input == 1 {
        let title = read_input("Title");
        let id = read_input("ID");
        let password = read_input("Password");

        let cred: Cred = Cred {
            title,
            id, 
            password,
        };

        vault.push(cred);

        println!("Credentials added successfully");
        vault = runner(vault);
        return vault;
    }

    // List
    if parsed_input == 2 {
        for cred in &vault {
            println!("-----");
            println!("Title: {}", cred.title);
            println!("ID: {}", cred.id);
            println!("Password: {}", cred.password);
            println!("-----");
        }
        vault = runner(vault);
        return vault;
    }

    // Search
    if parsed_input == 3 {
        let title = read_input("Search by title");
        for cred in &vault {
            if cred.title == title {
                println!("-----");
                println!("Title: {}", cred.title);
                println!("ID: {}", cred.id);
                println!("Password: {}", cred.password);
                println!("-----");
                break;
            }
        }
        vault = runner(vault);
        return vault;
    }


    // Update
    if parsed_input == 4 {
        

    }

    if parsed_input == 6 {
        process::exit(0);
    }

    vault = runner(vault);
    return vault;
}

fn main() {
    let mut vault: Vec<Cred> = Vec::new();

    print_box("P A S S W O R D  V A U L T");

    runner(vault);
}
