use std::collections::HashMap;

use crate::language::Token;
use crate::tables::*;

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

pub fn interpret(runtime: &mut Runtime, tokens: &mut Vec<Token>) -> Result<String, String> {
    let mut output = String::new();

    for i in 0..tokens.len() {
        match tokens[i] {
            Token::Let => match (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3]) {
                (Token::Identifier(ref name), Token::Equals, Token::Identifier(ref value)) => {
                    runtime
                        .variables
                        .insert(name.to_string(), value.to_string());
                }
                _ => return Err(format!("[ERR | INTERPRET]: Invalid \"let\" syntax.",)),
            },

            Token::Register(ref name) => match (&tokens[i + 1], &tokens[i + 2]) {
                (Token::Equals, Token::Identifier(ref value)) => {
                    match runtime.variables.get(value) {
                        Some(value) => {
                            output.push_str(&*format!("mov {}, {}\n", name, value));
                        }
                        None => match x64_syscall(value) {
                            Some(value) => {
                                output.push_str(&*format!("mov {}, {}\n", name, value));
                            }
                            None => match x64_syscall(value) {
                                Some(value) => {
                                    output.push_str(&*format!("mov {}, {}\n", name, value));
                                }
                                None => match get_file_descriptor(value) {
                                    Some(value) => {
                                        output.push_str(&*format!("mov {}, {}\n", name, value));
                                    }
                                    None => {
                                        return Err(format!(
                                            "[ERR | INTERPRET]: {} = {} -> {} undefined",
                                            name, value, value
                                        ))
                                    }
                                },
                            },
                        },
                    }
                }
                (Token::Equals, Token::Number(ref value)) => {
                    output.push_str(&*format!("mov {}, {}\n", name, value));
                }
                (Token::Equals, _) => {
                    output.push_str(&*format!("mov {}, ", name));
                    continue;
                }
                _ => {}
            },

            Token::Length => match (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3]) {
                (Token::LParen, Token::Identifier(ref name), Token::RParen) => {
                    match runtime.variables.get(name) {
                        Some(value) => {
                            output.push_str(&*format!("{}\n", get_string_length(&value)));
                        }
                        None => {
                            return Err(format!(
                                "[ERR | INTERPRET]: len({}) -> {} undefined",
                                name, name
                            ))
                        }
                    }
                }
                _ => return Err(format!("[ERR | INTERPRET]: Invalid \"len\" syntax.",)),
            },
            _ => {}
        }
    }

    Ok(output)
}
