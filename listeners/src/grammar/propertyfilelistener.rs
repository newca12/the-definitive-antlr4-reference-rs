#![allow(nonstandard_style)]
// Generated from PropertyFile.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::propertyfileparser::*;

pub trait PropertyFileListener<'input> : ParseTreeListener<'input,PropertyFileParserContextType>{
/**
 * Enter a parse tree produced by {@link PropertyFileParser#file}.
 * @param ctx the parse tree
 */
fn enter_file(&mut self, _ctx: &FileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PropertyFileParser#file}.
 * @param ctx the parse tree
 */
fn exit_file(&mut self, _ctx: &FileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PropertyFileParser#prop}.
 * @param ctx the parse tree
 */
fn enter_prop(&mut self, _ctx: &PropContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PropertyFileParser#prop}.
 * @param ctx the parse tree
 */
fn exit_prop(&mut self, _ctx: &PropContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : PropertyFileListener<'input> }


