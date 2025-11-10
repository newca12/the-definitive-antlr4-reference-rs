use std::{env, fs};

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::token_factory::CommonTokenFactory;
use antlr4rust::tree::ParseTreeListener;
use antlr4rust::{InputStream, tree::ParseTree};
use listeners::{
    PropContext, PropContextAttrs, PropertyFileLexer, PropertyFileListener, PropertyFileParser,
    PropertyFileParserContextType, PropertyFileTreeWalker,
};
use std::collections::BTreeMap;

struct PropertyFileLoader {
    pub props: BTreeMap<String, String>,
}

impl<'input> ParseTreeListener<'input, PropertyFileParserContextType> for PropertyFileLoader {}

impl<'input> PropertyFileListener<'input> for PropertyFileLoader {
    fn exit_prop(&mut self, ctx: &PropContext<'input>) {
        let id = ctx.ID().unwrap().get_text();
        let value = ctx.STRING().unwrap().get_text();
        self.props.insert(id, value);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "You need to provide an input file.");
    let input_file = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let tf = CommonTokenFactory::default();
    let lexer =
        PropertyFileLexer::new_with_token_factory(InputStream::new(input_file.as_str()), &tf);
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = PropertyFileParser::new(tokens);
    let listener = Box::new(PropertyFileLoader {
        props: BTreeMap::new(),
    });
    let tree = parser.file().expect("parsed unsuccessfully");
    let listener = PropertyFileTreeWalker::walk(listener, &*tree);
    println!("{:?}", listener.unwrap().props);
}
