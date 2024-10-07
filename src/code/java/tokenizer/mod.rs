use std::char;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use crate::collections::push_back_iterator::PushBackIterator;

#[cfg(test)]
mod tests;

const OPERATORS: [&str; 27] = [
    "+", "/", "-", "=", "*", ">", "<", "|", "&", "++", "--", "==", "!=", "<=", ">=", "&&", "||", "+=",
    "-=", "*=", "/=", "%=", "&=", "|=", "^=", "!", "!=",
];

/// Scans a compilation unit and returns a vector of tokens
pub fn tokenize(compilation_unit: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = PushBackIterator::new(compilation_unit.chars().peekable());

    let handlers: Vec<fn(&mut PushBackIterator<Peekable<Chars>>) -> Option<Token>> = vec![
        handle_whitespace,
        handle_alpha,
        handle_number,
        handle_operator,
        handle_bracket,
        handle_comma,
        handle_asterisks,
        handle_dot,
        handle_semicolon,
        handle_unknown
    ];
    
    while iter.peek().is_some()  {
        for handler in &handlers {
            let token = handler(&mut iter);
            if token.is_some() {
                if token.as_ref().unwrap().essential {
                    tokens.push(token.unwrap());
                }
                break;
            }
        }
    }
    return tokens;
}

fn get_token_type(token: &str) -> TokenType {
    match token {
        token if is_keyword(token) => TokenType::Keyword,
        token if is_identifier(token) => TokenType::Identifier,
        token if is_operator(token) => TokenType::Operator,
        _ => panic!("Unknown token type: {}", token),
    }
}

fn handle_unknown(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    panic!("Unknown token: {}", iter.peek().unwrap());
}

fn handle_whitespace(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut handled: bool = false;
    while let Some(&ch) = iter.peek() {
        if ch.is_whitespace() {
            handled = true;
            iter.next();
        } else {
            break;
        }
    }
    if handled {
        return Some(Token::new(TokenType::Whitespace, " ".to_string(), false));
    } else {
        return None;
    }
}

fn handle_number(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    let mut has_dot = false;

    while let Some(&ch) = iter.peek() {
        if ch.is_numeric() {
            token.push(ch);
            iter.next();
        } else if ch == '.' && !has_dot {
            has_dot = true;
            token.push(ch);
            iter.next();
        } else {
            break;
        }
    }

    if !token.is_empty() && is_valid_number(&token) {
        Some(Token::with_defaults(TokenType::Number, token))
    } else {
        push_back_token(token, iter);
        None
    }
}

fn is_valid_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn handle_operator(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    while let Some(&ch) = iter.peek() {
        if OPERATORS.join("").contains(ch) {
            token.push(ch);
            iter.next();
        } else {
            break
        }
    }
    if !token.is_empty() && is_operator(&token) {
        Some(Token::with_defaults(TokenType::Operator, token))
    } else {
        None
    }
}

fn handle_alpha(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    while let Some(&ch) = iter.peek() {
        if ch.is_alphanumeric() {
            token.push(ch);
            iter.next();
        } else {
            break;
        }
    }
    if !token.is_empty() {
        if !is_valid_number(&token) {
            return Some(Token::with_defaults(get_token_type(&token), token))
        } else {
            push_back_token(token, iter);
        }
    }
    None
}

fn push_back_token(token: String, iter: &mut PushBackIterator<Peekable<Chars<'_>>>) {
    for c in token.chars().rev() {
        iter.push_back(c);
    }
}

fn handle_bracket(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    while let Some(&ch) = iter.peek() {
        if ch == '{' || ch == '}' || ch == ')' || ch == '(' {
            token.push(ch);
            iter.next();
            break;
        } else {
            break;
        }
    }
    if !token.is_empty() {
        Some(Token::with_defaults(TokenType::Bracket, token))
    } else {
        None
    }
}

fn handle_comma(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    while let Some(&ch) = iter.peek() {
        if ch == ',' {
            token.push(ch);
            iter.next();
            break;
        } else {
            break;
        }
    }
    if !token.is_empty() {
        Some(Token::with_defaults(TokenType::Comma, token))
    } else {
        None
    }
}

fn handle_asterisks(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    while let Some(&ch) = iter.peek() {
        if ch == '*' {
            token.push(ch);
            iter.next();
            break;
        } else {
            break;
        }
    }
    if !token.is_empty() {
        Some(Token::with_defaults(TokenType::Asterisks, token))
    } else {
        None
    }
}

fn handle_dot(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    while let Some(&ch) = iter.peek() {
        if ch == '.' {
            token.push(ch);
            iter.next();
        } else {
            break;
        }
    }
    if !token.is_empty() {
        Some(Token::with_defaults(TokenType::Dot, token))
    } else {
        None
    }
}

fn handle_semicolon(iter: &mut PushBackIterator<Peekable<Chars>>) -> Option<Token> {
    let mut token = String::new();
    while let Some(&ch) = iter.peek() {
        if ch == ';' {
            token.push(ch);
            iter.next();
        } else {
            break;
        }
    }
    if !token.is_empty() {
        Some(Token::with_defaults(TokenType::Semicolon, token))
    } else {
        None
    }
}

fn is_keyword(token: &str) -> bool {
    matches!(
        token,
        "class" | "public" | "static" | "void" | "final" | "import"
    )
}

fn is_operator(token: &str) -> bool {
    OPERATORS.contains(&token)
}

fn is_identifier(token: &str) -> bool {
    token.chars().all(char::is_alphanumeric)
}

/// Represents a token in a compilation unit
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub essential: bool
}

impl Token {
    pub fn new(token_type: TokenType, value: String, essential: bool) -> Token {
        Token { token_type, value, essential }
    }
    pub fn with_defaults(token_type: TokenType, value: String) -> Token {
        Token {
            token_type,
            value,
            essential: true,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token type: {:?}, value: {}",
            self.token_type, self.value
        )
    }
}

/// Represents the type of a token
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Number,
    Keyword,
    Bracket,
    Comma,
    Asterisks,
    Dot,
    Semicolon,
    Operator,
    Whitespace
}
