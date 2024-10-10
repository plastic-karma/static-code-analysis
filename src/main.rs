mod code {
    pub mod java {
        pub mod tokenizer;
        pub mod parser;
    }
}


pub mod collections {
    pub mod push_back_iterator;
}

use code::java::tokenizer::tokenize;

fn main() {
    tokenize("class Test{}").iter().for_each(|token| {
        println!("{:?}", token);
    });
}
