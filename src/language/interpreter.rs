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

    for i in 0..value.len() {
        let character = value.chars().nth(i).unwrap();
        if character == '\\' {
            continue;
        }

        ret += 1;
    }

    ret
}

fn write(output: &mut String, string: String) {
    output.push_str(&*string);
}

pub fn generate_data_section(runtime: &Runtime) -> String {
    let mut output = String::from("section .data\n");
    for (name, tvalue) in &runtime.variables {
        let mut value = tvalue.clone();
        write(&mut output, format!("{}: db `{}`", name, value));
    }
    output
}

pub fn interpret(runtime: &mut Runtime, tokens: &mut Vec<Token>) -> Result<String, String> {
    let mut output = String::new();

    let mut in_proc = false;

    for i in 0..tokens.len() {
        match tokens[i] {
            Token::Let => match (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3]) {
                (Token::Identifier(ref name), Token::Equals, Token::Identifier(ref value)) => {
                    let mut value = value.clone();
                    value.pop();
                    value.remove(0);

                    runtime
                        .variables
                        .insert(name.to_string(), value.to_string());
                }
                _ => return Err(format!("[ERR | INTERPRET]: Invalid \"let\" syntax.",)),
            },

            Token::Register(ref name) => match (&tokens[i + 1], &tokens[i + 2]) {
                (Token::Equals, Token::Identifier(ref value)) => {
                    match runtime.variables.get(value) {
                        Some(_) => write(&mut output, format!("mov {}, {}\n", name, value)),
                        None => match x64_syscall(value) {
                            Some(value) => write(&mut output, format!("mov {}, {}\n", name, value)),
                            None => match x32_syscall(value) {
                                Some(value) => {
                                    write(&mut output, format!("mov {}, {}\n", name, value))
                                }
                                None => match get_file_descriptor(value) {
                                    Some(value) => {
                                        write(&mut output, format!("mov {}, {}\n", name, value))
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
                    write(&mut output, format!("mov {}, {}\n", name, value));
                }
                (Token::Equals, _) => {
                    write(&mut output, format!("mov {}, ", name));
                    continue;
                }
                _ => {}
            },

            Token::Length => match (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3]) {
                (Token::LParen, Token::Identifier(ref name), Token::RParen) => {
                    match runtime.variables.get(name) {
                        Some(value) => {
                            write(&mut output, format!("{}\n", get_string_length(&value)));
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

            Token::Procedure => match (&tokens[i + 1], &tokens[i + 2]) {
                (Token::Identifier(ref name), Token::LCurBrack) => {
                    in_proc = true;
                    write(&mut output, format!("{}:\n", name));
                }

                (Token::Identifier(ref name), _) => {
                    return Err(format!(
                        "[ERR | INTERPRET]: Invalid token after proc call.\n Expected proc {} {{",
                        name
                    ))
                }
                (_, Token::LCurBrack) => {
                    return Err(format!(
                        "[ERR | INTERPRET]: Invalid token after proc call.\n Expected [name] {{"
                    ))
                }
                _ => return Err(format!("[ERR | INTERPRET]: Invalid syntax for proc.")),
            },

            Token::RCurBrack => {
                if in_proc {
                    write(&mut output, format!("ret\n"));
                    in_proc = false;
                }
            }

            Token::Syscall => output.push_str("syscall\n\n"),
            _ => {}
        }
    }

    Ok(output)
}
