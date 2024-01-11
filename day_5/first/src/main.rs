use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::cmp;


fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::build(&args).expect("Problem parsing arguments");

   let sum = run(config).expect("Error");

   println!("The lowest location number that corresponds to any of the initial seed numbers is: {sum}");

}

fn run(config: Config) -> Result<i64, Box<dyn Error>> {
   let file = File::open(config.file_path)?;
   let reader = BufReader::new(file);
   let mut seeds : Option<Vec<i64>> = None;
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
         },
         x if x.contains("seeds") => {
            seeds = Some(x.split(" ").filter_map(|x| x.parse::<i64>().ok()).collect());
            
         },
         x if x.ends_with("map:")  => {
            let map_name = x.replace("map:", "").replace("-", "").trim().to_string();
           
            let mut mapping = map_name.split("to").into_iter();
            let from = mapping.next().unwrap().to_string();
            let to = mapping.next().unwrap().to_string();
      
            last_map = Some(Map {
               from,
               to,
               ranges: Vec::new(),
            })
            
         },

         x => {
            let numbers: Vec<i64> = x.split(" ").map(|n| {
               println!("{}", n);
               n.trim().parse().unwrap()
            }).collect();
            let range = MapRange {
               dst: numbers[0],
               src: numbers[1],
               range: numbers[2]
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


   for seed in seeds.unwrap().iter() {
      println!("Seed: {}", seed);
      let mut next_seed = seed.clone();
      for map in &maps {

         for r in &map.ranges {
            if next_seed >= r.src && next_seed <= (r.src + r.range)  {
               next_seed = next_seed + &r.dst - &r.src;
               break;
            }
         }

         if map.to == "location" {
            if lowest_seed_location == 0 {
               lowest_seed_location = next_seed;
            } else {
               lowest_seed_location = cmp::min(lowest_seed_location, next_seed);
            }
         }

         println!("  {} -> {} => {}", map.from, map.to, next_seed);

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
