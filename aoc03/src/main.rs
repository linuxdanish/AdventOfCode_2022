/// Advent of Code 2022 Day 3
/// Daniel T. 2022-12-05
/// Part 1 Goal: Locate the character repeated in the first half and second half
///              of each string. Then calculate score based on: a-z = [1-26] A-Z=[27-52]


use std::env;
use file_input;
use std::collections::HashMap;

fn main() {
    // project input
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);

    // go ahead and get content:
    let content: Vec<String> = file_input::file_contents_as_vec(filename).expect("Failed to open input file");

    let content_split:Vec<(&str,&str)> = content.iter().map(|x| -> (&str,&str) {
        x.split_at(x.len()/2)
    }).collect();

    // Perform duplicate detection:
    let mut duplicates: Vec<char> = Vec::new();
    for bag in content_split.iter() {
        let mut unique_chars: HashMap<char,i32> = HashMap::new();
        for y in bag.0.chars() {
            unique_chars.insert(y,0);
        }
        for y in bag.1.chars() {
            if unique_chars.contains_key(&y) {
                duplicates.push(y);
                println!("bag.0 {}; bag.1 {}; Duplicate: {}",bag.0, bag.1, y);
                break;
            }
        }
    }

    // need to create the scores/values for each
    let alphabet = "abcdefghijklmnopqrstuvwyxzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    println!("Sanity check: A: {}", alphabet.chars().position(|y| y == 'A').unwrap());
    let score = duplicates.iter().fold(0, |acc, x| -> i32 {
        acc + alphabet.chars().position(|y| x == &y).unwrap() as i32 + 1
    });

    println!("The final score is: {}", score);


}
