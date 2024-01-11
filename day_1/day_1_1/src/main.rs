use std::env;
use std::fs::File;
use std::error::Error;
use std::process;

use std::io::{prelude::*, BufReader};

fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::build(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {err}");
      process::exit(1);
   });

   let sum = run(config).unwrap_or_else(|err| {
      println!("Error: {err}");
      process::exit(1);
   });

   println!("The sum of all of the calibration values is: {sum}");
   
}

fn run(config: Config) -> Result<i32, Box<dyn Error>> {
   let file = File::open(config.file_path)?;
   let reader = BufReader::new(file);
   let mut sum = 0;

   for line in reader.lines() {
      let line_as_chars: Vec<char> = line.unwrap_or_else(|err| {
         eprintln!("Error reading line: {}", err);
         String::new()
      }).chars().collect();

      let mut first_digit: Option<char> = None;
      let mut last_digit: Option<char> = None;

      for c in line_as_chars {
         if c.is_numeric() {
            if first_digit.is_none() {
               first_digit = Some(c);
            }
            last_digit = Some(c);
         }
        
      }

      let calibration_value = first_digit.map(|c| c.to_string()).unwrap_or_default() + &last_digit.map(|c| c.to_string()).unwrap_or_default();
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


