#![allow(nonstandard_style)]
// Generated from Expr.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::exprparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ExprParser}.
 */
pub trait ExprVisitor<'input>: ParseTreeVisitor<'input,ExprParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ExprParser#s}.
	 * @param ctx the parse tree
	 */
	fn visit_s(&mut self, ctx: &SContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ExprParser#e}.
	 * @param ctx the parse tree
	 */
	fn visit_e(&mut self, ctx: &EContext<'input>) { self.visit_children(ctx) }

}

pub trait ExprVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= ExprParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ExprParser#s}.
	 * @param ctx the parse tree
	 */
		fn visit_s(&mut self, ctx: &SContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ExprParser#e}.
	 * @param ctx the parse tree
	 */
		fn visit_e(&mut self, ctx: &EContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> ExprVisitor<'input> for T
where
	T: ExprVisitorCompat<'input>
{
	fn visit_s(&mut self, ctx: &SContext<'input>){
		let result = <Self as ExprVisitorCompat>::visit_s(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_e(&mut self, ctx: &EContext<'input>){
		let result = <Self as ExprVisitorCompat>::visit_e(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}