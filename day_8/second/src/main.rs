use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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
        let final_node_suffix = "Z";
        let mut next_nodes: Vec<String> = network
            .keys()
            .filter_map(|k| k.ends_with("A").then(|| k.to_string()))
            .collect();
        let mut node_cycles_count: Vec<i64> = vec![0; next_nodes.len()];

        println!("{next_nodes:?}", next_nodes = next_nodes);

        for direction in left_right_intructions.iter().cycle() {
            let mut new_next_nodes = Vec::new();
            for (node_index, node) in next_nodes.iter().enumerate() {
                if node.ends_with(final_node_suffix) {
                    new_next_nodes.push(node.clone());
                } else {
                    let (left, right) = network.get(node).unwrap();
                    let next_node = match direction {
                        0 => left.clone(),
                        1 => right.clone(),
                        _ => panic!("Invalid direction"),
                    };
                    new_next_nodes.push(next_node);
                    node_cycles_count[node_index] += 1;
                }
            }

            next_nodes = new_next_nodes;

            if next_nodes
                .iter()
                .all(|node| node.ends_with(final_node_suffix))
            {
                println!("You have reached the final nodes");
                println!("{next_nodes:?}", next_nodes = next_nodes);
                break;
            }
        }

        println!(
            "{node_cycles_count:?}",
            node_cycles_count = node_cycles_count
        );

        calculate_lcd(node_cycles_count)
    };

    Ok(number_of_steps)
}

fn calculate_lcd(values: Vec<i64>) -> i64 {
    let mut lcd = 1;
    for value in values {
        lcd = lcm(lcd, value);
    }
    lcd
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
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
