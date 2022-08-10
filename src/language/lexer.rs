use crate::language::token::Token;

use itertools::Itertools;

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut char_iterator = source.chars().enumerate().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while char_iterator.peek() != None {
        let (_, character) = char_iterator.next().unwrap();

        match character {
            character if character.is_whitespace() || character == '\n' => continue,

            '"' => {
                let string_chars: Vec<char> = char_iterator
                    .by_ref()
                    .take_while(|&(_, x)| x != '"')
                    .map(|(_, x)| x)
                    .collect();
                let text: String = string_chars.into_iter().collect();
                tokens.push(Token::Str(text));
            }

            '-' => {
                if !tokens.is_empty() && tokens.last().unwrap().is_value() {
                    tokens.push(Token::Minus);
                } else {
                    tokens.push(Token::UnaryMinus);
                }
            }

            '+' => tokens.push(Token::Plus),
            '/' => tokens.push(Token::Divide),
            '*' => tokens.push(Token::Multiply),

            '!' => tokens.push(Token::Bang),
            ',' => tokens.push(Token::Comma),

            '{' => tokens.push(Token::LCurBrack),
            '}' => tokens.push(Token::RCurBrack),

            '[' => tokens.push(Token::LBrack),
            ']' => tokens.push(Token::RBrack),

            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),

            _ => {
                let mut token_chars: Vec<char> = char_iterator
                    .by_ref()
                    .peeking_take_while(|&(_, x)| !(x.is_whitespace()))
                    .map(|(_, x)| x)
                    .collect();
                token_chars.insert(0, character);
                let mut token_string: String = token_chars.into_iter().collect();

                match token_string.chars().last().unwrap() {
                    ')' => {
                        tokens.push(Token::RParen);
                        token_string.pop();
                    }

                    ']' => {
                        tokens.push(Token::RBrack);
                        token_string.pop();
                    }

                    '}' => {
                        tokens.push(Token::RCurBrack);
                        token_string.pop();
                    }
                    _ => {}
                }

                let token = Token::to_token(token_string.as_str());
                match token {
                    None => tokens.push(Token::Identifier(token_string.to_string())),
                    Some(token) => tokens.push(token),
                }
            }
        }
    }

    tokens
}
