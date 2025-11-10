use std::cell::RefCell;
use std::rc::Rc;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::errors::ANTLRError;
use antlr4rust::parser::ParserNodeType;
use antlr4rust::parser_rule_context::BaseParserRuleContext;
use antlr4rust::rule_context::CustomRuleContext;
use antlr4rust::token_factory::CommonTokenFactory;
use antlr4rust::tree::{ParseTreeListener, ParseTreeWalker};
use antlr4rust::{InputStream, tree::ParseTree};
use starter::{
    ArrayInitLexer, ArrayInitListener, ArrayInitParser, ArrayInitParserContextType, InitContext,
    InitContextExt, ValueContext, ValueContextAttrs,
};

struct ShortToUnicodeStringListener {}

impl<'input> ParseTreeListener<'input, ArrayInitParserContextType>
    for ShortToUnicodeStringListener
{
}

impl<'input> ArrayInitListener<'input> for ShortToUnicodeStringListener {
    fn enter_init(&mut self, _ctx: &InitContext) {
        print!("'");
    }

    fn exit_init(&mut self, _ctx: &InitContext) {
        print!("'");
    }

    //fn enter_value(&mut self, ctx: &ValueContext<'input>) {
    //    print!("ctx1 {:?}", ctx.INT());
    //}

    //the value is only available on exit. Maybe an antlr4rust bug.
    fn exit_value(&mut self, ctx: &ValueContext<'input>) {
        let value: u32 = ctx.INT().unwrap().get_text().parse().unwrap();
        print!("\\u{:#04x}", value);
    }
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let tf = CommonTokenFactory::default();
    let lexer = ArrayInitLexer::new_with_token_factory(InputStream::new(input.as_str()), &tf);
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = ArrayInitParser::new(tokens);
    parser.add_parse_listener(Box::new(ShortToUnicodeStringListener {}));
    let tree = parser.init().expect("parsed unsuccessfully");
    //Use a trick to walk the tree, legacy walker require an annotation for node and syntax is yet to be found.
    //let t: &Result<Rc<BaseParserRuleContext<InitContextExt>>, ANTLRError> = &tree;
    //let l: Box<ShortToUnicodeStringListener> = Box::new(ShortToUnicodeStringListener {});
    //let walker = ParseTreeWalker::walk::<_,_>(l, t);
    tree.to_string_tree(&*parser);
}
