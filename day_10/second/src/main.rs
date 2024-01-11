use colored::*;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, stdin, BufReader};
use std::{env, vec};

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

fn map_pipe_to_char(pipe: &char) -> char {
    let char = match pipe {
        PV => '|',
        PJ => '⌟',
        PF => '⌜',
        PL => '⌞',
        P7 => '⌝',
        PH => '-',
        DOT => '.',
        S => 'S',
        _ => '0',
    };

    return char;
}

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

fn _print_matrix_with_colored_path(
    matrix: &Vec<Vec<char>>,
    path: &Vec<(usize, usize)>,
    current_pos: (usize, usize),
    color: &str,
) {
    // print column indexes aligend with column values
    print!("    ");
    for (i, _) in matrix[0].iter().enumerate() {
        let digit_count = i.to_string().len();
        let space_count = 3 - digit_count;
        print!(
            "{}{}",
            " ".repeat(space_count),
            i.to_string().bright_yellow()
        );
    }
    println!();
    for (i, row) in matrix.iter().enumerate() {
        // print row index and its values aligned with column indexes
        let digit_count = i.to_string().len();
        let space_count = 3 - digit_count;
        print!("{} - ", i.to_string().bright_yellow());
        for (j, c) in row.iter().enumerate() {
            let mut colored = false;
            for (pi, pj) in path {
                if *pi == i && *pj == j {
                    print!("{}{}", " ".repeat(space_count), c.to_string().bright_red());
                    colored = true;
                    break;
                }
            }
            if !colored {
                if current_pos.0 == i && current_pos.1 == j {
                    print!(
                        "{}{}",
                        " ".repeat(space_count),
                        c.to_string().bright_green(),
                    );
                } else {
                    print!("{}{}", " ".repeat(space_count), c.to_string().color(color));
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

    let pipe_path = get_pipe_path(start_pos, &matrix);

    let mut matrix_with_path_only = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| {
                    if pipe_path.contains(&(i, j)) {
                        return map_pipe_to_char(c);
                    }
                    return '0'.clone();
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut sum_inner_pipes = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if pipe_path.contains(&(i, j)) {
                continue;
            }

            if winding_number((i, j), &pipe_path) != 0 {
                matrix_with_path_only[i][j] = '1';
                sum_inner_pipes += 1;
            }
        }
    }

    Ok(sum_inner_pipes)
}

fn get_pipe_path(start_pos: (usize, usize), matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    fn dfs(
        i: usize,
        j: usize,
        matrix: &Vec<Vec<char>>,
        visited: &mut HashSet<(usize, usize)>,
        path: &mut Vec<(usize, usize)>,
        target: (usize, usize),
    ) -> bool {
        let mut stack = VecDeque::new();
        stack.push_back((i, j, 0));

        while let Some((i, j, depth)) = stack.pop_back() {
            if (i, j) == target && depth > 0 {
                return true;
            }

            if visited.contains(&(i, j)) {
                continue;
            }

            visited.insert((i, j));
            path.push((i, j));

            let connected_pipes = get_connected_pipes(i, j, matrix);
            for connected_pipe in connected_pipes {
                stack.push_back((connected_pipe.0, connected_pipe.1, depth + 1));
            }
        }

        false
    }

    dfs(
        start_pos.0,
        start_pos.1,
        matrix,
        &mut visited,
        &mut path,
        start_pos,
    );
    println!("Path: {:?}", path);

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

fn winding_number(point: (usize, usize), polygon: &[(usize, usize)]) -> i32 {
    let mut wn = 0; // the winding number
    let n = polygon.len(); // the number of vertices
    for i in 0..n {
        // loop through all edges of the polygon
        let j = (i + 1) % n; // the next vertex index
        if polygon[i].1 <= point.1 {
            // start y <= point y
            if polygon[j].1 > point.1 {
                // an upward crossing
                if is_left(polygon[i], polygon[j], point) > 0.0 {
                    // point left of edge
                    wn += 1; // have a valid up intersect
                }
            }
        } else {
            // start y > point y (no test needed)
            if polygon[j].1 <= point.1 {
                // a downward crossing
                if is_left(polygon[i], polygon[j], point) < 0.0 {
                    // point right of edge
                    wn -= 1; // have a valid down intersect
                }
            }
        }
    }
    wn
}

// Define a function to check if a point is to the left of a line segment
fn is_left(p1: (usize, usize), p2: (usize, usize), p3: (usize, usize)) -> f64 {
    let p1 = (p1.0 as f64, p1.1 as f64);
    let p2 = (p2.0 as f64, p2.1 as f64);
    let p3 = (p3.0 as f64, p3.1 as f64);

    (p2.0 - p1.0) * (p3.1 - p1.1) - (p3.0 - p1.0) * (p2.1 - p1.1)
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
