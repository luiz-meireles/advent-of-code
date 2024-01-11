use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).expect("Problem parsing arguments");

    let sum = run(config).expect("Error");

    println!(
        "The lowest location number that corresponds to any of the initial seed numbers is: {sum}"
    );
}

fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    let reader_iter = reader.lines().into_iter();

    let mut lines_iter = reader_iter.into_iter();
    let timing = lines_iter
        .next()
        .unwrap()?
        .clone()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()?;

    let distance = lines_iter
        .next()
        .unwrap()?
        .clone()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<f64>()?;

    let t_1 = (timing as f64 - f64::sqrt(timing.pow(2) as f64 - 4.0 * distance)) / 2.0;
    let t_2 = (timing as f64 + f64::sqrt(timing.pow(2) as f64 - 4.0 * distance)) / 2.0;

    println!("t_1: {}", t_1);
    println!("t_2: {}", t_2);

    Ok((t_2.ceil() - t_1.floor() - 1.0) as i64)
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
