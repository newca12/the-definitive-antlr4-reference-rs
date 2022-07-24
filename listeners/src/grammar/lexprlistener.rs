#![allow(nonstandard_style)]
// Generated from LExpr.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::lexprparser::*;

pub trait LExprListener<'input> : ParseTreeListener<'input,LExprParserContextType>{
/**
 * Enter a parse tree produced by {@link LExprParser#s}.
 * @param ctx the parse tree
 */
fn enter_s(&mut self, _ctx: &SContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LExprParser#s}.
 * @param ctx the parse tree
 */
fn exit_s(&mut self, _ctx: &SContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Add}
 * labeled alternative in {@link LExprParser#e}.
 * @param ctx the parse tree
 */
fn enter_Add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Add}
 * labeled alternative in {@link LExprParser#e}.
 * @param ctx the parse tree
 */
fn exit_Add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Mult}
 * labeled alternative in {@link LExprParser#e}.
 * @param ctx the parse tree
 */
fn enter_Mult(&mut self, _ctx: &MultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Mult}
 * labeled alternative in {@link LExprParser#e}.
 * @param ctx the parse tree
 */
fn exit_Mult(&mut self, _ctx: &MultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Int}
 * labeled alternative in {@link LExprParser#e}.
 * @param ctx the parse tree
 */
fn enter_Int(&mut self, _ctx: &IntContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Int}
 * labeled alternative in {@link LExprParser#e}.
 * @param ctx the parse tree
 */
fn exit_Int(&mut self, _ctx: &IntContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : LExprListener<'input> }


