// Generated from PropertyFile.g4 by ANTLR 4.13.2

use super::propertyfileparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by PropertyFileParser.

pub trait PropertyFileBaseListener<'input>:
    ParseTreeListener<'input, PropertyFileParserContextType> {

    /**
     * Enter a parse tree produced by \{@link PropertyFileBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_file(&mut self, _ctx: &FileContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PropertyFileBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_file(&mut self, _ctx: &FileContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PropertyFileBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_prop(&mut self, _ctx: &PropContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PropertyFileBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_prop(&mut self, _ctx: &PropContext<'input>) {}


}