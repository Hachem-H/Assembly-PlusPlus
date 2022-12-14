use crate::language::Token;

use std::i32;
use std::str::FromStr;

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut char_iterator = source.chars().enumerate().peekable();
    let mut token_strings: Vec<String> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();

    let mut is_comment = false;
    let mut is_string = false;

    let mut buffer: String = String::new();
    while char_iterator.peek() != None {
        let (_, character) = char_iterator.next().unwrap();

        if is_comment {
            if character == '\n' {
                is_comment = false;
            } else {
                continue;
            }
        }

        if character.is_whitespace() {
            if !is_string {
                if !buffer.is_empty() {
                    token_strings.push(buffer.clone());
                    buffer = String::new();
                }
                continue;
            }
        }

        if is_string {
            if buffer.chars().last().unwrap() == '"' {
                is_string = false;
                token_strings.push(buffer.clone());
                buffer = String::new();
            }
        } else {
            match character {
                '(' | '[' | '{' | '}' | ']' | ')' | ',' => {
                    if !buffer.is_empty() {
                        token_strings.push(buffer.clone());
                    }
                    buffer = String::new();
                    token_strings.push(character.to_string());
                    continue;
                }

                '"' => {
                    is_string = true;
                    buffer.push('"');
                    buffer.push(char_iterator.next().unwrap().1);
                    continue;
                }

                ';' => {
                    is_comment = true;
                    continue;
                }
                _ => {}
            }
        }

        if character != '\n' {
            buffer.push(character);
        }
    }

    for token_string in token_strings {
        let number = i32::from_str(&token_string);
        if number.is_ok() {
            tokens.push(Token::Number(number.unwrap()));
            continue;
        }

        if token_string.starts_with("0x") {
            let number_string = token_string.trim_start_matches("0x");
            let number = i32::from_str_radix(number_string, 16);
            if number.is_ok() {
                tokens.push(Token::Number(number.unwrap()));
                continue;
            }
        }

        if token_string.starts_with("0b") {
            let number_string = token_string.trim_start_matches("0b");
            let number = i32::from_str_radix(number_string, 2);
            if number.is_ok() {
                tokens.push(Token::Number(number.unwrap()));
                continue;
            }
        }

        let token = Token::to_token(&token_string);
        match token {
            None => tokens.push(Token::Identifier(token_string)),
            Some(token) => tokens.push(token),
        }
    }

    tokens
}
