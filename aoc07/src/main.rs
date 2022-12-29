
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
    let mut cur_path = Vec::<&FsDirectory>::new();
    let mut directories = HashMap::<&str, FsDirectory>::new();



    for line in content.iter() {
        let mut line_parts = line.split_whitespace();
        match line_parts.next() {
            Some("$") => {
                
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
fn command_processor(mut line: SplitWhitespace, cur_path: &mut Vec<FsDirectory>, dirs: &mut HashMap<&str, FsDirectory> ) {
    match line.next() {
        Some("ls") => {},
        Some("cd") => {
            if let Some(arg) = line.next() {
                match arg {
                    ".." => {
                        // Pop the current directory off, and add its size to the new current directory
                        let old_dir = cur_path.pop().unwrap();
                        if let Some(new_dir) = cur_path.last(){
                            new_dir.total_size +=  old_dir.total_size;
                        }                        
                        // update the old current directory in the hashmap
                        dirs.insert(old_dir.name, old_dir);
                    },
                    "_" => {
                        // The argument is a file name we will need to add it to our stack. 
                        if let Some(dir) = dirs.get(arg) {
                            cur_path.push(*dir);
                        } else {
                            let mut dir = FsDirectory {
                                name: &arg,
                                total_size: 0,
                                FsType: FsObjectType::Directory,
                                Files: Vec::<FsFile>::new()
                            };
                            dirs.insert(dir.name, dir);
                            cur_path.push(dir);

                        }
                    }
                }
            }
        }
    }
}
enum FsObjectType {
    Directory,
    File
}

struct FsFile {
    size: i32,
    FsType: FsObjectType
}

struct FsDirectory<'a> {
    name: &'a str,
    total_size: i32,
    FsType:  FsObjectType,
    Files: Vec<FsFile>
}

impl FsDirectory {
    fn size(&self)-> i32 {
        self.Files.iter().fold(0, |acc, x| acc + x.size)
    }
}

