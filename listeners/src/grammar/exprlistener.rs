#![allow(nonstandard_style)]
// Generated from Expr.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::exprparser::*;

pub trait ExprListener<'input> : ParseTreeListener<'input,ExprParserContextType>{
/**
 * Enter a parse tree produced by {@link ExprParser#s}.
 * @param ctx the parse tree
 */
fn enter_s(&mut self, _ctx: &SContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ExprParser#s}.
 * @param ctx the parse tree
 */
fn exit_s(&mut self, _ctx: &SContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ExprParser#e}.
 * @param ctx the parse tree
 */
fn enter_e(&mut self, _ctx: &EContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ExprParser#e}.
 * @param ctx the parse tree
 */
fn exit_e(&mut self, _ctx: &EContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : ExprListener<'input> }


