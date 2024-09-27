mod code {
    pub mod java {
        pub mod tokenizer;
    } 
}

use code::java::tokenizer::tokenize;

fn main() {
    tokenize("class Test{}").iter().for_each(|token| {
        println!("{:?}", token);
    });
}
