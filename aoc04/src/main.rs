/// Advent of Code 2022 Day 4
/// Daniel T. 2022-12-07
/// Part 1 Goal: Locate pairs that are completely contained within each other.
///             Return the total count

use std::env;
use file_input;

fn main() {
    // project input
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);

    // go ahead and get the content:
    let content: Vec<String> = file_input::file_contents_as_vec(filename).expect("Failed to open input file");

    let lines_to_range = |x: &String| -> (Range, Range) {
        let pairs: Vec<&str> = x.split(',').collect::<Vec<&str>>();
        let range1: Vec<i32> = pairs[0].split('-').map(|x| -> i32 { x.parse::<i32>().unwrap()}).collect();
        let range1 = Range {
            min: range1[0],
            max: range1[1]
        };
        let range2: Vec<i32> = pairs[1].split('-').map(|x| -> i32 { x.parse::<i32>().unwrap()}).collect();
        let range2 = Range {
            min: range2[0],
            max: range2[1]
        };
        (range1, range2)
    };

    // Each line is in the form <i32>-<i32>,<i32>-<i32>
    // Split each line into a tuple of set structs
    let content: Vec<(Range,Range)> = content.iter().take_while(|x| !x.is_empty())
    .map(|x| lines_to_range(x)).collect();
    let count = content.iter().fold(0, |acc, x| -> i32 {
        if (x.0.min >= x.1.min && x.0.max <= x.1.max) || (x.1.min >= x.0.min && x.1.max <=  x.0.max) {
            acc + 1
        } else  {
            acc
        }
    });
    println!("Part 1: The number of sets contained in another = {}", count);

}

struct Range {
    min: i32,
    max: i32
}
