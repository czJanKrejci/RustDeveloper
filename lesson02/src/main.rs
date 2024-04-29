use slug::slugify;
use std::io;
use std::env;

fn main() {
    // read input (what we want to change)
    println!("Enter a string to be transformed:");
        let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("You entered the string: \"{}\"", name);
    
    // parse CLI arguments (how we want to change it)
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Chosen argument(s):");
        for arg in &args[1..] {
            println!("{}", arg);
        }
    } else {
        println!("No command-line arguments provided. No change to the string.");
    }

    //transforming the string
    if args.len() > 1 {
        for arg in &args[1..] {
            if arg == "lowercase"{
                name = name.to_lowercase();
            }
            if arg == "uppercase"{
                name = name.to_uppercase();
            }
            if arg == "no-spaces"{
                name = name.replace(" ","");
            }
            if arg == "slugify"{
                name = name.slugify();
            }
            if arg == "capitalize"{
                name = capitalize(name);
            }
            if arg == "lowercase"{
                name = name.to_lowercase();
            }
        }
    }
    //output to user
    println!("Transformed string: \"{}\"", name);
    //arguments applied (in this order): pokud byla shoda
}

fn ft_strcapitalize(str: &mut str) -> &str {
    let mut i: usize = 0;
    let mut new_word: i32 = 1;

    while let Some(c) = str.chars().nth(i) {
        if new_word == 1 && c >= 'a' && c <= 'z' {
            str.as_bytes_mut()[i] -= 32;
        } else if new_word == 0 && c >= 'A' && c <= 'Z' {
            str.as_bytes_mut()[i] += 32;
        }
        if c < '0' || (c > '9' && c < 'A') || (c > 'Z' && c < 'a') || c > 'z' {
            new_word = 1;
        } else {
            new_word = 0;
        }
        i += 1;
    }
    str
}

