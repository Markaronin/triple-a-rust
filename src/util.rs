use std::io::Write;

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
