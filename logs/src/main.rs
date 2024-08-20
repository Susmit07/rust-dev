use std::fs;
use std::io::Error;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            println!("{}", text_that_was_read.len())
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    }

    match divide(5.0, 3.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division)
        }
        Err(error_of_division) => {
            println!("{}", error_of_division)
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cant divide by 0"))
    } else {
        Ok(a / b)
    }
}
