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

    #[test]
    fn test_class_with_constructor() {
        let tokens = tokenize(r#"class Test {
            public Test() {}
        }"#);
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Bracket);
        assert_eq!(tokens[3].token_type, TokenType::Keyword);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[5].token_type, TokenType::Bracket);
        assert_eq!(tokens[6].token_type, TokenType::Bracket);
        assert_eq!(tokens[7].token_type, TokenType::Bracket);
        assert_eq!(tokens[8].token_type, TokenType::Bracket);
        assert_eq!(tokens[9].token_type, TokenType::Bracket);
    }

    #[test]
    fn test_constructor_with_parameters() {
        let tokens = tokenize(r#"class Test {
            public Test(final String str, int i) {}
        }"#);

        let expected_tokens = vec![
            Token::new(TokenType::Keyword, "class".to_string()),
            Token::new(TokenType::Identifier, "Test".to_string()),
            Token::new(TokenType::Bracket, "{".to_string()),
            Token::new(TokenType::Keyword, "public".to_string()),
            Token::new(TokenType::Identifier, "Test".to_string()),
            Token::new(TokenType::Bracket, "(".to_string()),
            Token::new(TokenType::Keyword, "final".to_string()),
            Token::new(TokenType::Identifier, "String".to_string()),
            Token::new(TokenType::Identifier, "str".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Identifier, "int".to_string()),
            Token::new(TokenType::Identifier, "i".to_string()),
            Token::new(TokenType::Bracket, ")".to_string()),
            Token::new(TokenType::Bracket, "{".to_string()),
            Token::new(TokenType::Bracket, "}".to_string()),
            Token::new(TokenType::Bracket, "}".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }
}