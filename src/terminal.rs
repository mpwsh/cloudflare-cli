use colored::Colorize;
use text_io::read;

pub fn prompt(message: &str) -> String {
    println!("{}", message);
    read!("{}\n")
}

pub fn error(message: &str) {
    println!("{}", message.bright_red());
}

pub fn warn(message: &str) {
    println!("{}", message.bright_yellow());
}

pub fn info(message: &str) {
    println!("{}", message);
}
