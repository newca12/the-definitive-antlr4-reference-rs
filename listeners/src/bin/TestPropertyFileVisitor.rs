use antlr4rust::InputStream;
use antlr4rust::tree::{ParseTree, ParseTreeVisitorCompat, Visitable};
use antlr4rust::{common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory};
use listeners::{
    PropContext, PropContextAttrs, PropertyFileLexer, PropertyFileParser,
    PropertyFileParserContextType, PropertyFileVisitorCompat,
};
use std::{env, fs};

use std::collections::BTreeMap;

struct PropertyFileLoader {
    pub props: BTreeMap<String, String>,
    pub dummy: i64,
}

impl ParseTreeVisitorCompat<'_> for PropertyFileLoader {
    type Node = PropertyFileParserContextType;
    type Return = i64;

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + &next
    }

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.dummy
    }
}

impl PropertyFileVisitorCompat<'_> for PropertyFileLoader {
    fn visit_prop(&mut self, ctx: &PropContext<'_>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        let value = ctx.STRING().unwrap().get_text();
        self.props.insert(id, value);
        0
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
    let mut loader = PropertyFileLoader {
        props: BTreeMap::new(),
        dummy: 0,
    };
    let tree = parser.file().expect("parsed unsuccessfully");
    tree.accept(&mut loader);
    println!("{:?}", loader.props);
}
