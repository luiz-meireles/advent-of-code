use std::{env, usize};
use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::cmp;

fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::build(&args).expect("Problem parsing arguments");

   let sum = run(config).expect("Error");

   println!("The sum of all of the calibration values is: {sum}");
   
}

fn run(config: Config) -> Result<i32, Box<dyn Error>> {
   let file = File::open(config.file_path)?;
   let reader = BufReader::new(file);
   let mut digits: HashMap<String, String> =  HashMap::new();
   let mut digits_parts_indexes: HashMap<String, String> = HashMap::new();
   let mut symbols: Vec<(usize, usize)> = Vec::new();
   let mut sum = 0;

   let mut n_cols = 0;
   let mut n_rows = 0;

   for (row, line) in reader.lines().enumerate() {
      let line = line?;
      let mut col_digit_start: Option<usize> = None;

      n_rows = cmp::max(row, n_rows);
      
      for (col, c) in line.char_indices() {
            n_cols = cmp::max(col, n_cols);
   
            if c.is_digit(10) {
               if col_digit_start.is_none() {
                  col_digit_start = Some(col);
               }
               let digit_key = format!("{},{}", row, col_digit_start.unwrap());
   
               digits
                  .entry(digit_key.clone())
                  .and_modify(|digit| { digit.push(c)})
                  .or_insert(c.to_string());

               let digit_part_key = format!("{},{}", row, col);
   
               digits_parts_indexes.insert(digit_part_key, digit_key.clone());
               
            } else {
               if c != '.' {
                     print!("{c}");
                     symbols.push((row, col));
                  }
         
               col_digit_start = None;
            }
            
         }

      
   }

   let nd = digits.len();
   let ns = symbols.len();
   println!("rows = {n_rows}\ncols = {n_cols}\ndigists = {nd}\nsymbols = {ns}");


   for (i, j) in symbols.iter() {
      let neighbors = get_adjacent_neighbors(*i as i32, *j as i32, (n_rows as i32) + 1, (n_cols as i32) + 1);

      for (row, col) in neighbors {
         let digit_part_key = format!("{},{}", row, col);
         if let Some(digit_key) = digits_parts_indexes.get(&digit_part_key) {
               if let Some(digit_str) = digits.get(digit_key) {
                  let digit: i32 = digit_str.parse().unwrap();
                  sum += digit;

                  digits.remove(digit_key);
               }
         }
      }
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


 fn get_adjacent_neighbors(i: i32, j: i32, n: i32, m: i32) -> impl Iterator<Item = (i32, i32)> {
   let neighbors = vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];

   neighbors.into_iter().filter(move |&(x, y)| {
      x < n && y < m && x >= 0 && y >= 0 && (x, y) != (i, j)
   })
 }

