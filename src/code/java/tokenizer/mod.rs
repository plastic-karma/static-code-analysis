use std::fmt;
use std::char;

#[cfg(test)]
mod tests;

/// Scans a compilation unit and returns a vector of tokens
pub fn tokenize(compilation_unit: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();
    for character in compilation_unit.chars() {
        match character {
            
            // Handle whitespace
            _ if character.is_whitespace() => {
                if !current_token.is_empty() {
                    tokens.push(Token::new(
                        get_token_type(&current_token),
                        current_token.clone(),
                    ));
                    current_token.clear();
                }
            }

            // Handle alphanumeric characters
            _ if char::is_alphanumeric(character) => {
                current_token.push(character);
            }

            // Handle brackets
            _ if character == '{' || character == '}' || character == ')' || character == '(' => {
                if !current_token.is_empty() {
                    tokens.push(Token::new(
                        get_token_type(&current_token),
                        current_token.clone(),
                    ));
                    current_token.clear();
                }
                tokens.push(Token::new(TokenType::Bracket, character.to_string()));
            }
            _ => {}
        }
    }
    return tokens;
}

fn get_token_type(token: &str) -> TokenType {
    if is_keyword(token) {
        TokenType::Keyword
    } else if is_identifier(token) {
        TokenType::Identifier
    } else {
        panic!("Unknown token type: {}", token);
    }
}

fn is_keyword(token: &str) -> bool {
    matches!(token, "class" | "public" | "static" | "void")
}

fn is_identifier(token: &str) -> bool {
    token.chars().all(char::is_alphanumeric)
}

/// Represents a token in a compilation unit
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token { token_type, value }
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
    Keyword,
    Bracket,
}
