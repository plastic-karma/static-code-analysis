#[cfg(test)]
mod tests {
    use crate::code::java::tokenizer::{Token, TokenType, tokenize};

    #[test]
    fn test_token_new() {
        let token = Token::new(TokenType::Identifier, String::from("myToken"));
        assert_eq!(token.value, "myToken"); 
    }

    #[test]
    fn test_basic_empty_class() {
        let tokens = tokenize("class Test{}");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Bracket);
        assert_eq!(tokens[3].token_type, TokenType::Bracket);
    }
}