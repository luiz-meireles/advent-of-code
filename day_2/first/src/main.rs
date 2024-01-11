use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};

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
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum_of_possible_games_ids = 0;

    'game: for line in reader.lines() {
        let line = line?;
        let separators = [game_id_sep, cubes_set_sep];
        let mut game_and_sets = line.split_terminator(&separators[..]).into_iter();
        let game_id: i32 = game_and_sets.next().unwrap().replace("Game ", "").parse().unwrap();
        for part in game_and_sets {
            let cube_items = part.split(cube_items_sep);
            for cube_item in cube_items {
                let mut value_and_color = cube_item.trim().split(' ');
                let value: i32 = value_and_color.next().unwrap().parse().unwrap();
                let color = value_and_color.next().unwrap();

                match color {
                    "red" => if value > max_red { continue 'game },
                    "green" => if value > max_green { continue 'game },
                    "blue" =>  if value > max_blue { continue 'game },
                    _ => panic!()
                }

            }
        }
        println!("Game {game_id} is possible!");
        sum_of_possible_games_ids += game_id;
    
    }

    Ok(sum_of_possible_games_ids)
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