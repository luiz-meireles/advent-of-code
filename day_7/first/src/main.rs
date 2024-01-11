use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::zip;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).expect("Problem parsing arguments");

    let sum = run(config).expect("Error");

    println!("The total winnings are: {sum}");
}

fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    let reader_iter = reader.lines().into_iter();

    let sorted_cards = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let card_ranks: HashMap<char, usize> = sorted_cards
        .iter()
        .enumerate()
        .map(|(i, &card)| (card, i))
        .collect();

    let compare_cards = |a: char, b: char| {
        let a_index = card_ranks[&a];
        let b_index = card_ranks[&b];

        a_index.cmp(&b_index)
    };

    let compare_hands = |a: &str, b: &str| {
        let a_type = get_hand_type(a);
        let b_type = get_hand_type(b);

        if a_type != b_type {
            return a_type.cmp(&b_type);
        }

        let a_chars = a.chars().collect::<Vec<char>>();
        let b_chars = b.chars().collect::<Vec<char>>();

        for (a, b) in zip(a_chars.iter(), b_chars.iter()) {
            let cmp = compare_cards(*a, *b);

            if cmp != std::cmp::Ordering::Equal {
                return cmp;
            }
        }

        std::cmp::Ordering::Equal
    };

    let cards_with_bids = {
        let mut cards_with_bids = Vec::new();

        for line in reader_iter.into_iter() {
            if let Ok(line) = line {
                let mut cards_with_bid = line
                    .split_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>();
                let bid = cards_with_bid.pop().unwrap();
                let cards = cards_with_bid.pop().unwrap();
                print_hand_type(&cards);
                cards_with_bids.push((cards, bid));
            }
        }
        cards_with_bids.sort_by(|a, b| compare_hands(&a.0, &b.0));
        cards_with_bids.reverse();
        cards_with_bids
    };

    let total_winnings = cards_with_bids
        .iter()
        .enumerate()
        .filter_map(|(i, (card, bid))| {
            bid.parse::<i64>().ok().map(|bid| {
                let winnings = bid * (i as i64 + 1);
                println!("{} {} {}", card, bid, winnings);
                winnings
            })
        })
        .sum::<i64>();

    Ok(total_winnings)
}

fn print_hand_type(hand: &str) {
    let hand_type = get_hand_type(hand);

    let hand_type_str = match hand_type {
        1 => "five of a kind",
        2 => "four of a kind",
        3 => "full house",
        4 => "three of a kind",
        5 => "two pair",
        6 => "one pair",
        7 => "highest card",
        _ => panic!("Invalid hand"),
    };

    println!("{}: {}", hand, hand_type_str);
}

fn get_hand_type(hand: &str) -> i32 {
    let mut count = HashMap::new();

    for c in hand.chars() {
        let counter = count.entry(c).or_insert(0);
        *counter += 1;
    }
    let max_count = *count.values().max().unwrap();
    let hand_type = match (count.len(), max_count) {
        (1, _) => 1,
        (2, 4) => 2,
        (2, _) => 3,
        (3, 3) => 4,
        (3, _) => 5,
        (4, _) => 6,
        (5, _) => 7,
        _ => {
            panic!("Invalid hand")
        }
    };

    hand_type
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
