use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, stdin, BufReader};
use std::iter::zip;

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
    let timings = if let Some(Ok(line)) = lines_iter.next() {
        line.split(" ")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let distances = if let Some(Ok(line)) = lines_iter.next() {
        line.split(" ")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let mut margin_error = 1;

    let mut buffer = String::new();
    let stdin = stdin(); // We get `Stdin` here.

    for (t, d) in zip(timings, distances) {
        let mut possible_combinations = 0;
        for tn in 0..t + 1 {
            let v = if tn > 0 { t - tn } else { 0 };
            let s = v * tn;

            if s > d {
                possible_combinations += 1;
            }

            println!("{} {} {}", tn, v, s);

            stdin.read_line(&mut buffer)?;

            if buffer.trim() == "\n" {
                break;
            }
        }

        margin_error *= possible_combinations;
    }

    Ok(margin_error)
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
