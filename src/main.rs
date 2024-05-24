use std::io::{self, BufRead};
use crate::tree::Data;
use crate::tree::SortedContainer;
mod tree;

#[derive(Debug)]
enum Command {
    Insert{age: i32, name: String},
    Erase{age: i32, name: String},
    Contains{age: i32, name: String},
    Print,
    Reset,
    Exit,
    Error(String)
}

fn parse_command(input: String) -> Command {
    let command_items: Vec<&str> = input.split_whitespace().collect();
    if command_items.len() == 0 {
        Command::Error("invalid command (empty line)".to_string())
    } else {
        match (command_items[0], command_items.len()) {
            ("p", 1) => Command::Print,
            ("q", 1) => Command::Exit,
            ("x", 1) => Command::Reset,
            ("i", 3) => {
                if let Ok(age) = command_items[1].parse::<i32>() {
                    Command::Insert{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },
            ("e", 3) => {
                if let Ok(age) = command_items[1].parse::<i32>() {
                    Command::Erase{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },
            ("c", 3) => {
                if let Ok(age) = command_items[1].parse::<i32>() {
                    Command::Contains{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },

            (_, _) => Command::Error("invalid command.".to_string())
        }
    }
}

fn command_loop(br: &mut dyn BufRead, tree: &mut SortedContainer) {
    loop {
        let mut input = String::new();
        
        match br.read_line(&mut input) {
            Ok(0) => {
                // End of file
                break;
            }
            Ok(_) => {
                match parse_command(input) {
                    Command::Insert{age, name} => {
                        tree.insert_node(Data {age: age, name: name});
                    },
                    Command::Erase{age, name} => {
                        tree.delete(&Data {age: age, name: name});
                    },
                    Command::Contains{age, name} => {
                        if tree.contains(&Data {age: age, name: name}) {
                            println!("y");
                        } else {
                            println!("n");
                        }
                    }
                    Command::Print => {
                        println!("{}", tree);
                    },
                    Command::Reset => {
                        tree.reset()
                    },
                    Command::Exit => {
                        break;
                    },
                    Command::Error(error) => {
                        print!("Error: {}", error);
                    }
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}


fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut tree = SortedContainer::new();
    command_loop(&mut handle, &mut tree);
}