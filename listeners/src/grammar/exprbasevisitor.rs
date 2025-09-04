
// Generated from Expr.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::exprparser::*;

// A complete Visitor for a parse tree produced by ExprParser.

pub trait ExprBaseVisitor<'input>:
    ParseTreeVisitor<'input, ExprParserContextType> {
	// Visit a parse tree produced by ExprParser#s.
	fn visit_s(&mut self, ctx: &SContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by ExprParser#e.
	fn visit_e(&mut self, ctx: &EContext<'input>) {
            self.visit_children(ctx)
        }

}