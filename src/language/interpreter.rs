use std::collections::HashMap;

use crate::language::Token;

#[allow(dead_code)]
pub struct Runtime {
    variables: HashMap<String, String>,
    reserved: HashMap<String, usize>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
            reserved: HashMap::new(),
        }
    }
}

fn get_string_length(string: &String) -> i32 {
    let mut ret = 0;
    let mut value = string.clone();
    
    value.remove(0);
    value.pop();

    for i in 0..value.len() {
        let character = value.chars().nth(i).unwrap();
        if character == '\\' {
            continue;
        } 

        ret += 1;
    }

    ret
}

pub fn interpret(runtime: &mut Runtime, tokens: &Vec<Token>) -> Result<String, String> {
    let mut token_iter = tokens.iter().peekable();

    while token_iter.peek() != None {
        let ref token = *token_iter.next().unwrap();

        match *token {
            Token::Let => match (token_iter.next(), token_iter.next(), token_iter.next()) {
                (
                    Some(&Token::Identifier(ref name)),
                    Some(&Token::Equals),
                    Some(&Token::Identifier(ref value)),
                ) => {
                    runtime
                        .variables
                        .insert(name.to_string(), value.to_string());
                }
                _ => return Err(format!("[ERR | INTERPRET]: Invalid \"let\" syntax.",)),
            },

            Token::Register(ref name) => {
                match (token_iter.next(), token_iter.next()) {
                    (Some(&Token::Equals),
                    Some(&Token::Identifier(ref value)),)
                    => {
                        println!("mov {}, {}", name, value);
                    }
                    (Some(&Token::Equals), Some(_)) => {
                        println!("mov {}, ", name);
                    }
                    _ => {}
                }
            }

            Token::Length => match (token_iter.next(), token_iter.next(), token_iter.next()) {
                (
                    Some(&Token::LParen),
                    Some(&Token::Identifier(ref name)),
                    Some(&Token::RParen),
                ) => match runtime.variables.get(name) {
                    Some(value) => {
                        print!("{}", get_string_length(&value));
                    }
                    None => match runtime.variables.get(name) {
                        Some(value) => {
                            print!("{}", get_string_length(&value));
                        }
                        None => {}
                    },
                },
                _ => return Err(format!("[ERR | INTERPRET]: Invalid \"len\" syntax.",)),
            },

            _ => {}
        }
    }

    Ok(String::new())
}
