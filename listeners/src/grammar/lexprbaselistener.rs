// Generated from LExpr.g4 by ANTLR 4.13.2

use super::lexprparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by LExprParser.

pub trait LExprBaseListener<'input>:
    ParseTreeListener<'input, LExprParserContextType> {

    /**
     * Enter a parse tree produced by \{@link LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_s(&mut self, _ctx: &SContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_add(&mut self, _ctx: &AddContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add(&mut self, _ctx: &AddContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mult(&mut self, _ctx: &MultContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mult(&mut self, _ctx: &MultContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_int(&mut self, _ctx: &IntContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_int(&mut self, _ctx: &IntContext<'input>) {}


}