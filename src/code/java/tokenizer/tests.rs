#[cfg(test)]
mod tests {
    use crate::code::java::tokenizer::{Token, TokenType, tokenize};

    #[test]
    fn test_token_new() {
        let token = Token::with_defaults(TokenType::Identifier, String::from("myToken"));
        assert_eq!(token.value, "myToken"); 
    }

    #[test]
    fn test_basic_empty_class() {
        let tokens = tokenize("class Test{}");
        let expected_tokens = vec![
            Token::with_defaults(TokenType::Keyword, "class".to_string()),
            Token::with_defaults(TokenType::Identifier, "Test".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
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
            Token::with_defaults(TokenType::Keyword, "class".to_string()),
            Token::with_defaults(TokenType::Identifier, "Test".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Keyword, "public".to_string()),
            Token::with_defaults(TokenType::Identifier, "Test".to_string()),
            Token::with_defaults(TokenType::Bracket, "(".to_string()),
            Token::with_defaults(TokenType::Keyword, "final".to_string()),
            Token::with_defaults(TokenType::Identifier, "String".to_string()),
            Token::with_defaults(TokenType::Identifier, "str".to_string()),
            Token::with_defaults(TokenType::Comma, ",".to_string()),
            Token::with_defaults(TokenType::Identifier, "int".to_string()),
            Token::with_defaults(TokenType::Identifier, "i".to_string()),
            Token::with_defaults(TokenType::Bracket, ")".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_empty_method() {
        let tokens = tokenize(r#"class Test {
            public void testFunction(final String str, int i) {}
        }"#);

        let expected_tokens = vec![
            Token::with_defaults(TokenType::Keyword, "class".to_string()),
            Token::with_defaults(TokenType::Identifier, "Test".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Keyword, "public".to_string()),
            Token::with_defaults(TokenType::Keyword, "void".to_string()),
            Token::with_defaults(TokenType::Identifier, "testFunction".to_string()),
            Token::with_defaults(TokenType::Bracket, "(".to_string()),
            Token::with_defaults(TokenType::Keyword, "final".to_string()),
            Token::with_defaults(TokenType::Identifier, "String".to_string()),
            Token::with_defaults(TokenType::Identifier, "str".to_string()),
            Token::with_defaults(TokenType::Comma, ",".to_string()),
            Token::with_defaults(TokenType::Identifier, "int".to_string()),
            Token::with_defaults(TokenType::Identifier, "i".to_string()),
            Token::with_defaults(TokenType::Bracket, ")".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_imports() {
        let tokens = tokenize(r#"
        import java.util.List;
        import java.net.*;
        class Test{}
        "#);
        let expected_tokens = vec![
            Token::with_defaults(TokenType::Keyword, "import".to_string()),
            Token::with_defaults(TokenType::Identifier, "java".to_string()),
            Token::with_defaults(TokenType::Dot, ".".to_string()),
            Token::with_defaults(TokenType::Identifier, "util".to_string()),
            Token::with_defaults(TokenType::Dot, ".".to_string()),
            Token::with_defaults(TokenType::Identifier, "List".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
            Token::with_defaults(TokenType::Keyword, "import".to_string()),
            Token::with_defaults(TokenType::Identifier, "java".to_string()),
            Token::with_defaults(TokenType::Dot, ".".to_string()),
            Token::with_defaults(TokenType::Identifier, "net".to_string()),
            Token::with_defaults(TokenType::Dot, ".".to_string()),
            Token::with_defaults(TokenType::Operator, "*".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
            Token::with_defaults(TokenType::Keyword, "class".to_string()),
            Token::with_defaults(TokenType::Identifier, "Test".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string())
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_operators(){
        let tokens = tokenize(r#"
            int i += 0 + 1 - 2 * 3 / 4;
        "#);
        let expected_tokens = vec![
            Token::with_defaults(TokenType::Identifier, "int".to_string()),
            Token::with_defaults(TokenType::Identifier, "i".to_string()),
            Token::with_defaults(TokenType::Operator, "+=".to_string()),
            Token::with_defaults(TokenType::Identifier, "0".to_string()),
            Token::with_defaults(TokenType::Operator, "+".to_string()),
            Token::with_defaults(TokenType::Identifier, "1".to_string()),
            Token::with_defaults(TokenType::Operator, "-".to_string()),
            Token::with_defaults(TokenType::Identifier, "2".to_string()),
            Token::with_defaults(TokenType::Operator, "*".to_string()),
            Token::with_defaults(TokenType::Identifier, "3".to_string()),
            Token::with_defaults(TokenType::Operator, "/".to_string()),
            Token::with_defaults(TokenType::Identifier, "4".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }

}