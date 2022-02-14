use crate::spaces::spaces::SpaceName;
use std::{collections::HashSet, io::Write, str::FromStr};
use text_io::read;

pub fn input(message: &str) -> String {
    print!("{message} ");
    std::io::stdout().flush().unwrap();
    read!("{}\n")
}
pub fn input_int(message: &str) -> u64 {
    print!("{message} ");
    std::io::stdout().flush().unwrap();
    let line: String = read!("{}\n");
    match line.parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid integer");
            input_int(message)
        }
    }
}
pub fn input_space(message: &str, available: &HashSet<SpaceName>) -> SpaceName {
    match SpaceName::from_str(&input(message)) {
        Ok(space_name) => {
            if available.contains(&space_name) {
                space_name
            } else {
                println!("Invalid space name");
                input_space(message, available)
            }
        }
        Err(_) => {
            println!("Invalid unit location");
            input_space(message, available)
        }
    }
}

pub enum AttackingOrDefending {
    Attacking,
    Defending,
}
