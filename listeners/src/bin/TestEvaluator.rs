use std::any::Any;
use std::borrow::Cow;

use antlr4rust::rule_context::RuleContext;
use antlr4rust::token::Token;
use antlr4rust::tree::{ParseTree, ParseTreeListener, ParseTreeVisitor, Tree, Visitable};
use antlr4rust::InputStream;
use antlr4rust::{common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory};

use listeners::{EContext, ExprLexer, ExprListener, ExprParser, ExprParserContextType, ExprTreeWalker, ExprVisitor, SContext, exprparser};

pub struct Evaluator {
    pub stack: Vec<i64>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Evaluator {
            stack: Vec::new(),
        }
    }
}

pub struct EvaluatorParser {
    pub(crate) evaluator: Evaluator,
}


impl<'i> ParseTreeVisitor<'i, ExprParserContextType> for EvaluatorParser {
    fn visit_terminal(&mut self, node: &antlr4rust::tree::TerminalNode<'i, ExprParserContextType>) {
        println!("visit");
        if node.symbol.get_token_type() == exprparser::Expr_INT {
            println!("push"); 
            self.evaluator.stack.push(node.get_text().parse::<i64>().unwrap());
            //if let Cow::Borrowed(s) = node.symbol.text {
            //    self.0.push(s);
            //}
        }
        //let symbol = node.get_text();
        //println!("node {:?}", symbol);
    }

}

impl<'i> ExprVisitor<'i> for EvaluatorParser {
    
	fn visit_s(&mut self, ctx: &SContext<'i>) { self.visit_children(ctx) }

	fn visit_e(&mut self, ctx: &EContext<'i>) { self.visit_children(ctx) }
}


impl<'input> ParseTreeListener<'input, ExprParserContextType> for EvaluatorParser {}

impl<'input> ExprListener<'input> for EvaluatorParser {

    fn exit_e(&mut self, ctx: &EContext<'input>) { 
        if ctx.get_child_count() == 3  { 
          //let left = self.evaluator.stack.pop().unwrap();
         //let right = self.evaluator.stack.pop().unwrap();
        }
        
        println!("all {:?}", ctx.op);
        println!("exit_e");
    }

    fn exit_s(&mut self, _ctx: &SContext<'input>) { }

}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let tf = CommonTokenFactory::default();
    let lexer = ExprLexer::new_with_token_factory(InputStream::new(input.as_str()), &tf);
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = ExprParser::new(tokens);
    let mut evaluator_parser = EvaluatorParser {
        evaluator: Default::default(),
    };
    parser.add_parse_listener(Box::new(EvaluatorParser { evaluator: Default::default()}));
    let tree = parser.s().expect("parsed unsuccessfully");
    tree.accept(&mut evaluator_parser);
    //let listener = ExprTreeWalker::walk(Box::new(evaluator_parser), &*tree);
}