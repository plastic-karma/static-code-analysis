#[cfg(test)]
mod tests {
    use crate::code::java::parser::parse;
    use crate::code::java::tokenizer::tokenize;


    #[test]
    fn test_parse_class() {
        let tokens = tokenize(r#"
            import java.util.List;
            class Test {
                int i = 3;
            }"#);
        let compilation_unit = parse(&tokens);

        assert_eq!(compilation_unit.classes.len(), 1);
        assert!(compilation_unit.classes.iter().any(|class| class.name == "Test"));
    }

    #[test]
    #[should_panic(expected = "Expected identifier after class keyword")]
    fn test_parse_bad_class() {
        let tokens = tokenize(r#"
            import java.util.List;
            class return {
                int i = 3;
            }"#);
        let compilation_unit = parse(&tokens);
    }

}