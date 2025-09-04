#![allow(nonstandard_style)]
// Generated from PropertyFile.g4 by ANTLR 4.13.2
use antlr4rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::propertyfileparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link PropertyFileParser}.
 */
pub trait PropertyFileVisitor<'input>: ParseTreeVisitor<'input,PropertyFileParserContextType>{
	/**
	 * Visit a parse tree produced by {@link PropertyFileParser#file}.
	 * @param ctx the parse tree
	 */
	fn visit_file(&mut self, ctx: &FileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PropertyFileParser#prop}.
	 * @param ctx the parse tree
	 */
	fn visit_prop(&mut self, ctx: &PropContext<'input>) { self.visit_children(ctx) }

}

pub trait PropertyFileVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= PropertyFileParserContextType>{
	/**
	 * Visit a parse tree produced by {@link PropertyFileParser#file}.
	 * @param ctx the parse tree
	 */
		fn visit_file(&mut self, ctx: &FileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PropertyFileParser#prop}.
	 * @param ctx the parse tree
	 */
		fn visit_prop(&mut self, ctx: &PropContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> PropertyFileVisitor<'input> for T
where
	T: PropertyFileVisitorCompat<'input>
{
	fn visit_file(&mut self, ctx: &FileContext<'input>){
		let result = <Self as PropertyFileVisitorCompat>::visit_file(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_prop(&mut self, ctx: &PropContext<'input>){
		let result = <Self as PropertyFileVisitorCompat>::visit_prop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}