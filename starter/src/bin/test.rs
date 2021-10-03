use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::{tree::ParseTree, InputStream};
use starter::{ArrayInitLexer, ArrayInitParser};

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let tf = CommonTokenFactory::default();
    let lexer = ArrayInitLexer::new_with_token_factory(InputStream::new(input.as_str()), &tf);
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = ArrayInitParser::new(tokens);
    let tree = parser.init().expect("parsed unsuccessfully");
    println!("{}", tree.to_string_tree(&*parser));
}
