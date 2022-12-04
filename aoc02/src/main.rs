/// Advent of Code 2022 project 02
/// Daniel T. 2022-12-03
/// Part 1 Goal: What is your score if the strategy is played
/// 
/// Legend:
/// A, x = Rock = 1
/// B, Y = Paper = 2
/// C, Z = Scissors = 3
/// 0 for loss, 3 for draw, 6 for win points for win
///
/// Part 2: x,y, and z now indicate lose, draw, win.
/// Goal calculate score with new information.



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
      //  println!("char: {}",x );
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
       // println!("DEBUG:: acc {}, result: {}, x.0: {}, x.1: {}", acc, challenge(x.0, x.1), x.0 as i32, x.1 as i32);
        acc + (challenge(x.0, x.1) + ( x.1 as i32 + 1))
    });

    // result 
    println!("The score if (part 1) strategy is played is: {}", score);

    // Part 2 processor


    // function takes opponent's ' selection and chooses what we should play in response
    let choose_play = | x: &str, y: &str | -> GameOption {

        // New truth table with results in mind
        //|          | Lose     | draw     | win      |
        //| Rock     | Scissors | Rock     | Paper    |
        //| Paper    | Rock     | Paper    | Scissors |
        //| Scissors | Paper    | Scissors | Rock     |
        let  tt2 = [
            [GameOption::Scissors, GameOption::Rock, GameOption::Paper],
            [GameOption::Rock, GameOption::Paper, GameOption::Scissors],
            [GameOption::Paper, GameOption::Scissors, GameOption::Rock]
        ];
        // This is hokie, but we can re-use the same mapping as before for indexing
        let new_index = | x: char | -> usize {
             match x.to_ascii_uppercase() {
            'A' | 'X' => {0},
            'B' | 'Y' => {1},
            'C' | 'Z' => {2},
            _ => {panic!("invalid game option")}
            }
        };

        let x = x.chars().next().unwrap();
        let x = new_index(x);
        let y = y.chars().next().unwrap();
        let y = new_index(y);
        // return the lookup
        tt2[x][y]
    };

    let mut rounds2: Vec<(GameOption, GameOption)> = Vec::new();
    for line in content.iter() {
        let line = line.split_whitespace().collect::<Vec<&str>>();

        if line.len() >= 2 {
            let other_shape = option_map(line[0].chars().next().unwrap());
            let our_shape = choose_play(line[0], line[1]);
            rounds2.push((other_shape, our_shape));
        }
    };

    // processing our data for the second round is the same as the first. Because we prepped the
    // data to the same valid structure
    // lets process our data:
    let part2_score = rounds2.iter().fold(0, | acc, x| -> i32 {
        // println!("DEBUG:: acc {}, result: {}, x.0: {}, x.1: {}", acc, challenge(x.0, x.1), x.0 as i32, x.1 as i32);
        acc + (challenge(x.0, x.1) + ( x.1 as i32 + 1))
    });

    println!("The score if (part 2) strategy is played is: {}", part2_score);

}

// names for different options that can be selects
#[derive(Copy, Clone)]
enum GameOption {
    Rock,
    Paper,
    Scissors
}

