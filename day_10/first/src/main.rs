use colored::*;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, stdin, BufReader};
use std::thread::sleep;
use std::{env, vec};
use std::{io::stdout, time::Duration};

const T: &char = &'T';
const R: &char = &'R';
const B: &char = &'B';
const L: &char = &'L';

const PV: &char = &'|';
const PJ: &char = &'J';
const PF: &char = &'F';
const PL: &char = &'L';
const P7: &char = &'7';
const PH: &char = &'-';
const DOT: &char = &'.';
const S: &char = &'S';

fn get_matching_pipes<'a>(oritentation: &'a char, pipe: &'a char) -> Vec<&'a char> {
    let matching_pipes = match (oritentation, pipe) {
        (T, S) => vec![PV, P7, PF],
        (R, S) => vec![PH, P7, PJ],
        (B, S) => vec![PV, PJ, PL],
        (L, S) => vec![PH, PL, PF],
        (T, PV) => vec![PV, PF, P7],
        (B, PV) => vec![PV, PL, PJ],
        (R, PH) => vec![PH, P7, PJ],
        (L, PH) => vec![PH, PL, PF],
        (T, PL) => vec![PV, PF, P7],
        (R, PL) => vec![PH, P7, PJ],
        (T, PJ) => vec![PV, PF, P7],
        (L, PJ) => vec![PH, PL, PF],
        (B, P7) => vec![PV, PJ, PL],
        (L, P7) => vec![PH, PL, PF],
        (R, PF) => vec![PH, P7, PJ],
        (B, PF) => vec![PV, PJ, PL],
        _ => vec![],
    };

    return matching_pipes;
}

fn get_oposite_orientation<'a>(oritentation: &'a char) -> &'a char {
    let oposite_orientation = match oritentation {
        T => B,
        R => L,
        B => T,
        L => R,
        _ => panic!("Invalid orientation"),
    };

    return oposite_orientation;
}

fn _read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer.trim().to_string();
}

fn print_matrix_with_colored_path(
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

    println!("The amount of steps it takes to get from the starting position to the point farthest from the starting position are: {sum}");
}

fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    let reader_iter = reader.lines().into_iter();

    // construct a matrix from the input file
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut start_pos = (0, 0);
    for (i, line) in reader_iter.enumerate() {
        let line = line?;
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.char_indices() {
            row.push(c);
            if c == 'S' {
                start_pos = (i, j);
            }
        }
        matrix.push(row);
    }

    println!("Matrix size: {}x{}", matrix.len(), matrix[0].len());
    println!("Start position: {:?}", start_pos);

    let start_post_connections = get_pipe_path(start_pos, &matrix)
        .iter()
        .map(|(i, j)| matrix[*i][*j])
        .collect::<Vec<char>>();

    let middle_distance = start_post_connections.len() / 2;
    println!("Middle distance: {}", middle_distance);

    Ok(middle_distance as i64)
}

fn get_pipe_path(start_pos: (usize, usize), matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut path: Vec<(usize, usize)> = Vec::new();

    queue.push(start_pos);
    path.push(start_pos);
    visited.insert(start_pos, true);

    while !queue.is_empty() {
        let current_pos = queue.remove(0);
        let connected_pipes = get_connected_pipes(current_pos.0, current_pos.1, matrix);
        for connected_pipe in connected_pipes {
            if !visited.contains_key(&connected_pipe) {
                path.push(connected_pipe);

                queue.push(connected_pipe);
                visited.insert(connected_pipe, true);
                // execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
                // print_matrix_with_colored_path(matrix, &path, current_pos);
                // sleep(Duration::from_secs(1));
            }
        }
    }

    return path;
}

fn get_connected_pipes(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let ajdacent_indexes = get_adjacent_indexes(i, j, matrix.len(), matrix[0].len());
    let pipe = &matrix[i][j];
    let connected_pipes = ajdacent_indexes
        .iter()
        .filter(|(i, j, o)| {
            let ajdacent_pipe = &matrix[*i][*j];
            let ajdacent_oriented_pipes =
                get_matching_pipes(get_oposite_orientation(o), ajdacent_pipe);
            let matching_pipes = get_matching_pipes(o, pipe);
            return ajdacent_oriented_pipes
                .iter()
                .any(|ajdacent_oriented_pipe| matching_pipes.contains(ajdacent_oriented_pipe));
        })
        .map(|(i, j, _)| (*i, *j))
        .collect::<Vec<(usize, usize)>>();
    return connected_pipes;
}

fn get_adjacent_indexes(
    i: usize,
    j: usize,
    n: usize,
    m: usize,
) -> Vec<(usize, usize, &'static char)> {
    // top, right, left, bottom
    //              (i-1, j)
    // (i, j-1)     (i, j)     (i, j+1)
    //              (i+1, j)
    let i = i as i32;
    let j = j as i32;
    let n = n as i32;
    let m = m as i32;

    return vec![(i - 1, j, T), (i, j + 1, R), (i + 1, j, B), (i, j - 1, L)]
        .iter()
        .filter(|(i, j, _)| *i >= 0 && *i < n && *j >= 0 && *j < m)
        .map(|(i, j, o)| (*i as usize, *j as usize, *o))
        .collect::<Vec<(usize, usize, &char)>>();
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
