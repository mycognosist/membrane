use std::env;
use std::io::{self, BufRead};

fn main() {
    // vector containing cli arguments (selectors for matching)
    let args: Vec<String> = env::args().collect();

    let stdin = io::stdin();
    // iterate over each line received on stdin
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                // iterate over selectors
                for arg in &args {
                    let selector = arg.as_str();
                    if input.starts_with(selector) {
                        println!("{}", input)
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
