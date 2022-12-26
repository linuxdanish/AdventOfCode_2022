/// Advent of Code 2022 Day 6
/// Daniel T. 2022-12-26
/// Part 1 Goal: Locate the marker (four chars that are all different) and 
///      determine how many chars need to be processed before the first 
///      start of packed marker is detected?

use std::env;
use std::collections::LinkedList;
use std::io;
use std::fs::File;
use std::io::BufRead;

fn main() {
    // project input
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);

    // Lets read the file contents. I should contain a single line 
    // with out input, so we will just read it in.
    let file = File::open(filename).expect("Failed to open file");
    let mut reader = io::BufReader::new(file);
    let mut input = String::new();
    let _len = reader.read_line( &mut input); // we just want the first line. The _len isn't

    // create a window iterator over our string (convert ot slice if needed)
    let input_slice = input.get(0..input.len()).unwrap();
    let binding = input_slice.chars().collect::<Vec<char>>();
    let binding = &binding[..];
    let check_window = &mut binding.windows(4);

    let mut marker_index: i32  = 3;
    let mut found = false;
    while !found {
        if let Some(window) = check_window.next() {
            marker_index += 1;
            println!("Debug: {:#?}", window);
            let mut exists = false;
            for i in 1..4 {
                println!("i is: {}", i);
                if window[i..4].contains(&window[i-1]) {
                    exists = true;
                    println!("duplicates {}", &window[i-1]);
                }
            }
            if !exists {
                println!("Found marker {:#?}", window);
                found = true;
            }
        } else {
            break;
        }
    }

    println!("The number of chars to process before the end of the first marker is: {}", marker_index);
}




