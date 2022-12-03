/// Advent of Code 2022 project 01
/// Daniel T. 2022-12-03
/// Part 1 Goal: Find the elf carrying the most calories
///              How many total calories is that elf carrying?
/// 
/// Part 2: Goal identify the top three elves with the most calories. 
///              How many calories are those elves carrying in total?

use std::env;
use file_input;

fn main() {
    // project input
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);

    // get content of input file to process:
    let content: Vec<String> = file_input::file_contents_as_vec(filename).expect("Failed to open input file");

    // create a vector of each elf's callories
    let mut elf_cals: Vec<i32> = Vec::new();
    //populate elf_cals
    let content_iter = content.iter();
    let mut cal: i32 = 0;
    for line in content_iter {
        if line.len() != 0 {
            cal += str::parse::<i32>(line).expect("failed to parse line to int");
        } else {
            elf_cals.push(cal);
            cal = 0;
        }
    }
    // if we finish the loop and cal still has a value, then we need to push it to our vector. 
    if cal != 0 {
        elf_cals.push(cal);
    }

    // get the highest value in the elf_cals vector
    let max = elf_cals.iter().max().unwrap();
    println!("The elf carrying the most calories is carrying {} calories.", max);

    // Part 2: reverse sort
    elf_cals.sort();
    // print the top three
    let top_three = elf_cals[elf_cals.len()-3 .. elf_cals.len()].to_vec();
    println!("The top three elves are carrying: {:?}", top_three);
    let sum: i32 = top_three.iter().sum();
    println!("The total of the top three is {}", sum);

}
