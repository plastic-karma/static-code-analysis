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
            Token::with_defaults(TokenType::Number, "0".to_string()),
            Token::with_defaults(TokenType::Operator, "+".to_string()),
            Token::with_defaults(TokenType::Number, "1".to_string()),
            Token::with_defaults(TokenType::Operator, "-".to_string()),
            Token::with_defaults(TokenType::Number, "2".to_string()),
            Token::with_defaults(TokenType::Operator, "*".to_string()),
            Token::with_defaults(TokenType::Number, "3".to_string()),
            Token::with_defaults(TokenType::Operator, "/".to_string()),
            Token::with_defaults(TokenType::Number, "4".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_numbers() {
        let tokens = tokenize(r#"
            int i = 1012;
            double d = 0.01;
        "#);
        let expected_tokens = vec![
            Token::with_defaults(TokenType::Identifier, "int".to_string()),
            Token::with_defaults(TokenType::Identifier, "i".to_string()),
            Token::with_defaults(TokenType::Operator, "=".to_string()),
            Token::with_defaults(TokenType::Number, "1012".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
            Token::with_defaults(TokenType::Identifier, "double".to_string()),
            Token::with_defaults(TokenType::Identifier, "d".to_string()),
            Token::with_defaults(TokenType::Operator, "=".to_string()),
            Token::with_defaults(TokenType::Number, "0.01".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_identifiers_with_digits() {
        let tokens = tokenize(r#"
            int i10 = 1012;
            double d3 = 0.01;
        "#);
        let expected_tokens = vec![
            Token::with_defaults(TokenType::Identifier, "int".to_string()),
            Token::with_defaults(TokenType::Identifier, "i10".to_string()),
            Token::with_defaults(TokenType::Operator, "=".to_string()),
            Token::with_defaults(TokenType::Number, "1012".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
            Token::with_defaults(TokenType::Identifier, "double".to_string()),
            Token::with_defaults(TokenType::Identifier, "d3".to_string()),
            Token::with_defaults(TokenType::Operator, "=".to_string()),
            Token::with_defaults(TokenType::Number, "0.01".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_function_with_if_statement() {
        let tokens = tokenize(r#"
            public boolean testFunction(final String str, int i) {
                if (i == 0) {
                    return true;
                } else {
                    return false;
                }
            }"#);
            let expected_tokens = vec![
            Token::with_defaults(TokenType::Keyword, "public".to_string()),
            Token::with_defaults(TokenType::Identifier, "boolean".to_string()),
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
            Token::with_defaults(TokenType::Keyword, "if".to_string()),
            Token::with_defaults(TokenType::Bracket, "(".to_string()),
            Token::with_defaults(TokenType::Identifier, "i".to_string()),
            Token::with_defaults(TokenType::Operator, "==".to_string()),
            Token::with_defaults(TokenType::Number, "0".to_string()),
            Token::with_defaults(TokenType::Bracket, ")".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Keyword, "return".to_string()),
            Token::with_defaults(TokenType::Identifier, "true".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
            Token::with_defaults(TokenType::Keyword, "else".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Keyword, "return".to_string()),
            Token::with_defaults(TokenType::Identifier, "false".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
        ];
        assert_tokens_equal(tokens, expected_tokens);
    }

    #[test]
    fn test_while_loop() {
        let tokens = tokenize(r#"
                while (i < 10) {
                    i++;
                }
            }"#);
        

        let expected_tokens = vec![
            Token::with_defaults(TokenType::Keyword, "while".to_string()),
            Token::with_defaults(TokenType::Bracket, "(".to_string()),
            Token::with_defaults(TokenType::Identifier, "i".to_string()),
            Token::with_defaults(TokenType::Operator, "<".to_string()),
            Token::with_defaults(TokenType::Number, "10".to_string()),
            Token::with_defaults(TokenType::Bracket, ")".to_string()),
            Token::with_defaults(TokenType::Bracket, "{".to_string()),
            Token::with_defaults(TokenType::Identifier, "i".to_string()),
            Token::with_defaults(TokenType::Operator, "++".to_string()),
            Token::with_defaults(TokenType::Semicolon, ";".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
            Token::with_defaults(TokenType::Bracket, "}".to_string()),
        ];

        assert_tokens_equal(tokens, expected_tokens);
    }

    fn assert_tokens_equal(tokens: Vec<Token>, expected_tokens: Vec<Token>) {
        assert_eq!(tokens.len(), expected_tokens.len());
        let mut i = 0;
        for (token, expected_token) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(token.token_type, expected_token.token_type, "checking token types at position {} for {} {}", i, token, expected_token);
            assert_eq!(token.value, expected_token.value);
            i += 1;
        }
    }

}