use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;


fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::build(&args).expect("Problem parsing arguments");

   let sum = run(config).expect("Error");

   println!("The sum of all of the calibration values is: {sum}");
   
}

fn run(config: Config) -> Result<i32, Box<dyn Error>> {
   let file = File::open(config.file_path)?;
   let reader = BufReader::new(file);
   let mut sum = 0;

   for  line in reader.lines() {
      let line = line?;
      let mut parts = line.split_terminator(&[':', '|'][..]);
      let card = parts.next().unwrap();
      let winner_part: Vec<i32> = parts.next()
                                       .unwrap()
                                       .split(' ')
                                       .filter_map(|n| n.trim().parse::<i32>().ok())
                                       .collect();
      let owner_part: Vec<i32> =  parts.next()
                                       .unwrap()
                                       .split(' ')
                                       .filter_map(|n| n.trim().parse::<i32>().ok())
                                       .collect();

      let winner_set: HashSet<_> = winner_part.iter().cloned().collect();
      let owner_set: HashSet<_> = owner_part.iter().cloned().collect();

      let common_numbers: HashSet<_> = owner_set.intersection(&winner_set).cloned().collect();
      
      let points: i32 = match common_numbers.len() {
          0 => 0,
          x => u32::pow(2, (x as i32 - 1) as u32) as i32
      };

      println!("{card}: {points}");
      sum += points;

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
