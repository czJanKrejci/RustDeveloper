use std::io;

fn main() {
    println!("Enter your Rust nickname:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("Hello, {}, Rust adventurer!", name);
}
