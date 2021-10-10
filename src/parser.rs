use std::{collections::HashMap, ops::Range};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct State {
    pub current: ParserState,
    pub previous: ParserState,
    pub doc_map: HashMap<String, Vec<Vec<String>>>,
    pub sig_map: Vec<Range<usize>>,
    pub index: usize,
    pub sig_index: usize,
}

impl State {
    pub fn new() -> State {
        State { current: ParserState::Default, previous: ParserState::Default, doc_map: HashMap::new(), sig_map: vec![], index: 0, sig_index: 0 }
    }

    pub fn add_doc(&mut self, doc_type: &str, entry: Vec<String>) {
        if self.doc_map.get(doc_type) != None {
            let mut vec = self.doc_map.get(doc_type).unwrap().clone();
            vec.push(entry);
            self.doc_map.insert(doc_type.to_string(), vec);
        } else {
            let mut vec: Vec<Vec<String>> = vec![vec![]];
            vec.push(entry);
            self.doc_map.insert(doc_type.to_string(), vec);
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParserState {
    Default,
    InFunction,
    InComment,
    TypeStart,
    InType,
    InTable,
    OneLineTable,
    LeftComment,
}

#[derive(Debug, Clone)]
struct Type {
    contains_string: String,
    find_string: String,
    find_expected_index: usize,
    line_type: LineType,
}

impl Type {
    fn new(c_str: &str, l_type: LineType) -> Type {
        Type {contains_string: c_str.to_string(), find_string: c_str.to_string(), find_expected_index: 0, line_type: l_type}
    }

    fn new_with_find(c_str: &str, f_str: &str, f_index: usize, l_type: LineType) -> Type {
        Type {contains_string: c_str.to_string(), find_string: f_str.to_string(), find_expected_index: f_index, line_type: l_type}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LineType {
    Function,
    End,
    While,
    For,
    CommentStart,
    CommentEnd,
    TableInitialization,
    TableStart,
    EndBracket,
    TypeDefinition,
    Default,
}

pub fn parse_function(line: &str, state: &mut State) {
    // Compile regex functions once
    lazy_static! {
        static ref ARGUMENT_REGEX: Regex = Regex::new(r"\([^)]*\)").unwrap();
        static ref RETURN_REGEX: Regex = Regex::new(r"\):*.[a-zA-Z]+").unwrap();
    }

    let func_name_split: Vec<&str> = line.strip_prefix("function ").unwrap().split("(").collect();
    let func_name = func_name_split[0];
    let mut func_vec: Vec<String> = vec![func_name.to_string()];
    
    // Return an option of the argument captures 
    let arguments = ARGUMENT_REGEX.captures(line);
    // Match to handle the None case
    match arguments {
        Some(matches) => println!("{:?}", matches),
        None => println!("No arguments found"),
    }
    
    // Return an option of the return captures
    let return_type = RETURN_REGEX.captures(line);
    // Match to handle the None case
    match return_type {
        Some(matches) => println!("{:?}", matches),
        None => println!("No return type found"),
    }

    state.add_doc("function", func_vec);
}

fn get_line_type(line: &str) -> LineType {
    let line_types: [Type; 10] = [
        Type::new("function", LineType::Function), 
        Type::new("end", LineType::End),
        Type::new("while", LineType::While),
        Type::new("for", LineType::For),
        Type::new("--[[--", LineType::CommentStart),
        Type::new("--]]", LineType::CommentEnd),
        Type::new("type", LineType::TypeDefinition),
        Type::new("}", LineType::EndBracket),
        Type::new_with_find("= {", "local", 0, LineType::TableStart),
        Type::new_with_find("= {}", "local", 0, LineType::TableInitialization),
    ];

    let mut line_type_enum = LineType::Default;

    for line_type in line_types {
        if line.contains(&line_type.contains_string) {
            if line.find(&line_type.find_string) == Some(line_type.find_expected_index) {
                line_type_enum = line_type.line_type;
            }
        }
    }

    line_type_enum
}

pub fn parse_line(line: &str,  state: &mut State) {
    let line_type_enum = get_line_type(line);

    state.previous = state.current;

    match line_type_enum {
        LineType::Function => {
            state.current = ParserState::InFunction;
            state.sig_index = state.index;
            parse_function(line, state);
        },
        LineType::End => {
            state.current = ParserState::Default;
            state.sig_map.push(Range {start: state.sig_index, end: state.index + 1});
        },
        LineType::CommentStart => {
            state.current = ParserState::InComment;
            state.sig_index = state.index;
        },
        LineType::CommentEnd => {
            state.current = ParserState::LeftComment;
            state.sig_map.push(Range {start: state.sig_index, end: state.index + 1});
        },
        LineType::TypeDefinition => {
            state.current = ParserState::TypeStart;
            state.sig_index = state.index;
        },
        LineType::TableStart => {
            state.current = ParserState::InTable;
            state.sig_index = state.index;
        },
        LineType::TableInitialization => {
            state.current = ParserState::OneLineTable;
            state.sig_map.push(Range {start: state.index, end: state.index + 1});
        },
        LineType::EndBracket => {
            if state.current == ParserState::InType {
                state.current = ParserState::Default;
                state.sig_map.push(Range {start: state.sig_index, end: state.index + 1});
            }
        }
        _ => if state.current == ParserState::LeftComment || state.current == ParserState::OneLineTable {
            state.current = ParserState::Default;
        } else if state.current == ParserState::TypeStart {
            state.current = ParserState::InType;
        }
    }
}