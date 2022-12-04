/// Advent of Code 2022 project 02
/// Daniel T. 2022-12-03
/// Part 1 Goal: What is your score if the strategy is played
/// 
/// Legend:
/// A, x = Rock = 1
/// B, Y = Paper = 2
/// C, Z = Scissors = 3
/// 3 points for win

use std::env;
use file_input;
use core::panic;

fn main() {
    // project input
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);

    // table for comparisons,to return value for each win
    // |          | rock | paper | scissors |
    // |     rock | d 3  | l 0   | w 6      |
    // |    paper | w 6  | d 3   | l 0      |
    // | scissors | l 0  | w 6   | d 3      |
    let tt = [
        [3,0,6],
        [6,3,0],
        [0,6,3]
    ];
    // Not fully sure if using c like enums is the most "Rust" way of doing this. 
    let challenge = | x:GameOption, y:GameOption | -> i32 { tt[y as usize][x as usize]};

    // Lets go ahead an read my input
    let content: Vec<String> = file_input::file_contents_as_vec(filename).expect("Failed to open input file");

    let option_map = | x:char | -> GameOption {
        println!("char: {}",x );
        match x.to_ascii_uppercase() {
            'A' | 'X' => {GameOption::Rock},
            'B' | 'Y' => {GameOption::Paper},
            'C' | 'Z' => {GameOption::Scissors},
            _ => {panic!("invalid game option")}
        }
    };

    let mut rounds: Vec<(GameOption, GameOption)> = Vec::new();
    for line in content.iter() {
        let line = line.split_whitespace().map(|x| -> GameOption {
            option_map(x.chars().next().unwrap())
        }).collect::<Vec<GameOption>>();
        if line.len() >= 2 {
            rounds.push((line[0], line[1]));
        }

    };

    // lets process our data:
    let score = rounds.iter().fold(0, | acc, x| -> i32 {
        println!("DEBUG:: acc {}, result: {}, x.0: {}, x.1: {}", acc, challenge(x.0, x.1), x.0 as i32, x.1 as i32);
        acc + (challenge(x.0, x.1) + ( x.1 as i32 + 1))
    });

    // result 
    println!("The score if stategy is played is: {}", score);
}

// names for different options that can be selects
#[derive(Copy, Clone)]
enum GameOption {
    Rock,
    Paper,
    Scissors
}

