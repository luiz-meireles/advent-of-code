use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
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
    let game_id_sep = ':';
    let cubes_set_sep = ';';
    let cube_items_sep = ',';
    
    let mut sum_of_set_power = 0;
    
    for line in reader.lines() {
        let line = line?;
        let separators = [game_id_sep, cubes_set_sep];
        let mut game_and_sets = line.split_terminator(&separators[..]).into_iter();
        let game_id: i32 = game_and_sets.next().unwrap().replace("Game ", "").parse().unwrap();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for part in game_and_sets {
            let cube_items = part.split(cube_items_sep);
            for cube_item in cube_items {
                let mut value_and_color = cube_item.trim().split(' ');
                let value: i32 = value_and_color.next().unwrap().parse().unwrap();
                let color = value_and_color.next().unwrap();

                match color {
                    "red" => max_red = cmp::max(max_red, value),
                    "green" => max_green = cmp::max(max_green, value),
                    "blue" =>  max_blue = cmp::max(max_blue, value),
                    _ => panic!()
                }

            }
        }
        let power = max_red * max_green * max_blue;
        println!("Game {game_id}: ({max_red}, {max_green}, {max_blue}) => {power}");
        sum_of_set_power += power;
    
    }

    Ok(sum_of_set_power)
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