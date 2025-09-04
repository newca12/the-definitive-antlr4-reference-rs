// Generated from ArrayInit.g4 by ANTLR 4.13.2

use super::arrayinitparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by ArrayInitParser.

pub trait ArrayInitBaseListener<'input>:
    ParseTreeListener<'input, ArrayInitParserContextType> {

    /**
     * Enter a parse tree produced by \{@link ArrayInitBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_init(&mut self, _ctx: &InitContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  ArrayInitBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_init(&mut self, _ctx: &InitContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link ArrayInitBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_value(&mut self, _ctx: &ValueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  ArrayInitBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_value(&mut self, _ctx: &ValueContext<'input>) {}


}