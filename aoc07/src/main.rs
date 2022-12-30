
/*
 * Advent of Code 2022 Day 07
 * Daniel T. 2022-12-28
 * Part 1 Goal: Find all of the directories with a total size of at most 100000. 
 *      What is the sum of the total sizes of those directories.
 */

 use std::{env::{self, Args}, str::SplitWhitespace};
 use file_input;
 use b_tree_lib;
 use std::collections::HashMap;


fn main() {
    // Go ahead and get the input and create a vec of file content. 
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);
    let content: Vec<String> = file_input::file_contents_as_vec(filename).expect("failed to open input file");

    // will build a hashmap of directories and their sizes
    // Maintain a stack of the current path 
    let mut cur_path = Vec::<&str>::new();
    let mut directories = HashMap::<&str, FsDirectory>::new();



    for line in content.iter() {
        let mut line_parts = line.split_whitespace();
        match line_parts.next() {
            Some("$") => {
                command_processor(line_parts, &mut cur_path, &mut directories);
            },
            Some("dir") => {

            },
            Some(_) => {

            },
            None => {}
        }
    }

}

// helper functions

// Function takes a command line "cd, ls, etc." then handles the process of CDing into and out of a directory
fn command_processor<'b>(mut line: SplitWhitespace<'b>, cur_path: &'b mut Vec<&'b str>, dirs: &'b mut HashMap<&'b str, FsDirectory<'b>> ) {
    match line.next() {
        Some("cd") => {
            if let Some(arg) = line.next() {
                match arg {
                    ".." => {
                        // Pop the current directory name off of the stack, lookup its size and add it to 
                        // the new directory's size
                        let old_dir_size = dirs.get(cur_path.pop().unwrap()).unwrap().total_size;
                        if let Some(new_dir_name) = cur_path.last(){
                            if let Some(new_dir) = dirs.get_mut(new_dir_name) {
                                new_dir.total_size = old_dir_size;
                            }
                        }                        
                    },
                    _ => {
                        // The argument is a directory name we will need to add it to our stack. 
                        if let Some(dir) = dirs.get(arg) {
                            cur_path.push(dir.name);
                        } else {
                            let mut dir = FsDirectory {
                                name: arg.clone(),
                                total_size: 0
                            };
                            dirs.insert(dir.name, dir);
                            cur_path.push(dir.name);

                        }
                    }
                }
            }
        },
        Some(_) | None => {}
    }
}


#[derive(Copy, Clone)]
struct FsDirectory<'a> {
    name: &'a str,
    total_size: i32,
}

