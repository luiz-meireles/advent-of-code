use std::cmp;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Range;

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
    let mut seeds: Option<Vec<(i64, i64)>> = None;
    let mut maps = Vec::new();
    let mut last_map: Option<Map> = None;
    let mut reader_iter = reader.lines().into_iter();

    let mut lowest_seed_location = 0;

    while let Some(line) = reader_iter.next() {
        // let line = line?;
        match line? {
            x if x.is_empty() => {
                if last_map.is_some() {
                    let mapping = last_map.unwrap();
                    maps.push(mapping);
                    last_map = None;
                }
            }
            x if x.contains("seeds") => {
                let mut seeds_ranges = Vec::new();
                let mut numbers = x
                    .split(" ")
                    .filter_map(|x: &str| x.parse::<i64>().ok())
                    .into_iter();
                while let Some(start) = numbers.next() {
                    let end = numbers.next().unwrap();
                    // println!("{}..{}", start, start + end);
                    seeds_ranges.push((start, start + end - 1));
                }
                seeds = Some(seeds_ranges);
            }
            x if x.ends_with("map:") => {
                let map_name = x.replace("map:", "").replace("-", "").trim().to_string();

                let mut mapping = map_name.split("to").into_iter();
                let from = mapping.next().unwrap().to_string();
                let to = mapping.next().unwrap().to_string();

                last_map = Some(Map {
                    from,
                    to,
                    ranges: Vec::new(),
                })
            }

            x => {
                let numbers: Vec<i64> = x.split(" ").map(|n| n.trim().parse().unwrap()).collect();
                let range = MapRange {
                    dst: numbers[0],
                    src: numbers[1],
                    range: numbers[2],
                };

                if last_map.is_some() {
                    last_map.as_mut().unwrap().ranges.push(range);
                }
            }
        }
    }

    if last_map.is_some() {
        let mapping = last_map.unwrap();
        maps.push(mapping);
    }

    for (start, end) in seeds.clone().unwrap().iter() {
        println!("{}..{}", start, end);
        let mut seeds_to_convert = vec![(*start, *end)];

        for map in &maps {
            println!("{} -> {}", map.from, map.to);
            let mut new_seeds = Vec::new();
            let mut converted_seeds = Vec::new();
            for (idx, (i, j)) in seeds_to_convert.iter().enumerate() {
                for map_range in &map.ranges {
                    let intersection_range = intersection(
                        *i as usize..*j as usize,
                        map_range.src as usize..(map_range.src + map_range.range) as usize,
                    );

                    if let Some(range) = intersection_range.clone() {
                        let new_seed_range = (
                            map_range.dst + (range.start as i64 - map_range.src),
                            map_range.dst + (range.end as i64 - map_range.src),
                        );
                        new_seeds.push(new_seed_range);
                        converted_seeds.push(idx);
                        println!(
                            "   From {}..{} using {}..{} on {:?} to {:?}",
                            i,
                            j,
                            map_range.src,
                            map_range.src + map_range.range,
                            range,
                            new_seed_range
                        );
                    }
                }
            }

            for (idx, seed) in seeds_to_convert.iter().enumerate() {
                if !converted_seeds.contains(&idx) {
                    new_seeds.push(*seed);
                }
            }

            seeds_to_convert = new_seeds;
        }

        println!("Seeds to convert: {:?}", seeds_to_convert);

        for seed in seeds_to_convert {
            if lowest_seed_location == 0 {
                lowest_seed_location = seed.0;
            } else {
                lowest_seed_location = cmp::min(lowest_seed_location, seed.0);
            }
        }
    }

    Ok(lowest_seed_location)
}

struct MapRange {
    src: i64,
    dst: i64,
    range: i64,
}

struct Map {
    from: String,
    to: String,
    ranges: Vec<MapRange>,
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

// Define a function that takes two ranges and returns a tuple of three Option<Range>
fn intersection(range1: Range<usize>, range2: Range<usize>) -> Option<Range<usize>> {
    // Check if the ranges do not overlap
    if range1.end <= range2.start || range2.end <= range1.start {
        // Return None
        None
    } else {
        // Find the maximum of the lower bounds
        let start = range1.start.max(range2.start);
        // Find the minimum of the upper bounds
        let end = range1.end.min(range2.end);
        // Return Some(Range)
        Some(start..end)
    }
}
