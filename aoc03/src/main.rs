/// Advent of Code 2022 Day 3
/// Daniel T. 2022-12-05
/// Part 1 Goal: Locate the character repeated in the first half and second half
///              of each string. Then calculate score based on: a-z = [1-26] A-Z=[27-52]
/// Part 2 Goal: For each set of three lines, locate the one character common to all three...
///                then compute the score.

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

    // Part 2:
    let mut counter = 0;
    let mut badges: Vec<char> = Vec::new();
    let mut candidates: HashMap<char, i32> = HashMap::new();
    for line in content.iter() {
    // sort and dedup
        let mut sorted = line.chars().collect::<Vec<char>>();
        sorted.sort_unstable();
        sorted.dedup();
        for y in sorted.iter() {
            // if the key is already in the candidate pool, we increase the count.
            // else we add it to the pool
            if let Some(x) = candidates.get_mut(&y) {
                *x = *x + 1;
            } else {
                candidates.insert(*y, 1);
            }
        }
        // increase counter for group
        counter = counter + 1;
        if counter >= 3 {
            // for every three lines, find the badge, then clear the candidate and reset the counter.
            for (key, value) in candidates.iter() {
                if value >= &3 {
                    badges.push(*key);
                    break;
                }
            }
            candidates.clear();
            counter = 0;
        }
    }

    let score_part2 = badges.iter().fold(0, |acc, x| -> i32 {
        acc + alphabet.chars().position(|y| x == &y).unwrap() as i32 + 1
    });

    println!("The part two score is: {}", score_part2);
}
