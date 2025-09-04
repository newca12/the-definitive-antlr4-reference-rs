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
    fn enter_prog(&mut self, _ctx: &ProgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_prog(&mut self, _ctx: &ProgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stat(&mut self, _ctx: &StatContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stat(&mut self, _ctx: &StatContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  ExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input>) {}


}