use std::io;

struct Character {
    name: String
}

fn list_characters(chars: & Vec<Character>) {
    println!("Current Characters:");
    for (idx, char) in chars.iter().enumerate() {
        println!("{}: {}", idx, char.name)
    }
    println!();
}

fn parse_u8() -> u8 {
    return parse_str().parse().expect("Input not an u8");
}

fn parse_str() -> String {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line.");
    return str.trim().to_owned();
}

fn add_character(chars: &mut Vec<Character>) -> bool {
    println!("Commands:\n\
        0: Add Character\n\
        1: Done");
    let cmd = parse_u8();
    if cmd == 0 {
        println!("Enter name:");
        chars.push(Character { name: parse_str() });
        return true;
    }
    return false;
}

fn add_characters(chars: &mut Vec<Character>) {
    println!("1) Add all combatants.\n");
    loop {
        list_characters(chars);
        if !add_character(chars) { break; }
    }
}

fn main() {
    println!("Welcome to the GURPS combat tracker.");
    let characters = &mut Vec::new();
    add_characters(characters);
    println!("{}", characters.len());
}
