use std::io::{self, Write};

// Function to prompt the user for input
pub fn prompt(print_value: &str) -> io::Result<String> {
    print!("{} ", print_value);
    io::stdout().flush()?; // Flush the stdout to ensure prompt is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // Trim leading and trailing whitespace
}
