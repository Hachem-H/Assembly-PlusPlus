#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Procedure,
    Length,

    Syscall,
    Interrupt,

    Number(i32),
    Str(String),

    Comment,
    Identifier(String),

    RCurBrack,
    LCurBrack,
    RParen,
    LParen,
    RBrack,
    LBrack,

    Plus,
    Minus,
    Divide,
    Multiply,

    Equals,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    Comma,
    Bang,
    UnaryMinus,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[allow(dead_code)]
impl Token {
    pub fn to_token(symbol: &str) -> Option<Token> {
        match symbol {
            "proc" => Some(Token::Procedure),
            "len" => Some(Token::Length),

            "syscall" => Some(Token::Syscall),
            "int" => Some(Token::Interrupt),

            "+" => Some(Token::Plus),
            "-" => Some(Token::Minus),
            "/" => Some(Token::Divide),
            "*" => Some(Token::Multiply),

            "=" => Some(Token::Equals),
            "<>" => Some(Token::NotEqual),
            "<" => Some(Token::LessThan),
            ">" => Some(Token::GreaterThan),
            "<=" => Some(Token::LessThanEqual),
            ">=" => Some(Token::GreaterThanEqual),

            "{" => Some(Token::LCurBrack),
            "}" => Some(Token::RCurBrack),
            "[" => Some(Token::LBrack),
            "]" => Some(Token::RBrack),
            "(" => Some(Token::LParen),
            ")" => Some(Token::RParen),

            "," => Some(Token::Comma),
            "!" => Some(Token::Bang),

            _ => None,
        }
    }

    pub fn is_operator(&self) -> bool {
        match *self {
            Token::Equals
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan
            | Token::LessThanEqual
            | Token::GreaterThanEqual
            | Token::Plus
            | Token::Minus
            | Token::Divide
            | Token::Multiply
            | Token::Bang
            | Token::UnaryMinus => true,
            _ => false,
        }
    }

    pub fn is_comparison_operator(&self) -> bool {
        match *self {
            Token::Equals
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan
            | Token::LessThanEqual
            | Token::GreaterThanEqual => true,
            _ => false,
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        match *self {
            Token::Bang | Token::UnaryMinus => true,
            _ => false,
        }
    }

    pub fn is_binary_operator(&self) -> bool {
        self.is_operator() && !self.is_unary_operator()
    }

    pub fn is_value(&self) -> bool {
        match *self {
            Token::Identifier(_) | Token::Number(_) | Token::Str(_) => true,
            _ => false,
        }
    }

    pub fn operator_precedence(&self) -> Result<u8, String> {
        if !self.is_operator() {
            return Err("ERR: Not an operator!".to_string());
        }

        match *self {
            Token::UnaryMinus | Token::Bang => Ok(1 << 5),
            Token::Multiply | Token::Divide => Ok(1 << 4),
            Token::Minus | Token::Plus => Ok(1 << 3),
            _ => Ok(1 << 2),
        }
    }

    pub fn operator_associativity(&self) -> Result<Associativity, String> {
        match *self {
            Token::UnaryMinus | Token::Bang => Ok(Associativity::Right),
            _ => Ok(Associativity::Left),
        }
    }
}
