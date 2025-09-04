
// Generated from PropertyFile.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::propertyfileparser::*;

// A complete Visitor for a parse tree produced by PropertyFileParser.

pub trait PropertyFileBaseVisitor<'input>:
    ParseTreeVisitor<'input, PropertyFileParserContextType> {
	// Visit a parse tree produced by PropertyFileParser#file.
	fn visit_file(&mut self, ctx: &FileContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by PropertyFileParser#prop.
	fn visit_prop(&mut self, ctx: &PropContext<'input>) {
            self.visit_children(ctx)
        }

}