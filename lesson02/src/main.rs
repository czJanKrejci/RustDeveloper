use slug::slugify;
use std::io;
use std::env;

fn main() {
    // Read input (what we want to change)
    println!("Enter a string to be transformed:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("You entered the string: \"{}\"", name);
    
    // Parse CLI arguments (how we want to change it)
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Chosen argument(s):");
        for arg in &args[1..] {
            println!("{}", arg);
            // Transform the string based on arguments
            name = &transform_string(&name, arg);
        }
    } else {
        println!("No command-line arguments provided. No change to the string.");
    }

    // Output to user
    println!("Transformed string: \"{}\"", name);
}

// Function to transform the string based on the given argument
fn transform_string(name: &str, arg: &String) -> String {
    match arg.as_str() {
        "lowercase" => name.to_lowercase(),
        "uppercase" => name.to_uppercase(),
        "no-spaces" => name.replace(" ",""),
        // Assuming slugify is meant to be slugified here
        "slugify" => slugify(&name),
        "capitalize" => capitalize(name),
        _ => {
            println!("Invalid argument: {}", arg);
            name.to_string() // Return original string if argument is invalid
        }
    }
}

// Function to capitalize the string
fn capitalize(name: &str) -> String {
    let mut result = String::new();
    let mut new_word = true;

    for c in name.chars() {
        if new_word && c.is_ascii_alphabetic() {
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c);
        }

        if c.is_ascii_whitespace() {
            new_word = true;
        } else {
            new_word = false;
        }
    }

    result
}