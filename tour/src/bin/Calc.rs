use antlr4rust::tree::{ParseTree, ParseTreeVisitorCompat, Visitable};
use antlr4rust::InputStream;
use antlr4rust::{common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory};
use serde::{Deserialize, Serialize};
use tour::{AddSubContext, AddSubContextAttrs, AssignContext, AssignContextAttrs, IdContext, IntContext, LabeledExprLexer, LabeledExprParser, LabeledExprParserContextType, LabeledExprVisitorCompat, MulDivContext, MulDivContextAttrs, PrintExprContext, PrintExprContextAttrs};
use std::collections::HashMap;
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calc {
    pub memory: HashMap<String, i64>,
    pub dummy: i64,
}

impl Default for Calc {
    fn default() -> Self {
        Calc {
            memory: HashMap::new(),
            dummy: 0,
        }
    }
}

pub struct CalcParser {
    pub(crate) calc: Calc,
}

impl ParseTreeVisitorCompat<'_> for CalcParser {
    type Node = LabeledExprParserContextType;
    type Return = i64;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.calc.dummy
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + &next
    }
}

impl LabeledExprVisitorCompat<'_> for CalcParser {
    fn visit_printExpr(&mut self, ctx: &PrintExprContext) -> Self::Return {
        let value = self.visit(&*ctx.expr().unwrap());
        println!("{}", value);
        0
    }

    fn visit_assign(&mut self, ctx: &AssignContext) -> Self::Return {
        let id = ctx.ID().unwrap();
        let value = self.visit(&*ctx.expr().unwrap());
        self.calc.memory.insert(id.get_text(), value);
        value
    }

    fn visit_MulDiv(&mut self, ctx: &MulDivContext) -> Self::Return {
        let left = self.visit(&*ctx.expr(0).unwrap());
        let right = self.visit(&*ctx.expr(1).unwrap());
        if ctx.MUL().is_some() {
            left * right
        } else {
            left / right
        }
    }

    fn visit_AddSub(&mut self, ctx: &AddSubContext) -> Self::Return {
        let left = self.visit(&*ctx.expr(0).unwrap());
        let right = self.visit(&*ctx.expr(1).unwrap());
        if ctx.ADD().is_some() {
            left + right
        } else {
            left - right
        }
    }

    fn visit_id(&mut self, ctx: &IdContext) -> Self::Return {
        let id = ctx.get_text();
        if self.calc.memory.get(&id).is_none() {
            0
        } else {
            *self.calc.memory.get(&id).unwrap()
        }
    }

    fn visit_int(&mut self, ctx: &IntContext) -> Self::Return {
        ctx.get_text().parse().unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "You need to provide an input file.");
    let input_file = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    let tf = CommonTokenFactory::default();
    let lexer = LabeledExprLexer::new_with_token_factory(InputStream::new(input_file.as_str()), &tf);
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = LabeledExprParser::new(tokens);
    let tree = parser.prog().expect("parsed unsuccessfully");
    
    let mut calc_parser = CalcParser {
        calc: Default::default(),
    };

    tree.accept(&mut calc_parser);

    calc_parser.calc;

}