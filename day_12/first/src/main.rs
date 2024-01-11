use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, stdin, BufReader};

use std::env;

fn _read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer.trim().to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).expect("Problem parsing arguments");

    let sum = run(config).expect("Error");

    println!(
        "The sum of the lengths of the shortest paths between every pair of galaxies is: {sum}"
    );
}

fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    let reader_iter = reader.lines().into_iter();

    for line in reader_iter {
        let line = line?;
        let mut line_iter = line.split(" ");
        let records = line_iter.next().unwrap().chars().collect::<Vec<char>>();
        let damaged_records = line_iter
            .next()
            .unwrap()
            .split(",")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        println!("records: {:?}", records);
        println!("damaged_records: {:?}", damaged_records);

        let mut next_range = 0;
        for r in damaged_records {
            for i in next_range..r {
                println!("i: {} -> {}", i, records[i as usize]);
            }
            next_range += r;
        }
    }
    Ok(0)
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].to_owned();

        Ok(Config { file_path })
    }
}
