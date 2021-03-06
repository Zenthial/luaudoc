#![allow(dead_code)]

mod parser;

use std::fs::File;
use std::io::prelude::*;

use parser::State;

fn find_end_of_function(contents: Vec<&str>, mut index: usize) -> Option<usize> {
    let mut end_found = false;
    while !end_found {
        if index >= contents.len() {
            break;
        }

        if contents.get(index).unwrap().contains("end") {
            end_found = true;
        } else {
            index += 1;
        }
    }

    if end_found {
        return Some(index);
    } else {
        return None;
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.lua")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let contents_vec: Vec<&str> = contents.split("\n").collect();
    let iter_arr = contents_vec.clone();

    let mut state = State::new();
    for line in iter_arr {
        parser::parse_line(line, &mut state);
        state.index += 1;
    }

    for vec_key in state.doc_map.keys() {
        let vec = state.doc_map.get(vec_key).unwrap();
        for vector in vec {
            for doc in vector {
                print!("{}", doc);
            }
            println!("");
        }
    }

    Ok(())
}