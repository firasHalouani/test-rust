use std::process::Command;
use std::io;

fn main() {
    println!("Enter a command:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // ⚠️ VULNERABLE: command injection
    Command::new("sh")
        .arg("-c")
        .arg(input)
        .output()
        .expect("failed to execute process");
}