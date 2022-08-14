use std::collections::HashMap;

use crate::language::Token;
use crate::tables::*;

#[allow(dead_code)]
pub struct Runtime {
    variables: HashMap<String, String>,
    reserved: HashMap<String, usize>,
    globals: Vec<String>,
    externs: Vec<String>,
}
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
            reserved: HashMap::new(),
            globals: Vec::new(),
            externs: Vec::new(),
        }
    }

    pub fn generate_data_section(&self) -> String {
        let mut output = String::new();
        for (name, value) in &self.variables {
            write(&mut output, &4, format!("{}: db `{}`\n", name, value));
        }
        output
    }

    pub fn generate_bss_section(&self) -> String {
        let mut output = String::new();
        for (name, value) in &self.reserved {
            write(&mut output, &4, format!("{}: resb {}\n", name, value));
        }
        output
    }

    pub fn generate_globals(&self) -> String {
        let mut output = String::new();
        for name in &self.globals {
            output.push_str(&*format!("global {}\n", name));
        }
        output.push('\n');
        output
    }

    pub fn generate_externs(&self) -> String {
        let mut output = String::new();
        for name in &self.externs {
            output.push_str(&*format!("extern {}\n", name));
        }
        output.push('\n');
        output
    }
}

fn write(output: &mut String, indent: &usize, string: String) {
    let output_string: String;
    if *indent == 0 {
        output_string = format!("{}", string);
    } else {
        output_string = format!(
            "{indent_char:>width$}{code}",
            indent_char = ' ',
            width = *indent,
            code = string
        );
    }
    output.push_str(&*output_string);
}

fn get_string_length(string: &String) -> i32 {
    let mut ret = 0;

    for i in 0..string.len() {
        let character = string.chars().nth(i).unwrap();
        if character == '\\' {
            continue;
        }

        ret += 1;
    }

    ret
}
struct Handler;
impl Handler {
    pub fn assignment(
        runtime: &mut Runtime,
        lh: (&String, bool),
        rh: (&String, bool),
    ) -> Result<String, String> {
        let name: String;
        let value: String;
        let mut output = String::new();

        if lh.1 || runtime.reserved.get(lh.0).is_some() {
            name = lh.0.to_string();
        } else {
            return Err(format!(
                "[ERR | PARSE]: {} = {} -> {} unknown identifier",
                lh.0, rh.0, lh.0
            ));
        }

        if rh.1 || runtime.reserved.get(rh.0).is_some() || runtime.variables.get(rh.0).is_some() {
            value = rh.0.to_string();
        } else {
            match x64_syscall(rh.0) {
                Some(val) => value = val.to_string(),
                None => match x32_syscall(rh.0) {
                    Some(val) => value = val.to_string(),
                    None => match get_file_descriptor(rh.0) {
                        Some(val) => value = val.to_string(),
                        None => {
                            return Err(format!(
                                "[ERR | PARSE]: {} = {} -> {} undefined",
                                lh.0, rh.0, rh.0
                            ))
                        }
                    },
                },
            };
        }

        output.push_str(&*format!("mov {}, {}\n", name, value));
        Ok(output)
    }

    pub fn external(index: &usize, tokens: &Vec<Token>) -> Result<Vec<String>, String> {
        let mut output: Vec<String> = Vec::new();

        for i in *index..tokens.len() {
            match tokens[i] {
                Token::Identifier(ref name) => match &tokens[i + 1] {
                    Token::Comma => output.push(name.clone()),

                    _ => {
                        return Err(format!(
                            "[ERR | PARSE]: Unknown syntax for extern, expected {}, [extern2...]",
                            name
                        ))
                    }
                },

                Token::RCurBrack => break,
                _ => {}
            }
        }

        Ok(output)
    }
}

