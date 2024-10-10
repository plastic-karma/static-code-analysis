use std::iter::Peekable;

use crate::code::java::tokenizer::Token;
use crate::code::java::tokenizer::TokenType::{Keyword, Identifier};

#[cfg(test)]
mod tests;

fn parse(tokens: &Vec<Token>) -> CompilationUnit {
    let mut iter = tokens.iter().peekable();
    let mut compilation_unit = CompilationUnit::new();
    while let Some(token) = iter.next() {
        if let Keyword = token.token_type {
            if token.value == "class" {
                compilation_unit.classes.push(parse_class(&mut iter));
            }
        }
    }
    compilation_unit
   
}

fn parse_class(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Class {
    if let Some(token) = iter.next() {
        if token.token_type == Identifier {
            return Class { name: token.value.clone() };
        }
    }
    panic!("Expected identifier after class keyword");
}

struct CompilationUnit {
    classes: Vec<Class>

}

struct Class {
    name: String
}

impl CompilationUnit {
    fn new() -> Self {
        CompilationUnit { classes: Vec::new() }
    }
}
