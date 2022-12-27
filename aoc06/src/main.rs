/// Advent of Code 2022 Day 6
/// Daniel T. 2022-12-26
/// Part 1 Goal: Locate the marker (four chars that are all different) and 
///      determine how many chars need to be processed before the first 
///      start of packed marker is detected?
/// Part 2 goal: Locate start of messager marker (14 unique characters)

use std::env;
use std::collections::LinkedList;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

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

    let mut start_of_packet_window = UniqueWindow {
        values: LinkedList::new(),
        dup_counter: HashMap::new(),
        size: 4
    };

    for (i,c) in input.chars().enumerate() {
        start_of_packet_window.push(c);
        if start_of_packet_window.is_unique() {
            let processed_chars = i + 1; // account for zero index. 
            println!("Part1 number of chars before the end of the first packet-start marker: {}", processed_chars);
            break;
        }
    }

    //part 2 
    let mut start_of_message_finder = UniqueWindow {
        values: LinkedList::new(),
        dup_counter: HashMap::new(),
        size: 14
    };

    for (i,c) in input.chars().enumerate() {
        start_of_message_finder.push(c);
        if start_of_message_finder.is_unique() {
            let processed_chars = i + 1;
            println!("Part 2 number of chars before the end of the first message-start marker: {}", processed_chars);
            break;
        }
    }
}

// This is a first in first out queue to create a sliding window. 
struct UniqueWindow {
    values: LinkedList<char>,
    dup_counter: HashMap<char, i32>,
    size: usize,
}

impl UniqueWindow {

    // Adds item to front of queue, queue is full, it pops off
    // the last item
    fn push(&mut self, x: char) {
        if self.values.len() >= self.size {
            if let Some(dead_key) = self.values.pop_back() {
                let dead_val = self.dup_counter.get(&dead_key).unwrap();
                self.dup_counter.insert(dead_key, dead_val - 1);
            }
        }
        self.values.push_front(x);
        if let Some(current_val) = self.dup_counter.get(&x) {
            self.dup_counter.insert(x, current_val + 1);
        } else {
            self.dup_counter.insert(x,1);
        }
    } 

    // checks to see if the contents of the window are each unique.
    // returns false if queue is not fully populated
    fn is_unique(&self)->bool {
        if self.values.len() < self.size {
            return false;
        } 

        let mut result = true;
        for value in self.dup_counter.values() {
            if *value >= 2 {
                result = false;
            }
        }

        return result;
    }
}


