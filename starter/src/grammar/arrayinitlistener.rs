#![allow(nonstandard_style)]
// Generated from ArrayInit.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::arrayinitparser::*;

pub trait ArrayInitListener<'input> : ParseTreeListener<'input,ArrayInitParserContextType>{
/**
 * Enter a parse tree produced by {@link ArrayInitParser#init}.
 * @param ctx the parse tree
 */
fn enter_init(&mut self, _ctx: &InitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ArrayInitParser#init}.
 * @param ctx the parse tree
 */
fn exit_init(&mut self, _ctx: &InitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ArrayInitParser#value}.
 * @param ctx the parse tree
 */
fn enter_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ArrayInitParser#value}.
 * @param ctx the parse tree
 */
fn exit_value(&mut self, _ctx: &ValueContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : ArrayInitListener<'input> }


