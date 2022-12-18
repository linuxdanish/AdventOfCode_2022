/// Advent of Code 2022 Day 5
/// Daniel T. 2022-12-013
/// Part 1 Goal: transfers crates, per instructions to determine which numbers
///             belong on the top.
/// Part 2 Goal: maintain order of the crates poped off the top of the stack


use std::env;
use file_input;
use std::collections::LinkedList;


fn main() {
    // project input
    let args: Vec<String> = env::args().collect();
    let filename: String = String::from(&args[1]);
    println!("Input filename: {}", filename);

    // go ahead and get the content:
    let content: Vec<String> = file_input::file_contents_as_vec(filename).expect("Failed to open input file");

    // File format is a complicated since it contains multiple content with it. Three types of lines:
    // 1. a crate definition looks like: [x] [x] [x]. A column can be empty.
    // 2. a column name line: 1 2 3 4... seperated by any number of spaces.
    // 3. a command line looks like: move x from x to x.

    // find and seperate the commands from the board design.
    let index = content.iter().position(|x| x.is_empty()).unwrap();
    println!("Found split index at: {}", index);

    // get the maximum number of columns:
    // We build a vector of LinkedList, which we will treat as stacks
    let mut columns = content[index -1 ].split_whitespace().map(|_x| -> LinkedList<char> {
        LinkedList::<char>::new()
    }).collect::<Vec<LinkedList<char>>>();

    // Just verify the number of columns
    println!("column line is:");
    println!("{}", content[index -1]);
    println!("Found the following columns: {}", columns.len());
    
    // build the board
    for board_counter in (0..index-1).rev() {
       board_line(&content[board_counter], &mut columns);
    }

    // before we process/alter our board, copy if for part 2
    let mut columns_part2 = columns.clone();

    // process commands
    for i in index+1..content.len() {
        let command = command_line(&content[i]);
        command.execute(&mut columns);
    }

    // calculate the final values:
    let results: String = columns.iter().fold("".to_string(), |mut acc: String, x| -> String {
        if let Some(next) = x.back() {
            acc.push(*next)
        };
        return acc
    });
    
    // print the results
    println!("The final top boxes for part 1 are: {}", results);

    // part 2 calculation
    // process commands
    for i in index+1..content.len() {
        let command = command_line(&content[i]);
        command.execute_part2(&mut columns_part2);
    }

    // calculate part 2 result
    let results_part2: String = columns_part2.iter().fold("".to_string(), |mut acc: String, x| -> String {
        if let Some(next) = x.back() {
            acc.push(*next)
        };
        return acc
    });

    println!("The final top boxes for part2 are: {}", results_part2);
}

fn board_line(line: &String, stacks: &mut Vec<LinkedList<char>>) {
        for (index, val) in line.chars().enumerate() {
            if val.is_alphabetic() {
               
                //use the quotient of a division of 4 to select the column to insert into...
                let col_index = index / 4;
                // Debug: 
                //println!("Found char {}, at column {}", val, col_index);
                if col_index <= stacks.len() {
                    stacks[col_index].push_back(val);
                }
            }
        }
}

fn command_line(line: &String) -> Command {
   // each command line is of the format: move x from x to x
   let split: Vec<&str> = line.split_whitespace().collect();
   // subtract 1 from index to line up with columns
   if split.len() >= 6 {
        let command = Command {
            amount:  split[1].parse::<i32>().unwrap(),
            from:  split[3].parse::<usize>().unwrap() -1 ,
            to:  split[5].parse::<usize>().unwrap() -1
        };
       return command;
   } else {
       let  command = Command {
            amount: 0,
            from: 0,
            to: 0
        };
       return command;
   }
}

struct Command {
    amount: i32,
    from: usize,
    to: usize
}

impl Command {
    fn execute(&self, stacks: &mut Vec<LinkedList<char>>) {
        for _i in 0..self.amount {
            let item = stacks[self.from].pop_back().unwrap();
            stacks[self.to].push_back(item);
        }
    }

    fn execute_part2(&self, stacks: &mut Vec<LinkedList<char>>) {
        let index = stacks[self.from].len() - self.amount as usize;
        let mut temp_list = stacks[self.from].split_off(index);
        stacks[self.to].append(&mut temp_list);
    }
}
