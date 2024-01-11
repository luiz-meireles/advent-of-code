use colored::*;

use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, stdin, BufReader};

use std::{env, vec};

fn _read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer.trim().to_string();
}

fn _print_matrix_with_colored_path(
    matrix: &Vec<Vec<char>>,
    path: &Vec<(usize, usize)>,
    current_pos: (usize, usize),
) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let mut colored = false;
            for (pi, pj) in path {
                if *pi == i && *pj == j {
                    print!("{}", c.to_string().red());
                    colored = true;
                    break;
                }
            }
            if !colored {
                if current_pos.0 == i && current_pos.1 == j {
                    print!("{}", c.to_string().green());
                } else {
                    print!("{}", c);
                }
            }
        }
        println!();
    }
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
    let expansion_factor = 1_000_000;

    // construct a matrix from the input file
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut galaxy_indexes: Vec<(usize, usize)> = Vec::new();
    for (i, line) in reader_iter.enumerate() {
        let line = line?;

        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.char_indices() {
            if c == '#' {
                galaxy_indexes.push((i, j));
            }
            row.push(c);
        }

        matrix.push(row);
    }

    let expanded_rows: HashSet<_> = (0..matrix.len())
        .into_iter()
        .filter(|i| galaxy_indexes.iter().all(|(gi, _)| *gi != *i))
        .collect();

    let expanded_cols: HashSet<_> = (0..matrix[0].len())
        .into_iter()
        .filter(|j| galaxy_indexes.iter().all(|(_, gj)| *gj != *j))
        .collect();

    let sum: i64 = galaxy_indexes
        .par_iter()
        .enumerate()
        .map(|(i, &idx)| {
            let (distances, _) = get_distances(
                idx,
                &matrix,
                &expanded_rows,
                &expanded_cols,
                expansion_factor,
            );
            (i..galaxy_indexes.len())
                .map(|j| distances[galaxy_indexes[j].0][galaxy_indexes[j].1] as i64)
                .sum::<i64>()
        })
        .sum();

    Ok(sum as i64)
}

fn get_distances(
    source: (usize, usize),
    matrix: &Vec<Vec<char>>,
    expanded_rows: &HashSet<usize>,
    expanded_cols: &HashSet<usize>,
    expansion_factor: i64,
) -> (Vec<Vec<i64>>, Vec<Vec<(usize, usize)>>) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut parents: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); matrix[0].len()]; matrix.len()];
    let mut distances = vec![vec![std::i64::MAX; matrix[0].len()]; matrix.len()];

    parents[source.0][source.1] = source;
    distances[source.0][source.1] = 0;

    queue.push_back(source);

    while !queue.is_empty() {
        let current_pos = queue.pop_front().unwrap();
        let connected_indexes =
            get_adjacent_indexes(current_pos.0, current_pos.1, matrix.len(), matrix[0].len());

        for connected_index in connected_indexes {
            if distances[connected_index.0][connected_index.1] == std::i64::MAX {
                let distance = distances[current_pos.0][current_pos.1]
                    + if expanded_cols.contains(&connected_index.1)
                        || expanded_rows.contains(&connected_index.0)
                    {
                        expansion_factor
                    } else {
                        1
                    };
                distances[connected_index.0][connected_index.1] = distance;
                queue.push_back(connected_index);
                parents[connected_index.0][connected_index.1] = current_pos;
            }
        }
    }

    return (distances, parents);
}

fn _get_path(
    source: (usize, usize),
    destination: (usize, usize),
    parents: Vec<Vec<(usize, usize)>>,
) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut current_pos = destination;

    while current_pos != source {
        path.push(current_pos);
        current_pos = parents[current_pos.0][current_pos.1];
    }

    path.reverse();

    return path;
}

fn get_adjacent_indexes(i: usize, j: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    // top, right, left, bottom
    //              (i-1, j)
    // (i, j-1)     (i, j)     (i, j+1)
    //              (i+1, j)
    let i = i as i32;
    let j = j as i32;
    let n = n as i32;
    let m = m as i32;

    return vec![(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)]
        .iter()
        .filter(|(i, j)| *i >= 0 && *i < n && *j >= 0 && *j < m)
        .map(|(i, j)| (*i as usize, *j as usize))
        .collect::<Vec<(usize, usize)>>();
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
