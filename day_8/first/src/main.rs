use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, stdin, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).expect("Problem parsing arguments");

    let sum = run(config).expect("Error");

    println!("The number of steps required to reach ZZZ are: {sum}");
}

fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    let mut reader_iter = reader.lines().into_iter();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    let first_node = "AAA".to_string();
    let final_node = "ZZZ".to_string();
    let left = 0;
    let right = 1;

    let left_right_intructions: Vec<i32> = reader_iter
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => right,
            'L' => left,
            _ => panic!("Invalid instruction"),
        })
        .collect();

    reader_iter
        .filter(|line| line.as_ref().is_ok_and(|f| f != ""))
        .for_each(|line| {
            let line = line.unwrap();
            let mut network_and_nodes = line.split("=").map(|s| s.trim());
            let node = network_and_nodes.next().unwrap();
            let binding = network_and_nodes
                .next()
                .unwrap()
                .replace("(", "")
                .replace(")", "")
                .split(",")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            let (left, right) = (binding[0].clone(), binding[1].clone());
            network.insert(node.to_string(), (left, right));
        });

    let number_of_steps = {
        let mut number_of_steps = 0;
        let mut next_node = first_node;
        let mut buffer = String::new();

        let _: Vec<_> = left_right_intructions
            .iter()
            .cycle()
            .take_while(|direction: &&i32| {
                let (left, right) = network.get(&next_node).unwrap();
                println!("{} = ({}, {})", next_node, left, right);

                next_node = match **direction {
                    0 => {
                        println!("  => L");
                        left.clone()
                    }
                    1 => {
                        println!("  => R");
                        right.clone()
                    }
                    _ => panic!("Invalid direction"),
                };

                number_of_steps += 1;
                if next_node == final_node {
                    println!("You have reached the final node");
                    stdin().read_line(&mut buffer).unwrap();
                    return false;
                }
                true
            })
            .collect();
        number_of_steps
    };

    Ok(number_of_steps)
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
