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
   let mut cards_points = Vec::new();
   let mut cards_count = Vec::new();

   for  (card, line) in reader.lines().enumerate() {
      let line = line?;
      let mut parts = line.split_terminator(&[':', '|'][..]);
      let _ = parts.next().unwrap();
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
      
      cards_points.push(common_numbers.len());
      
      if cards_count.len() <= card {
         cards_count.push(1);
      }

      for j in (card + 1)..(cards_points[card] + card + 1) {
   
         if cards_count.len() <= j  {
            cards_count.push(1);
         }
         cards_count[j] += cards_count[card];
         print!(" j = {j} : {} ", cards_count[j])
      }


   }


   // for (i, card_points) in cards_points.iter().enumerate() {
   //    print!("{i}: {card_points} | {}", cards_count[i]);

   //    for j in (i + 1)..(*card_points + i + 1) {
   //       cards_count[j] += cards_count[i];
   //       print!(" j = {j} : {} ", cards_count[j])
   //    }

   //    println!();
   // }



   Ok(cards_count.into_iter().reduce(|a, b| a + b).unwrap())
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
