
/*
 * Advent of Code 2022 Day 07
 * Daniel T. 2022-12-28
 * Part 1 Goal: Find all of the directories with a total size of at most 100000. 
 *      What is the sum of the total sizes of those directories.
 */

 use std::{env::{self}, str::SplitWhitespace};
 use file_input;
 use std::collections::HashMap;


fn main() {
    // Go ahead and get the input and create a vec of file content. 
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);
    let content: Vec<String> = file_input::file_contents_as_vec(filename).expect("failed to open input file");

    // will build a hashmap of directories and their sizes
    // Maintain a stack of the current path 
    let mut cur_path = Vec::<String>::new();
    let mut directories = HashMap::<String, FsDirectory>::new();
    let part1_max = 100000;

    for line in content.iter() {
        let mut line_parts = line.split_whitespace();
        match line_parts.next() {
            Some("$") => {
                command_processor(line_parts, &mut cur_path, &mut directories);
            },
            Some("dir") => {
                // build a new directory entry and add it to our list
                // line should be of the form "dir <name>"
                // I append the path of the cur_path to make each one unique
                let dir_name = line_parts.next().unwrap();
                let dir = FsDirectory{
                    total_size: 0
                };
                let long_hash = cur_path.concat() + dir_name;
                directories.insert(long_hash, dir);
            },
            Some(_) => {
                // anything else should be a file line of the form:
                // <size> <filename>
                // we will add the file size to the current directory
                if let Some(cur_dir_name) = cur_path.last() {
                    if let Some(cur_dir) = directories.get_mut(&cur_dir_name.to_string()) {
                        let mut line_part = line.split_whitespace();
                        cur_dir.total_size += str::parse::<i32>(line_part.next().unwrap()).expect("failed to parse file size to i32");
                    }
                }
            },
            None => {}
        }
    }

    // when done, I need to finish popping everything off of my cur_path stack:
    while !cur_path.is_empty() {
        println!("Debug: clean up remaining path items, cur_path len {}, directories count: {}", cur_path.len(), directories.len());
        let old_dir_name = cur_path.pop().expect("failed to pop cur path");
        let mut old_dir_size = 0;
        if let Some(old_dir) = directories.get(&old_dir_name) {
            old_dir_size = old_dir.total_size;
        }
        if let Some(new_dir_name) = cur_path.last(){
            if let Some(new_dir) = directories.get_mut(new_dir_name) {
                new_dir.total_size += old_dir_size;
            }
        }
        
    }

    let candidates: Vec<(&String,&FsDirectory)> = directories.iter().filter(|x| x.1.total_size <= part1_max).collect();
    let part1_result = candidates.iter()
        .fold(0, |acc, x| {
            acc + x.1.total_size
        });

    println!("Part1, sum of directories with a maximum size of 100,000: {}", part1_result);

}

// helper functions

// Function takes a command line "cd, ls, etc." then handles the process of CDing into and out of a directory
fn command_processor<'b>(mut line: SplitWhitespace<'b>, cur_path: &mut Vec<String>, dirs: &mut HashMap<String, FsDirectory> ) {
    match line.next() {
        Some("cd") => {
            if let Some(arg) = line.next() {
                match arg {
                    ".." => {
                        // Pop the current directory name off of the stack, lookup its size and add it to 
                        // the new directory's size
                        print!("Debug: directory count {}", dirs.len());
                        let old_dir_size = dirs.get(&cur_path.pop().unwrap()).unwrap().total_size;
                        if let Some(new_dir_name) = cur_path.last() {
                            if let Some(new_dir) = dirs.get_mut(new_dir_name) {
                                new_dir.total_size += old_dir_size;
                            }
                        }                      
                        println!("Debug: directory count {}", dirs.len());
                    },
                    _ => {
                        // The argument is a directory name we will need to add it to our stack. 
                        let long_hash = cur_path.concat() + arg.clone();
                        cur_path.push(long_hash);
                    }
                }
            }
        },
        Some(_) | None => {}
    }
}


#[derive(Copy, Clone, Debug)]
struct FsDirectory {
    total_size: i32,
}

