// Generated from Expr.g4 by ANTLR 4.13.2

use super::exprparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by ExprParser.

pub trait ExprBaseListener<'input>:
    ParseTreeListener<'input, ExprParserContextType> {

    /**
     * Enter a parse tree produced by \{@link ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_s(&mut self, _ctx: &SContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_e(&mut self, _ctx: &EContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_e(&mut self, _ctx: &EContext<'input>) {}


}