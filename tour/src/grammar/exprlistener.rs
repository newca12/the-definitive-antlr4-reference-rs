#![allow(nonstandard_style)]
// Generated from Expr.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::exprparser::*;

pub trait ExprListener<'input> : ParseTreeListener<'input,ExprParserContextType>{
/**
 * Enter a parse tree produced by {@link ExprParser#prog}.
 * @param ctx the parse tree
 */
fn enter_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ExprParser#prog}.
 * @param ctx the parse tree
 */
fn exit_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ExprParser#stat}.
 * @param ctx the parse tree
 */
fn enter_stat(&mut self, _ctx: &StatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ExprParser#stat}.
 * @param ctx the parse tree
 */
fn exit_stat(&mut self, _ctx: &StatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ExprParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ExprParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : ExprListener<'input> }


