use std::{env, fs};

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::{tree::ParseTree, InputStream};
use tour::{ExprLexer, ExprParser};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "You need to provide an input file.");
    let input_file = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    let tf = CommonTokenFactory::default();
    let lexer = ExprLexer::new_with_token_factory(InputStream::new(input_file.as_str()), &tf);
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = ExprParser::new(tokens);
    let tree = parser.prog().expect("parsed unsuccessfully");
    println!("{}", tree.to_string_tree(&*parser));
}
