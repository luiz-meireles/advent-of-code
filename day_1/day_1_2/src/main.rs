use std::env;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;

use std::io::{prelude::*, BufReader};

fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::build(&args).expect("Problem parsing arguments");

   let sum = run(config).expect("Error");

   println!("The sum of all of the calibration values is: {sum}");
   
}

fn run(config: Config) -> Result<i32, Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    let digit_mapping = get_digits_hash_map();
    let mut sum = 0;

    for line in reader.lines() {
        let line_as_chars: Vec<char> = line?.chars().collect();

        let mut first_digit: Option<i32> = None;
        let mut last_digit: Option<i32> = None;

        let mut set_first_and_last_digit = | key: &str | {
            if let Some(digit) = digit_mapping.get(key) {
                match first_digit {
                    None => first_digit = Some(*digit),
                    _ => (),
                }
                last_digit = Some(*digit);
            }
        };
        
        for i in 0..line_as_chars.len() {
            let key = line_as_chars[i].to_string();
            set_first_and_last_digit(&key);

            for j in i..line_as_chars.len() {
                let key = line_as_chars[i..j + 1].iter().collect::<String>();
                set_first_and_last_digit(&key);
            }
        }

        let first_digit_unwrapped = first_digit.unwrap_or_default().to_string();
        let last_digit_unwrapped = last_digit.unwrap_or_default().to_string();
        let calibration_value = first_digit_unwrapped + &last_digit_unwrapped;
        sum += calibration_value.parse().unwrap_or(0);
   }

   Ok(sum)
}

struct Config {
   file_path: String,
}

impl Config {
   fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 2 {
         return Err("not enough arguments");
      }

      let file_path = args[1].clone();
   
      Ok(Config { file_path })
   }
}


fn get_digits_hash_map() -> HashMap<String, i32> {
    let mut numbers = HashMap::new();

    numbers.insert("zero".to_string(), 0);
    numbers.insert("one".to_string(), 1);
    numbers.insert("two".to_string(), 2);
    numbers.insert("three".to_string(), 3);
    numbers.insert("four".to_string(), 4);
    numbers.insert("five".to_string(), 5);
    numbers.insert("six".to_string(), 6);
    numbers.insert("seven".to_string(), 7);
    numbers.insert("eight".to_string(), 8);
    numbers.insert("nine".to_string(), 9);

    for i in 0..10 {
        let key = format!("{}", i);
        numbers.insert(key, i);
    }

    numbers
}
