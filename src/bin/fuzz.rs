use std::io::{self, BufRead};

use tree::Tree;

#[derive(Debug)]
enum Command {
    Insert{age: u32, name: String},
    Erase{age: u32, name: String},
    Contains{age: u32, name: String},
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
                if let Ok(age) = command_items[1].parse::<u32>() {
                    Command::Insert{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },
            ("e", 3) => {
                if let Ok(age) = command_items[1].parse::<u32>() {
                    Command::Erase{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },
            ("c", 3) => {
                if let Ok(age) = command_items[1].parse::<u32>() {
                    Command::Contains{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },

            (_, _) => Command::Error("invalid command.".to_string())
        }
    }
}

fn command_loop(br: &mut dyn BufRead) {
    let mut tree = Tree::empty();
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
                        tree.insert(age, name);
                    },
                    Command::Erase{age, name} => {
                        tree.erase(age, name);
                    },
                    Command::Contains{age, name} => {
                        let msg = if tree.contains(age, name) { "y" } else { "n" };
                        println!("{}", msg);
                    }
                    Command::Print => {
                        println!("{}", tree);
                    },
                    Command::Reset => {
                        tree.delete();
                    },
                    Command::Exit => {
                        break;
                    },
                    Command::Error(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}

#[macro_use] extern crate afl;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    fuzz!(|data: &[u8]| {
      command_loop(&mut {data});
    });
}