pub fn transpile(runtime: &mut Runtime, tokens: &mut Vec<Token>) -> Result<String, String> {
    let mut output = String::new();

    let mut in_proc = false;
    let mut in_extern = false;
    let mut indent = 0usize;

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
                    continue;
                }
                (Token::Identifier(ref name), Token::Colon, Token::Number(ref value)) => {
                    runtime.reserved.insert(name.to_string(), *value as usize);
                }
                _ => return Err(format!("[ERR | PARSE]: Invalid \"let\" syntax.",)),
            },

            Token::Equals => {
                if tokens[i - 2] != Token::Let {
                    match (&tokens[i - 1], &tokens[i + 1]) {
                        (Token::Register(ref name), Token::Identifier(ref value)) => {
                            let result =
                                Handler::assignment(runtime, (&name, true), (&value, false));
                            match result {
                                Ok(result) => write(&mut output, &indent, result),
                                _ => return result,
                            }
                        }
                        (Token::Identifier(ref name), Token::Register(ref value)) => {
                            let result =
                                Handler::assignment(runtime, (&name, false), (&value, true));
                            match result {
                                Ok(result) => write(&mut output, &indent, result),
                                _ => return result,
                            }
                        }
                        (Token::Register(ref name), Token::Number(ref value)) => {
                            write(&mut output, &indent, format!("mov {}, {}\n", name, value))
                        }
                        (Token::Register(ref name), Token::Register(ref value)) => {
                            write(&mut output, &indent, format!("mov {}, {}\n", name, value))
                        }
                        (Token::Register(ref name), _) => {
                            write(&mut output, &indent, format!("mov {}, ", name))
                        }
                        (Token::Identifier(ref name), _) => match runtime.reserved.get(name) {
                            Some(_) => write(&mut output, &indent, format!("mov {}, ", name)),
                            None => {
                                return Err(format!(
                                    "[ERR | PARSE]: {} = [] -> {} undefined",
                                    name, name
                                ))
                            }
                        },
                        _ => {}
                    }
                } else if tokens[i - 2] == Token::Star {
                    // TODO(Hachem): Pointers
                }
            }

            Token::Length => match (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3]) {
                (Token::LParen, Token::Identifier(ref name), Token::RParen) => {
                    match runtime.variables.get(name) {
                        Some(value) => {
                            output.push_str(&*format!("{}\n", get_string_length(&value)))
                        }
                        None => match runtime.reserved.get(name) {
                            Some(value) => output.push_str(&*format!("{}\n", value)),
                            None => {
                                return Err(format!(
                                    "[ERR | PARSE]: len({}) -> {} undefined",
                                    name, name
                                ))
                            }
                        },
                    }
                }
                _ => return Err(format!("[ERR | PARSE]: Invalid \"len\" syntax.",)),
            },

            Token::Procedure => match (&tokens[i + 1], &tokens[i + 2]) {
                (Token::Identifier(ref name), Token::LCurBrack) => {
                    if i > 0 {
                        if tokens[i - 1] == Token::Global {
                            runtime.globals.push(name.clone());
                        }
                    }

                    write(&mut output, &indent, format!("{}:\n", name));
                    in_proc = true;
                    indent += 4;
                }

                (Token::Identifier(ref name), _) => {
                    return Err(format!(
                        "[ERR | PARSE]: Invalid token after proc call.\n Expected proc {} {{",
                        name
                    ))
                }
                (_, Token::LCurBrack) => {
                    return Err(format!(
                        "[ERR | PARSE]: Invalid token after proc call.\n Expected [name] {{"
                    ))
                }
                _ => return Err(format!("[ERR | PARSE]: Invalid syntax for proc.")),
            },

            Token::Extern => match (&tokens[i + 1], &tokens[i + 2]) {
                (Token::LCurBrack, _) => {
                    let result = Handler::external(&i, &tokens);
                    match result {
                        Ok(result) => {
                            in_extern = true;
                            indent += 4;
                            for name in result {
                                runtime.externs.push(name);
                            }
                        }
                        _ => return Err(result.unwrap_err()),
                    }
                }

                (Token::Identifier(ref name), _) => runtime.externs.push(name.clone()),

                _ => {
                    return Err(format!(
                        "[ERR | PARSE]: Unknown syntax for extern, expected: extern [name]"
                    ));
                }
            },

            Token::RCurBrack => {
                if in_proc {
                    write(&mut output, &indent, format!("ret\n\n"));
                    in_proc = false;
                }

                if in_extern {
                    in_extern = false;
                }

                indent -= 4;
            }

            Token::Syscall => write(&mut output, &indent, format!("syscall\n\n")),
            _ => {}
        }
    }

    Ok(output)
}
