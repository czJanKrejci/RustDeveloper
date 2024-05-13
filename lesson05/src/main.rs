// ty chybové hlášky si myslím, že chápu. Snad jsem správně pochopil i zadání.
// s csv funkcí jsem zase trochu "cheatoval", z hlavy bych to dokupy nedal


use slug::slugify;
use std::io;
use std::env;
use std::error::Error;
use std::fmt;
use std::io::Read;

fn main() {
    // Read input from standard input
    println!("Enter a string:");
    let mut stdin_input = String::new();
    io::stdin().read_to_string(&mut stdin_input).expect("Failed to read line");

    // Check only first argument of the CLI argument(s) and match it with corresponding function
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let result = match args[1].as_str() {
            "lowercase" => lowercase(stdin_input),
            "uppercase" => uppercase(stdin_input),
            "no-spaces" => no_spaces(stdin_input),
            "slugify" => myslugify(stdin_input),
            "capitalize" => capitalize(stdin_input),
            "csv" => csv(&stdin_input),
            _ => {
                eprintln!("Invalid argument: {}. No change to the string.", args[1]);
                return;
            }
        };
    // Print the result or error
    match result {
        Ok(output) => println!("Transformed text by {}: {}", args[1], output),
        Err(err) => eprintln!("Function {} encountered an error: {} ", args[1], err),
    }
    } else {
    println!("No command-line arguments provided. No change to the string.");
    }
}

fn lowercase(input: String) -> Result<String, Box<dyn Error>> {
    if input.trim().is_empty() {
        return Err("Input string is empty or contains only whitespace".into());
    }
    let lowercased: String = input.to_lowercase();
    Ok(lowercased)
}

fn uppercase(input: String) -> Result<String, Box<dyn Error>> {
    if input.trim().is_empty() {
        return Err("Input string is empty or contains only whitespace".into());
    }
    let uppercased: String = input.to_uppercase();
    Ok(uppercased)
}

fn no_spaces(input: String) -> Result<String, Box<dyn Error>> {
    if input.trim().is_empty() {
        return Err("Input string is empty or contains only whitespace".into());
    }
    let nospaced: String = input.replace(" ", "");
    Ok(nospaced)
}

fn myslugify(input: String) -> Result<String, Box<dyn Error>> {
    if input.trim().is_empty() {
        return Err("Input string is empty or contains only whitespace".into());
    }
    let slugified: String = slugify(input);
    Ok(slugified)
}

fn capitalize(input: String) -> Result<String, Box<dyn Error>> {
    if input.trim().is_empty() {
        return Err("Input string is empty or contains only whitespace".into());
    }

    // it is easier to convert the string into a list of characters
    let chars: Vec<char> = input.chars().collect();

    let capitalized: String = chars               // we take the list of the characters
        .windows(2)     // we view the list in windows of two, Hello -> [H,e] [e,l] [l,l], [l,o]
        .enumerate()    // we stick a numerical index to every window starting from 0
        .map(|(index, first_and_second)| {
            if index == 0 {
                // if this is the first character, uppercase it
                // these methods return something that should be converted to a string
                // because of crackhead languages
                // that do uppercase of one char as two chars
                first_and_second[0]
                    .to_uppercase()
                    .to_string()
                + &first_and_second[1]
                    .to_string()

            } else if first_and_second[0].is_whitespace() {
                // if the first character in this window is whitespace
                // capitalize and return the second one
                first_and_second[1]
                    .to_uppercase()
                    .to_string()

            } else {
                // if the first character is anything else,
                // just return the second character
                first_and_second[1]
                    .to_string()
            }
        })
        .collect::<String>();
    Ok(capitalized)
}

// Define a Csv struct to store headers and values
struct Csv {
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

impl Csv {
    // Implement a function to parse CSV data
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = Vec::new();
        let mut data = Vec::new();

        // Split input into lines
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err("Empty CSV input".into());
        }

        // Extract headers
        headers = lines[0].split(',').map(|s| s.to_string()).collect();

        // Extract data rows
        for line in &lines[1..] {
            let row: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            data.push(row);
        }

        Ok(Csv { headers, data })
    }
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the Csv data into an orderly table layout
        // Display headers
        for header in &self.headers {
            write!(f, "{:<16}", header)?;
        }
        writeln!(f)?;

        // Display data rows
        for row in &self.data {
            for item in row {
                write!(f, "{:<16}", item)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// Implement the csv operation function
fn csv(input: &str) -> Result<String, Box<dyn Error>> {
    // Parse CSV data
    let csv_data = Csv::parse(input)?;

    // Display the parsed CSV content
    Ok(csv_data.to_string())
}