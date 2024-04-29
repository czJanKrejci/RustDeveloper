// Zkusil jsem kód napsat úplně sám, ale ani s pomocí rad kompilátoru jsem ho nedokázal opravit.
// I moje původní funkce na kapitalizaci, kterou jsem dříve vytvořil v C a přepsal do Rust, byla vyhodnocena jako "unsafe". 
// Vzhledem k tomu, že jsem nikdy jako programátor nepracoval, tak nějak počítám s tím, že místo v Braiins dostanou zkušenější kolegové.
// Proto jsem si vypomohl cheatem (Chat GPT), aby mi ukázal jak to má vypadat. Tak to sem píšu, ať je to fér.
// Vyhovuje mi učit se stylem vidět už hotový kód a postupně rozebírat funkcionalitu. Pak to pochopím.
// Budu rád, když vůbec dokončím kurz.

use slug::slugify;
use std::io;
use std::env;

fn main() {
    // Read input from standard input
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Trim whitespace

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Iterate over CLI arguments (skipping the first one, which is the program's name)
        for arg in &args[1..] {
            // Transmute text based on argument
            let result = match arg.as_str() {
                "lowercase" => input.to_lowercase(),
                "uppercase" => input.to_uppercase(),
                "no-spaces" => input.replace(" ", ""),
                "slugify" => slugify(input),
                "capitalize" => capitalize(input),
                // Add your own transformations here for bonus points
                _ => {
                    println!("Invalid argument: {}", arg);
                    input.to_string() // Return original input if argument is invalid
                }
            };
            // Print the transmuted text
            println!("Transformed text ({}): {}", arg, result);
        }
    } else {
        println!("No command-line arguments provided. No change to the string.");
    }
}

// Function to capitalize the string
fn capitalize(input: &str) -> String {
    let mut result = String::new();
    let mut new_word = true;

    for c in input.chars() {
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